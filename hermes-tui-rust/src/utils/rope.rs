//! Rope-based text buffer (Phase 3.1)
//!
//! Wraps the `ropey::Rope` for O(log n) insertions, deletions, and slicing
//! in the embedded modal editor and large chat-log display.
//!
//! ## Render Culling
//!
//! `RopeBuffer::visible_lines(start_row, height)` returns only the subset of
//! lines visible within the terminal's physical `Rect`, avoiding full-buffer
//! traversal on every frame.

use ropey::{LineType, Rope};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn strip_trailing_line_break(s: &str) -> String {
    let trimmed = s.trim_end_matches(|c| c == '\n' || c == '\r');
    trimmed.to_string()
}

// ---------------------------------------------------------------------------
// RopeBuffer
// ---------------------------------------------------------------------------

/// A Rope-backed text buffer with O(log n) mutation and rendering culling.
///
/// All edit operations use **byte indices** (matching ropey 2.0 API). The
/// line type for line-oriented queries is always [`LineType::LF_CR`].
///
/// # Render Culling
///
/// Accessor methods (`visible_lines`, `line_slice`) only traverse the subset
/// of the Rope that falls within the viewport, never the entire document.
#[derive(Debug, Clone)]
pub struct RopeBuffer {
    rope: Rope,
    /// Cached styled spans per line. `None` = dirty (needs re-highlight).
    span_cache: Vec<Option<Vec<(usize, String)>>>,
}

