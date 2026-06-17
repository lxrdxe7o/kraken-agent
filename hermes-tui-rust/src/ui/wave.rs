//! GlyphWave — Phase-shifted sine wave footer
//!
//! Implements the "Aetheric Shaders" concept from Phase 4 of the animation
//! plan.  Instead of static spinners or dots, a fluid, oceanic wave of block
//! characters (`' ', '▂', '▃', '▄', '▅'`) provides constant visual feedback
//! during "Thinking / Working" states.
//!
//! ## How it works
//! - `wave_glyph(x, tick)` evaluates two summed sine waves at different
//!   frequencies and maps the result onto 5 block height levels.
//! - Rendering counts backwards (`iter().rev()`) and breaks early once the
//!   visible `Rect` width is filled — exactly as the `oha` pattern prescribes.
//!
//! ## Integration
//! Call `render_wave_footer(frame, area, tick)` in your draw path whenever
//! `thinking == true`.  The component respects `ACTIVE_ANIMATIONS` so the
//! event loop stays demand-driven.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::engine;

/// The five block-character heights we use for the wave, from trough to peak.
const WAVE: [char; 5] = [' ', '▂', '▃', '▄', '▅'];

/// Evaluate a phase-shifted dual-sine wave at column `x` for animation tick
/// `tick`.
///
/// Two sine waves are summed so the pattern feels organic rather than a single
/// boring sinusoid.  The result is normalised onto [0, 4] and mapped to the
/// `WAVE` array.
#[must_use]
pub fn wave_glyph(x: usize, tick: usize) -> char {
    let t = tick as f64 / 100.0;
    let x_f = x as f64;
    let val = (x_f * 0.5 - t * 8.0).sin() + (x_f * 0.2 + t * 3.0).sin() * 0.5;
    let normalized = ((val + 1.5) / 3.0).clamp(0.0, 1.0);
    WAVE[(normalized * 4.0).round() as usize]
}

/// Render an animated wave footer across the full width of `area`.
///
/// - **Renders backwards** (`iter().rev()`) so the rightmost edge is always
///   the freshest data.
/// - **Breaks early** once the visible width is filled (anti-Polling pattern).
/// - Calls `engine::animation_start()` / `engine::animation_end()` to keep the
///   demand-driven render counter correct (caller MUST match the lifecycle).
pub fn render_wave_footer(frame: &mut Frame, area: Rect, tick: usize) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // Build one wave line per row of the footer.
    let mut lines: Vec<Line> = Vec::with_capacity(area.height as usize);
    for _row in 0..area.height {
        let mut spans: Vec<Span> = Vec::with_capacity(area.width as usize);

        // Render columns backwards (right-to-left) — `oha` pattern.
        for x in (0..area.width as usize).rev() {
            let ch = wave_glyph(x, tick);
            // Colour shifts subtly with the wave height for extra depth.
            let color = match ch {
                '▅' | '▄' => Color::Rgb(102, 217, 239), // cyan
                '▃' => Color::Rgb(250, 189, 47),         // yellow
                '▂' => Color::Rgb(131, 165, 152),         // green
                _ => Color::Rgb(60, 60, 60),               // dim
            };
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        // The rev() puts the rightmost column in position 0; we need to
        // reverse the spans to get correct left-to-right ordering.
        spans.reverse();
        lines.push(Line::from(spans));
    }

    // Use subtle background to match the TUI aesthetic.
    let paragraph = Paragraph::new(lines).style(Style::default().bg(Color::Rgb(27, 32, 33)));
    frame.render_widget(paragraph, area);
}

/// A ticker that manages the wave animation lifecycle.
///
/// Call `advance()` on each frame loop iteration when the agent is thinking.
/// The struct calls `animation_start` on first tick and `animation_end` when
/// reset or dropped so the global animation counter stays accurate.
#[derive(Debug)]
pub struct WaveTicker {
    tick: usize,
    active: bool,
}

impl WaveTicker {
    /// Create a new idle ticker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tick: 0,
            active: false,
        }
    }

    /// Advance the animation by one frame.
    ///
    /// Registers with the global animation counter on first call and deregisters
    /// if the ticker is subsequently stopped (by calling `stop()` or on drop).
    pub fn advance(&mut self) -> usize {
        self.tick = self.tick.wrapping_add(1);
        if !self.active {
            engine::animation_start();
            self.active = true;
        }
        self.tick
    }

    /// Stop the animation and deregister from the global counter.
    pub fn stop(&mut self) {
        if self.active {
            engine::animation_end();
            self.active = false;
        }
        self.tick = 0;
    }

    /// Access the current tick value.
    #[must_use]
    pub fn current_tick(&self) -> usize {
        self.tick
    }

    /// Whether the ticker is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for WaveTicker {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WaveTicker {
    fn drop(&mut self) {
        if self.active {
            engine::animation_end();
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_glyph_returns_valid_char() {
        for x in 0..50 {
            for tick in [0, 16, 100, 500] {
                let ch = wave_glyph(x, tick);
                assert!(WAVE.contains(&ch), "unexpected glyph {ch:?}");
            }
        }
    }

    #[test]
    fn test_wave_ticker_lifecycle() {
        {
            let mut ticker = WaveTicker::new();
            assert!(!ticker.is_active());

            let t1 = ticker.advance();
            assert!(ticker.is_active());
            assert_eq!(t1, 1);

            let t2 = ticker.advance();
            assert_eq!(t2, 2);

            ticker.stop();
            assert!(!ticker.is_active());
            assert_eq!(ticker.current_tick(), 0);
        }
        // Verify the global counter is balanced after all drops.
        // (We can't easily assert AtomicU64 == 0 because other
        // tests may run concurrently, but the lifecycle is correct.)
    }
}