impl Default for RopeBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl RopeBuffer {
    /// Create a new empty rope buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            span_cache: Vec::new(),
        }
    }

    /// Create a rope buffer from an existing string.
    #[must_use]
    pub fn from_str(text: &str) -> Self {
        let rope = Rope::from_str(text);
        let n = rope.len_lines(LineType::LF_CR);
        Self {
            rope,
            span_cache: vec![None; n],
        }
    }

    // ==================================================================
    // Rope operations (O(log n) — byte indices)
    // ==================================================================

    /// Insert text at a **byte** offset.
    pub fn insert(&mut self, byte_idx: usize, text: &str) {
        let len = self.rope.len();
        let idx = byte_idx.min(len);
        self.rope.insert(idx, text);
        self._invalidate_from_byte(idx);
    }

    /// Remove `len` bytes starting at `byte_idx`.
    pub fn remove(&mut self, byte_idx: usize, len: usize) {
        let total = self.rope.len();
        if byte_idx >= total {
            return;
        }
        let end = (byte_idx + len).min(total);
        self.rope.remove(byte_idx..end);
        self._invalidate_from_byte(byte_idx);
    }

    /// Replace a byte range with new text (one operation).
    pub fn replace(&mut self, byte_idx: usize, len: usize, text: &str) {
        let total = self.rope.len();
        if byte_idx >= total {
            self.rope.insert(total, text);
        } else {
            let end = (byte_idx + len).min(total);
            self.rope.remove(byte_idx..end);
            self.rope.insert(byte_idx, text);
        }
        self._invalidate_from_byte(byte_idx);
    }

    /// Append text at the end.
    pub fn push_str(&mut self, text: &str) {
        let pos = self.rope.len();
        self.rope.insert(pos, text);
        self._invalidate_from_byte(pos);
    }

    /// Total byte count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rope.len()
    }

    /// Total line count.
    #[must_use]
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines(LineType::LF_CR)
    }

    /// Total char count.
    #[must_use]
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rope.len() == 0
    }

    /// Get the full text as a string.
    #[must_use]
    pub fn to_string(&self) -> String {
        self.rope.chunks().collect::<String>()
    }

    /// Get a specific line (0-indexed) as a string, with trailing line
    /// terminators stripped.
    #[must_use]
    pub fn line(&self, idx: usize) -> String {
        if idx >= self.rope.len_lines(LineType::LF_CR) {
            return String::new();
        }
        let s: String = self.rope.line(idx, LineType::LF_CR).chunks().collect();
        strip_trailing_line_break(&s)
    }

    // ==================================================================
    // Char <-> byte index convenience
    // ==================================================================

    /// Convert a char index to a byte index.
    #[must_use]
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.rope
            .char_to_byte_idx(char_idx.min(self.rope.len_chars()))
    }

    /// Convert a byte index to a char index.
    #[must_use]
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        self.rope
            .byte_to_char_idx(byte_idx.min(self.rope.len()))
    }

    // ==================================================================
    // Render Culling
    // ==================================================================

    /// Return the lines visible within the viewport.
    ///
    /// `start_row` is the first visible line (0-indexed), `height` is the
    /// number of terminal rows. Lines beyond the buffer are returned as empty.
    /// Trailing line-terminator characters are stripped.
    #[must_use]
    pub fn visible_lines(&self, start_row: usize, height: usize) -> Vec<String> {
        let max = self.rope.len_lines(LineType::LF_CR);
        (start_row..start_row + height)
            .map(|i| {
                if i < max {
                    let s: String =
                        self.rope.line(i, LineType::LF_CR).chunks().collect();
                    strip_trailing_line_break(&s)
                } else {
                    String::new()
                }
            })
            .collect()
    }

    /// Return a visible slice of a line, respecting `max_width`.
    #[must_use]
    pub fn visible_line_slice(&self, line: usize, col: usize, max_width: usize) -> String {
        if line >= self.rope.len_lines(LineType::LF_CR) {
            return String::new();
        }
        let line_str = self.rope.line(line, LineType::LF_CR);
        let line_s: String = line_str.chunks().collect();
        let total_width = line_s.len();
        if total_width <= max_width {
            return strip_trailing_line_break(&line_s);
        }
        let half = max_width.saturating_sub(1) / 2;
        let start_col = if col > half { col - half } else { 0 };
        strip_trailing_line_break(&line_s.chars().skip(start_col).take(max_width).collect::<String>())
    }

    // ==================================================================
    // Span cache invalidation
    // ==================================================================

    /// Mark a specific line as dirty (needs re-highlight).
    pub fn invalidate_line(&mut self, line: usize) {
        if line < self.span_cache.len() {
            self.span_cache[line] = None;
        }
    }

    /// Mark all lines as dirty.
    pub fn invalidate_all(&mut self) {
        for slot in &mut self.span_cache {
            *slot = None;
        }
    }

    /// Check whether a line's cache is valid.
    #[must_use]
    pub fn is_line_cached(&self, line: usize) -> bool {
        self.span_cache.get(line).is_some_and(|s| s.is_some())
    }

    // ==================================================================
    // Cursor helpers
    // ==================================================================

    /// Convert (line, col) to a byte index.
    #[must_use]
    pub fn line_col_to_byte(&self, line: usize, col: usize) -> usize {
        if line >= self.rope.len_lines(LineType::LF_CR) {
            return self.rope.len();
        }
        let line_start = self.rope.line_to_byte_idx(line, LineType::LF_CR);
        let line_str = self.rope.line(line, LineType::LF_CR);
        let line_s: String = line_str.chunks().collect();
        let mut byte_offset = 0;
        let mut width = 0;
        for ch in line_s.chars() {
            if width >= col {
                break;
            }
            width += 1; // column is in char units for convenience
            byte_offset += ch.len_utf8();
        }
        line_start + byte_offset
    }

    /// Convert a byte index to (line, column).
    #[must_use]
    pub fn byte_to_line_col(&self, byte_idx: usize) -> (usize, usize) {
        let line = self
            .rope
            .byte_to_line_idx(byte_idx.min(self.rope.len()), LineType::LF_CR);
        let line_start = self.rope.line_to_byte_idx(line, LineType::LF_CR);
        let col = byte_idx - line_start;
        (line, col)
    }

    /// Convert (line, col) to a char index.
    #[must_use]
    pub fn line_col_to_char(&self, line: usize, col: usize) -> usize {
        let byte_idx = self.line_col_to_byte(line, col);
        self.byte_to_char(byte_idx)
    }

    // ==================================================================
    // Internal
    // ==================================================================

    fn _invalidate_from_byte(&mut self, byte_idx: usize) {
        let line = self
            .rope
            .byte_to_line_idx(byte_idx.min(self.rope.len()), LineType::LF_CR);
        let new_line_count = self.rope.len_lines(LineType::LF_CR);
        self.span_cache.resize(new_line_count, None);
        for i in line..new_line_count {
            self.span_cache[i] = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buf = RopeBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.len_chars(), 0);
        assert_eq!(buf.len_lines(), 1); // empty rope still has 1 empty line
    }

    #[test]
    fn test_from_str() {
        let buf = RopeBuffer::from_str("hello\nworld");
        assert_eq!(buf.len_lines(), 2);
        assert_eq!(buf.line(0), "hello");
        assert_eq!(buf.line(1), "world");
    }

    #[test]
    fn test_insert_byte() {
        let mut buf = RopeBuffer::from_str("helo");
        buf.insert(3, "l");
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_remove() {
        let mut buf = RopeBuffer::from_str("hello world");
        buf.remove(5, 6);
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_replace() {
        let mut buf = RopeBuffer::from_str("hello xorld");
        buf.replace(6, 1, "w");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_push_str() {
        let mut buf = RopeBuffer::from_str("hello");
        buf.push_str(" world");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_visible_lines() {
        let buf = RopeBuffer::from_str("a\nb\nc\nd\ne");
        let visible = buf.visible_lines(1, 3);
        assert_eq!(visible.len(), 3);
        assert_eq!(visible[0], "b");
        assert_eq!(visible[1], "c");
        assert_eq!(visible[2], "d");
    }

    #[test]
    fn test_visible_lines_beyond_buffer() {
        let buf = RopeBuffer::from_str("a\nb");
        let visible = buf.visible_lines(0, 5);
        assert_eq!(visible.len(), 5);
        assert_eq!(visible[0], "a");
        assert_eq!(visible[1], "b");
        assert_eq!(visible[2], "");
        assert_eq!(visible[3], "");
        assert_eq!(visible[4], "");
    }

    #[test]
    fn test_cursor_conversion() {
        let buf = RopeBuffer::from_str("hello\nworld");
        let (line, col) = buf.byte_to_line_col(6);
        assert_eq!(line, 1);
        assert_eq!(col, 0);

        let byte_idx = buf.line_col_to_byte(1, 3);
        assert_eq!(byte_idx, 9);
        assert_eq!(buf.to_string().as_bytes()[9] as char, 'l');
    }

    #[test]
    fn test_char_byte_conversion() {
        let buf = RopeBuffer::from_str("héllo wörld");
        // 'é' is 2 bytes in UTF-8 (0xC3 0xA9), 'ö' is 2 bytes
        assert_eq!(buf.char_to_byte(0), 0);   // 'h'
        assert_eq!(buf.char_to_byte(1), 1);   // 'é' starts at byte 1
        // byte_to_char rounds down inside a multi-byte char
        assert_eq!(buf.byte_to_char(2), 1);   // byte 2 = inside 'é' → char 1
        assert_eq!(buf.byte_to_char(1), 1);   // byte 1 = start of 'é' → char 1
    }

    #[test]
    fn test_line_col_to_char() {
        let buf = RopeBuffer::from_str("héllo");
        assert_eq!(buf.line_col_to_char(0, 0), 0);
        assert_eq!(buf.line_col_to_char(0, 1), 1); // 'h'
        assert_eq!(buf.line_col_to_char(0, 2), 2); // 'é' (1 char)
        assert_eq!(buf.line_col_to_char(0, 3), 3); // first 'l'
    }

    #[test]
    fn test_visible_line_slice() {
        let buf = RopeBuffer::from_str("hello world");
        let slice = buf.visible_line_slice(0, 0, 5);
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_line_strips_trailing_break() {
        let buf = RopeBuffer::from_str("a\nb\n");
        assert_eq!(buf.line(0), "a");
        assert_eq!(buf.line(1), "b");
    }

    #[test]
    fn test_rope_with_crlf() {
        let buf = RopeBuffer::from_str("a\r\nb");
        assert_eq!(buf.len_lines(), 2);
        assert_eq!(buf.line(0), "a");
        assert_eq!(buf.line(1), "b");
    }
}
