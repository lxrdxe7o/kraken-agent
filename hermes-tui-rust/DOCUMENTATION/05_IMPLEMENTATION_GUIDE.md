# Hermes TUI Rust - Implementation Guide

**Project**: Rust-based Terminal User Interface for Hermes Agent  
**Status**: Phase 2 Implementation in Progress  
**Last Updated**: 2026-06-14  
**Document Version**: 2.0  

---

## 🎯 GETTING STARTED

### Prerequisites

Before you begin, ensure you have:

1. **Rust installed** (version 1.75+ recommended)
   ```bash
   rustc --version
   cargo --version
   ```

2. **Python environment** (for Hermes core)
   ```bash
   python3 --version
   pip --version
   ```

3. **Project cloned and checked out**
   ```bash
   cd /home/lxrdxe7o/.hermes/hermes-agent
   git status
   ```

4. **Dependencies installed**
   ```bash
   # For Hermes core
   source .venv/bin/activate  # or venv/bin/activate
   
   # For Rust TUI
   cd hermes-tui-rust
   cargo build
   ```

---

## 🚀 QUICK START (5-10 minutes)

### 1. Verify Current State

```bash
# Navigate to the project
cd /home/lxrdxe7o/.hermes/hermes-agent/hermes-tui-rust

# Check the build
cargo build --release

# Run all tests
cargo test

# Check for warnings
cargo clippy

# Check formatting
cargo fmt --check
```

All tests should pass (111/111) ✅

### 2. Read Essential Documentation

Read these files in order:

1. `DOCUMENTATION/01_EXECUTIVE_SUMMARY.md` (10-15 min)
2. `DOCUMENTATION/02_CODEBASE_STRUCTURE.md` (20-30 min)
3. `PHASE2_STATUS.md` (10 min)
4. `PHASE2_ORCHESTRATION.md` (15 min)

### 3. Understand the Current Blockers

From the executive summary, the **critical blockers** are:

1. **Handlers not implemented** - No keyboard/mouse input
2. **CLI integration missing** - Cannot launch from Hermes
3. **Gateway integration not tested** - May not work with actual gateway

### 4. Start with the First Task

**Recommended First Task**: Implement the Handlers Layer

The handlers are stubs and are blocking all other work. They are isolated and can be implemented without affecting other parts of the codebase.

---

## 📋 IMPLEMENTATION CHECKLIST

### Priority 1: CRITICAL (Blocking)

- [ ] **Implement Handlers**
  - [ ] `src/handlers/keys.rs` - Key event handling
  - [ ] `src/handlers/mouse.rs` - Mouse event handling
  - [ ] `src/handlers/input.rs` - Text input handling
  - [ ] Add tests for handlers
  - [ ] Verify handlers work with app

- [ ] **Add CLI Integration**
  - [ ] Add `--tui-rust` flag to `hermes_cli/main.py`
  - [ ] Create `hermes_cli/launch_tui_rust.py`
  - [ ] Pass configuration to Rust TUI
  - [ ] Handle spawn and communication

- [ ] **Test Gateway Integration**
  - [ ] Run Rust TUI with actual tui_gateway
  - [ ] Verify all message types work
  - [ ] Fix any protocol mismatches
  - [ ] Test connection error handling

### Priority 2: HIGH (After Critical)

- [ ] **Implement Config Loading**
  - [ ] Load from `~/.hermes/config.yaml`
  - [ ] Merge with defaults
  - [ ] Handle profile-specific config
  - [ ] Load themes from files

- [ ] **Implement oh-my-pi Features**
  - [ ] `src/ui/hashline.rs` - Hashline edit system
  - [ ] Enhance `src/ui/cards.rs` - Wire to tool execution
  - [ ] `src/ui/subagents.rs` - Subagent visualization
  - [ ] Integrate with delegation system

- [ ] **Complete Integration**
  - [ ] Auto-reconnect logic
  - [ ] Approval flow
  - [ ] Completion flow
  - [ ] Session lifecycle

### Priority 3: MEDIUM (Nice to Have)

- [ ] **Implement Utilities**
  - [ ] `src/utils/text.rs` - Text processing
  - [ ] `src/utils/ansi.rs` - ANSI handling

- [ ] **Add Testing**
  - [ ] Integration tests
  - [ ] E2E tests
  - [ ] Performance benchmarks

- [ ] **Add LSP/Debugger UI**
  - [ ] `src/ui/lsp.rs`
  - [ ] `src/ui/debugger.rs`

### Priority 4: LOW (Future)

- [ ] **Performance Optimization**
- [ ] **Additional Themes**
- [ ] **Custom Keybindings**
- [ ] **Localization**
- [ ] **Accessibility Features**

---

## 🛠️ DETAILED IMPLEMENTATION STEPS

### Task 1: Implement Handlers Layer

#### Overview
The handlers layer is responsible for processing user input events (keyboard, mouse, text) and converting them into Actions that the app can understand.

**Files to Implement**:
- `src/handlers/keys.rs`
- `src/handlers/mouse.rs`
- `src/handlers/input.rs`

**Reference**: See `ARCHITECTURE.md` section on Handlers

---

#### Step 1.1: Implement Key Handler (`src/handlers/keys.rs`)

**Purpose**: Handle keyboard events and convert to Actions

**Required Components**:

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use crate::state::config::{Keybindings, InputMode};

/// Action enum - defines all possible actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // Composer actions
    ComposerSubmit,
    ComposerCancel,
    ComposerNewline,
    ComposerDelete,
    ComposerBackspace,
    ComposerLeft,
    ComposerRight,
    ComposerUp,
    ComposerDown,
    ComposerHome,
    ComposerEnd,
    ComposerInsert,
    
    // Navigation actions
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    GoToTop,
    GoToBottom,
    
    // Session actions
    NewSession,
    PreviousSession,
    NextSession,
    ListSessions,
    
    // Command actions
    OpenCommandPalette,
    
    // System actions
    Quit,
    Suspend,
    ToggleTheme,
    ToggleHelp,
    
    // Mode switching
    EnterNormalMode,
    EnterInsertMode,
    EnterCommandMode,
    
    // Other
    Noop,
}

/// KeyHandler struct
pub struct KeyHandler {
    /// Keybindings configuration
    keybindings: Keybindings,
    /// Current input mode
    current_mode: InputMode,
}

impl KeyHandler {
    /// Create new KeyHandler with default keybindings
    pub fn new(keybindings: Keybindings) -> Self {
        Self {
            keybindings,
            current_mode: InputMode::Normal,
        }
    }
    
    /// Set current input mode
    pub fn set_mode(&mut self, mode: InputMode) {
        self.current_mode = mode;
    }
    
    /// Handle a key event and return the corresponding action
    pub fn handle(&self, key: KeyEvent) -> Action {
        // Get bindings for current mode
        let bindings = match self.current_mode {
            InputMode::Normal => &self.keybindings.normal,
            InputMode::Insert => &self.keybindings.insert,
            InputMode::Command => &self.keybindings.command,
        };
        
        // Look up the action for this key
        bindings.get(&key).copied().unwrap_or(Action::Noop)
    }
    
    /// Get the keybindings for a specific mode
    pub fn get_bindings(&self, mode: InputMode) -> &HashMap<KeyEvent, Action> {
        match mode {
            InputMode::Normal => &self.keybindings.normal,
            InputMode::Insert => &self.keybindings.insert,
            InputMode::Command => &self.keybindings.command,
        }
    }
}

/// Default keybindings
impl Default for Keybindings {
    fn default() -> Self {
        use Action::*;
        
        let mut normal = HashMap::new();
        // Normal mode bindings
        normal.insert(KeyEvent::Char('i'), EnterInsertMode);
        normal.insert(KeyEvent::Char('q'), Quit);
        normal.insert(KeyEvent::Char('k'), ScrollUp);
        normal.insert(KeyEvent::Char('j'), ScrollDown);
        normal.insert(KeyEvent::Char('h'), ComposerLeft);
        normal.insert(KeyEvent::Char('l'), ComposerRight);
        normal.insert(KeyEvent::Char('g'), GoToTop);
        normal.insert(KeyEvent::Char('G'), GoToBottom);
        normal.insert(KeyEvent::Char('n'), NewSession);
        normal.insert(KeyEvent::Char('p'), PreviousSession);
        normal.insert(KeyEvent::Char('N'), NextSession);
        normal.insert(KeyEvent::Char(':'), OpenCommandPalette);
        
        let mut insert = HashMap::new();
        // Insert mode bindings
        insert.insert(KeyEvent::Char('\n'), ComposerNewline);
        insert.insert(KeyEvent::Backspace, ComposerBackspace);
        insert.insert(KeyEvent::Delete, ComposerDelete);
        insert.insert(KeyEvent::Left, ComposerLeft);
        insert.insert(KeyEvent::Right, ComposerRight);
        insert.insert(KeyEvent::Up, ComposerUp);
        insert.insert(KeyEvent::Down, ComposerDown);
        insert.insert(KeyEvent::Home, ComposerHome);
        insert.insert(KeyEvent::End, ComposerEnd);
        insert.insert(KeyEvent::Esc, EnterNormalMode);
        
        let mut command = HashMap::new();
        // Command mode bindings
        command.insert(KeyEvent::Esc, ComposerCancel);
        command.insert(KeyEvent::Backspace, ComposerBackspace);
        
        Self {
            normal,
            insert,
            command,
        }
    }
}
```

**Implementation Steps**:

1. Define the `Action` enum with all possible actions
2. Create `KeyHandler` struct with keybindings and current mode
3. Implement `new()` method
4. Implement `set_mode()` method
5. Implement `handle()` method that:
   - Gets bindings for current mode
   - Looks up the key event
   - Returns the corresponding Action
6. Implement `get_bindings()` method
7. Implement `Default` for `Keybindings` with sensible defaults

**Testing**:
- Test key lookups in each mode
- Test mode switching
- Test default keybindings

---

#### Step 1.2: Implement Mouse Handler (`src/handlers/mouse.rs`)

**Purpose**: Handle mouse events (clicks, scroll, drag)

```rust
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use crate::handlers::keys::Action;

/// MouseHandler struct
pub struct MouseHandler;

impl MouseHandler {
    /// Create new MouseHandler
    pub fn new() -> Self {
        Self
    }
    
    /// Handle a mouse event within a specific area
    pub fn handle(&self, mouse: MouseEvent, area: Rect) -> Option<Action> {
        match mouse.kind {
            MouseEventKind::Down(button) => {
                // Handle mouse down (click)
                self.handle_click(mouse.column, mouse.row, button, area)
            }
            MouseEventKind::Up(button) => {
                // Handle mouse up (release)
                self.handle_release(mouse.column, mouse.row, button, area)
            }
            MouseEventKind::Drag => {
                // Handle drag
                None
            }
            MouseEventKind::ScrollUp => {
                // Handle scroll up
                Some(Action::ScrollUp)
            }
            MouseEventKind::ScrollDown => {
                // Handle scroll down
                Some(Action::ScrollDown)
            }
            _ => None,
        }
    }
    
    /// Handle click event
    fn handle_click(&self, col: u16, row: u16, button: crossterm::event::MouseButton, area: Rect) -> Option<Action> {
        // Check if click is within the area
        if col >= area.left() && col < area.right() && row >= area.top() && row < area.bottom() {
            // Convert click position to relative position
            let rel_col = col - area.left();
            let rel_row = row - area.top();
            
            // For now, just return focus action
            // This will be enhanced to handle specific widget clicks
            Some(Action::ComposerInsert) // Focus composer on click
        } else {
            None
        }
    }
    
    /// Handle release event
    fn handle_release(&self, col: u16, row: u16, button: crossterm::event::MouseButton, area: Rect) -> Option<Action> {
        // For now, no action on release
        None
    }
}
```

**Implementation Steps**:

1. Create `MouseHandler` struct
2. Implement `new()` method
3. Implement `handle()` method that:
   - Matches on mouse event kind
   - Delegates to specific handlers
   - Returns Option<Action>
4. Implement click handler
5. Implement scroll handler

**Testing**:
- Test scroll events
- Test click events within area
- Test click events outside area

---

#### Step 1.3: Implement Input Handler (`src/handlers/input.rs`)

**Purpose**: Handle text input processing with completions

```rust
use crate::ui::composer::InputComposer;
use crate::handlers::keys::Action;
use crossterm::event::KeyEvent;

/// InputHandler struct
pub struct InputHandler {
    composer: InputComposer,
    /// Completion state
    completions: Vec<String>,
    completion_index: Option<usize>,
}

impl InputHandler {
    /// Create new InputHandler
    pub fn new() -> Self {
        Self {
            composer: InputComposer::new(),
            completions: Vec::new(),
            completion_index: None,
        }
    }
    
    /// Handle a character input
    pub fn handle_char(&mut self, c: char) -> Action {
        self.composer.handle_char(c);
        Action::Noop
    }
    
    /// Handle a key event
    pub fn handle_key(&mut self, key: KeyEvent) -> Action {
        // Check if we're in completion mode
        if let Some(index) = self.completion_index {
            // Handle completion navigation
            return self.handle_completion_key(key, index);
        }
        
        // Handle regular key events
        match key.code {
            KeyCode::Enter => Action::ComposerSubmit,
            KeyCode::Esc => Action::ComposerCancel,
            KeyCode::Tab => {
                // Request completions
                self.request_completions();
                Action::Noop
            }
            KeyCode::Up => {
                if self.composer.history_up() {
                    Action::Noop
                } else {
                    Action::ScrollUp
                }
            }
            KeyCode::Down => {
                if self.composer.history_down() {
                    Action::Noop
                } else {
                    Action::ScrollDown
                }
            }
            _ => {
                // Let composer handle it
                self.composer.handle_key(key);
                Action::Noop
            }
        }
    }
    
    /// Handle key event during completion
    fn handle_completion_key(&mut self, key: KeyEvent, current_index: usize) -> Action {
        match key.code {
            KeyCode::Up => {
                self.completion_index = Some(current_index.saturating_sub(1));
                self.update_composer_with_completion();
                Action::Noop
            }
            KeyCode::Down => {
                self.completion_index = Some(std::cmp::min(current_index + 1, self.completions.len() - 1));
                self.update_composer_with_completion();
                Action::Noop
            }
            KeyCode::Enter => {
                if let Some(index) = self.completion_index {
                    self.apply_completion(index);
                    self.completion_index = None;
                }
                Action::ComposerSubmit
            }
            KeyCode::Esc => {
                self.completion_index = None;
                Action::Noop
            }
            KeyCode::Tab => {
                // Cycle through completions
                self.completion_index = Some((current_index + 1) % self.completions.len());
                self.update_composer_with_completion();
                Action::Noop
            }
            _ => {
                // Hide completions on other keys
                self.completion_index = None;
                Action::Noop
            }
        }
    }
    
    /// Request completions from gateway
    fn request_completions(&mut self) {
        let text = self.composer.get_text();
        // TODO: Send completion request to gateway
        // For now, just set some dummy completions
        self.completions = vec![
            "/help".to_string(),
            "/quit".to_string(),
            "/new".to_string(),
        ];
        self.completion_index = Some(0);
        self.update_composer_with_completion();
    }
    
    /// Update composer with current completion
    fn update_composer_with_completion(&mut self) {
        if let Some(index) = self.completion_index {
            if index < self.completions.len() {
                let text = self.completions[index].clone();
                self.composer.set_text(&text);
            }
        }
    }
    
    /// Apply a completion
    fn apply_completion(&mut self, index: usize) {
        if index < self.completions.len() {
            let text = self.completions[index].clone();
            self.composer.set_text(&text);
        }
        self.completion_index = None;
    }
    
    /// Get current text
    pub fn get_text(&self) -> &str {
        self.composer.get_text()
    }
    
    /// Clear input
    pub fn clear(&mut self) {
        self.composer.clear();
        self.completion_index = None;
    }
    
    /// Submit input
    pub fn submit(&mut self) -> String {
        let text = self.composer.submit();
        self.completion_index = None;
        text
    }
    
    /// Get composer reference
    pub fn composer(&self) -> &InputComposer {
        &self.composer
    }
    
    /// Get mutable composer reference
    pub fn composer_mut(&mut self) -> &mut InputComposer {
        &mut self.composer
    }
}
```

**Implementation Steps**:

1. Create `InputHandler` struct with composer and completion state
2. Implement `new()` method
3. Implement `handle_char()` method
4. Implement `handle_key()` method
5. Implement completion handling:
   - Request completions
   - Navigate completions
   - Apply completion
6. Implement utility methods (get_text, clear, submit)

**Testing**:
- Test character input
- Test key handling
- Test completion flow
- Test submission

---

#### Step 1.4: Update Module Exports (`src/handlers/mod.rs`)

**Current Content**:
```rust
// Handler module exports
pub mod keys;
pub mod mouse;
pub mod input;
```

**Update to**:
```rust
//! Handlers module - User input handling
//!
//! This module provides handlers for keyboard, mouse, and text input events.

pub mod input;
pub mod keys;
pub mod mouse;

// Re-export main types
pub use input::InputHandler;
pub use keys::{Action, KeyHandler, Keybindings};
pub use mouse::MouseHandler;
```

---

#### Step 1.5: Add Tests for Handlers

Create inline tests in each file:

**In `keys.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_keybindings() {
        let bindings = Keybindings::default();
        assert!(bindings.normal.contains_key(&KeyEvent::Char('i')));
        assert!(bindings.insert.contains_key(&KeyEvent::Esc));
    }
    
    #[test]
    fn test_key_handler_handle() {
        let handler = KeyHandler::new(Keybindings::default());
        let action = handler.handle(KeyEvent::Char('i'));
        assert_eq!(action, Action::EnterInsertMode);
    }
}
```

**In `mouse.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{MouseButton, MouseEventKind};
    
    #[test]
    fn test_scroll_up() {
        let handler = MouseHandler::new();
        let mouse = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: crossterm::event::KeyModifiers::NONE,
        };
        let area = Rect::new(0, 0, 10, 10);
        let action = handler.handle(mouse, area);
        assert_eq!(action, Some(Action::ScrollUp));
    }
}
```

**In `input.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};
    
    #[test]
    fn test_char_input() {
        let mut handler = InputHandler::new();
        handler.handle_char('a');
        assert_eq!(handler.get_text(), "a");
    }
    
    #[test]
    fn test_submit() {
        let mut handler = InputHandler::new();
        handler.handle_char('a');
        handler.handle_char('b');
        let text = handler.submit();
        assert_eq!(text, "ab");
        assert!(handler.get_text().is_empty());
    }
}
```

---

#### Step 1.6: Verify Handlers Work

```bash
# Build
cargo build

# Run tests
cargo test

# All tests should pass (111 + new handler tests)
```

---

### Task 2: Add CLI Integration

#### Overview
Add the `--tui-rust` flag to Hermes CLI to allow launching the Rust TUI.

**File to Modify**: `hermes_cli/main.py`

---

#### Step 2.1: Add CLI Flag

Find the argument parser setup in `hermes_cli/main.py`:

```python
# Add the --tui-rust flag
parser.add_argument(
    '--tui-rust',
    action='store_true',
    help='Use the Rust-based TUI (experimental)'
)
```

---

#### Step 2.2: Create Launch Function

Create a new file `hermes_cli/launch_tui_rust.py`:

```python
"""Launch the Rust-based TUI for Hermes Agent."""

import subprocess
import sys
import os
from pathlib import Path


def launch_tui_rust():
    """Launch the Rust TUI binary."""
    # Find the Rust TUI binary
    hermes_home = os.getenv('HERMES_HOME', os.path.expanduser('~/.hermes'))
    
    # Try different locations for the binary
    possible_paths = [
        Path(hermes_home) / 'hermes-tui-rust' / 'target' / 'release' / 'hermes-tui-rust',
        Path(hermes_home) / 'bin' / 'hermes-tui-rust',
        Path.cwd() / 'hermes-tui-rust' / 'target' / 'release' / 'hermes-tui-rust',
        Path('/usr/local/bin/hermes-tui-rust'),
    ]
    
    binary_path = None
    for path in possible_paths:
        if path.exists():
            binary_path = path
            break
    
    if not binary_path:
        print("Error: Rust TUI binary not found.")
        print("Please build it first: cd hermes-tui-rust && cargo build --release")
        return False
    
    # Run the binary
    try:
        # Run in the current directory with stdio connected to gateway
        process = subprocess.Popen(
            [str(binary_path)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=os.getcwd()
        )
        
        # The binary will communicate with tui_gateway via stdio
        # We need to connect it properly
        print(f"Launching Rust TUI from: {binary_path}")
        print("Press Ctrl+C to exit...")
        
        process.wait()
        return process.returncode == 0
        
    except Exception as e:
        print(f"Error launching Rust TUI: {e}")
        return False
```

---

#### Step 2.3: Integrate into Main CLI

In `hermes_cli/main.py`, find where the TUI is launched and add:

```python
# In the _launch_tui function or wherever TUI is launched
def launch_tui(args):
    if getattr(args, 'tui_rust', False):
        from .launch_tui_rust import launch_tui_rust
        return launch_tui_rust()
    else:
        # Existing TUI launch code
        return _launch_tui(args)
```

---

#### Step 2.4: Verify CLI Integration

```bash
# Build the Rust TUI
cd hermes-tui-rust
cargo build --release

# Test the flag
cd /home/lxrdxe7o/.hermes/hermes-agent
python -m hermes_cli.main --tui-rust --help

# Should show the flag
python -m hermes_cli.main --help | grep tui-rust
```

---

### Task 3: Test Gateway Integration

#### Overview
Test that the Rust TUI works with the actual Hermes gateway.

---

#### Step 3.1: Build and Run

```bash
# Build Rust TUI in release mode
cd /home/lxrdxe7o/.hermes/hermes-agent/hermes-tui-rust
cargo build --release

# Copy binary to accessible location
cp target/release/hermes-tui-rust /usr/local/bin/
```

---

#### Step 3.2: Test with Gateway

```bash
# Method 1: Pipe through gateway
# Run gateway in one terminal
python -m tui_gateway.server

# Run Rust TUI in another terminal, piped through gateway
# (This depends on how gateway expects to communicate)
```

**Note**: The exact integration method depends on how the gateway expects TUI clients to connect. See `tui_gateway/server.py` for the protocol.

---

#### Step 3.3: Test Message Flow

Test each message type:

1. **Gateway Ready** - Should receive capabilities
2. **Prompt Submit** - Should send message and receive response
3. **Message Delta** - Should display streaming content
4. **Message Complete** - Should display complete message
5. **Session List** - Should receive session list
6. **Session Resume** - Should switch to session
7. **Tool Start/Progress/Complete** - Should show tool cards
8. **Approval Request** - Should show approval prompt

---

#### Step 3.4: Fix Any Issues

If protocol mismatches are found:

1. Compare `src/protocol/types.rs` with:
   - `tui_gateway/server.py`
   - `ui-tui/src/gatewayTypes.ts`

2. Fix any serialization differences
3. Fix any field name differences
4. Fix any message type differences

---

### Task 4: Implement oh-my-pi Features

#### Overview
Implement the key differentiators from oh-my-pi:
- Hashline edits
- Tool cards
- Subagent UI
- LSP/debugger UI

---

#### Step 4.1: Hashline Edits

**File**: `src/ui/hashline.rs` (NEW)

**Purpose**: Content-hash anchored patch system for editing agent messages

**Implementation**:

```rust
use ratatui::prelude::*;
use crate::state::messages::Message;
use std::collections::HashMap;

/// Hashline anchor for message editing
#[derive(Debug, Clone)]
pub struct HashlineAnchor {
    /// Hash of the original content
    pub content_hash: String,
    /// Message ID this anchor refers to
    pub message_id: String,
    /// Position in the message
    pub position: (u16, u16), // (line, column)
    /// Current state of the anchor
    pub state: AnchorState,
}

/// Anchor state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorState {
    /// Anchor is valid and content matches
    Valid,
    /// Content has changed, anchor is stale
    Stale,
    /// Anchor has been applied
    Applied,
    /// Anchor has been dismissed
    Dismissed,
}

/// Hashline edit display
pub struct HashlineComponent<'a> {
    messages: &'a [Message],
    anchors: Vec<HashlineAnchor>,
    selected_anchor: Option<usize>,
    show_diff: bool,
}

impl<'a> HashlineComponent<'a> {
    pub fn new(messages: &'a [Message]) -> Self {
        Self {
            messages,
            anchors: Vec::new(),
            selected_anchor: None,
            show_diff: false,
        }
    }
    
    /// Extract hash anchors from messages
    pub fn extract_anchors(&mut self) {
        self.anchors.clear();
        
        for message in self.messages {
            if let Some(anchors) = extract_hash_anchors(&message.content) {
                for anchor in anchors {
                    self.anchors.push(HashlineAnchor {
                        content_hash: anchor.hash,
                        message_id: message.message_id.clone(),
                        position: anchor.position,
                        state: AnchorState::Valid,
                    });
                }
            }
        }
    }
    
    /// Validate anchors against current content
    pub fn validate_anchors(&mut self) {
        for anchor in &mut self.anchors {
            // Find the message
            if let Some(message) = self.messages.iter().find(|m| m.message_id == anchor.message_id) {
                // Calculate hash of current content at position
                let current_hash = calculate_content_hash(&message.content, anchor.position);
                
                if current_hash != anchor.content_hash {
                    anchor.state = AnchorState::Stale;
                } else {
                    anchor.state = AnchorState::Valid;
                }
            }
        }
    }
    
    /// Render the hashline component
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Render anchors as visual indicators
        // Show diff when selected
        // Allow accept/reject of edits
    }
}

/// Extract hash anchors from content
fn extract_hash_anchors(content: &str) -> Option<Vec<HashAnchor>> {
    // Parse content for hash:... patterns
    // Return vector of anchors
    None
}

/// Calculate content hash at position
fn calculate_content_hash(content: &str, position: (u16, u16)) -> String {
    // Calculate hash of content at specific position
    String::new()
}
```

**Implementation Steps**:

1. Define anchor structures
2. Implement hash calculation
3. Implement anchor extraction
4. Implement anchor validation
5. Implement rendering
6. Integrate with chat display

---

#### Step 4.2: Tool Cards

**File**: Enhance `src/ui/cards.rs`

**Current**: ToolCard struct exists but not wired to actual tool execution

**Enhancements Needed**:

1. Connect to gateway tool messages:
   - ToolStart → Create ToolCard
   - ToolProgress → Update ToolCard
   - ToolComplete → Complete ToolCard

2. Add to app.rs:
   - Track active tool cards
   - Update tool cards from gateway messages
   - Display tool cards in chat

3. Add features to ToolCard:
   - Progress bar for long-running tools
   - Status indicators
   - Expand/collapse details
   - Output display with syntax highlighting

---

#### Step 4.3: Subagent UI

**File**: `src/ui/subagents.rs` (NEW)

**Purpose**: Visual management of parallel subagents

**Implementation**:

```rust
use ratatui::prelude::*;
use std::collections::HashMap;

/// Subagent information
#[derive(Debug, Clone)]
pub struct SubagentInfo {
    pub id: String,
    pub name: String,
    pub task: String,
    pub status: SubagentStatus,
    pub progress: f32, // 0.0 to 1.0
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Subagent status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubagentStatus {
    /// Subagent is waiting to start
    Pending,
    /// Subagent is running
    Running,
    /// Subagent has completed
    Completed,
    /// Subagent has errored
    Errored,
    /// Subagent was cancelled
    Cancelled,
}

/// Subagent visualization component
pub struct SubagentComponent {
    subagents: HashMap<String, SubagentInfo>,
    show_details: bool,
    selected_subagent: Option<String>,
}

impl SubagentComponent {
    pub fn new() -> Self {
        Self {
            subagents: HashMap::new(),
            show_details: false,
            selected_subagent: None,
        }
    }
    
    /// Add a new subagent
    pub fn add_subagent(&mut self, info: SubagentInfo) {
        self.subagents.insert(info.id.clone(), info);
    }
    
    /// Update subagent status
    pub fn update_subagent(&mut self, id: &str, status: SubagentStatus) {
        if let Some(info) = self.subagents.get_mut(id) {
            info.status = status;
        }
    }
    
    /// Update subagent progress
    pub fn update_progress(&mut self, id: &str, progress: f32) {
        if let Some(info) = self.subagents.get_mut(id) {
            info.progress = progress.clamp(0.0, 1.0);
        }
    }
    
    /// Set subagent result
    pub fn set_result(&mut self, id: &str, result: String) {
        if let Some(info) = self.subagents.get_mut(id) {
            info.result = Some(result);
            info.status = SubagentStatus::Completed;
        }
    }
    
    /// Set subagent error
    pub fn set_error(&mut self, id: &str, error: String) {
        if let Some(info) = self.subagents.get_mut(id) {
            info.error = Some(error);
            info.status = SubagentStatus::Errored;
        }
    }
    
    /// Remove a subagent
    pub fn remove_subagent(&mut self, id: &str) {
        self.subagents.remove(id);
    }
    
    /// Render the subagent component
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Render subagent list
        // Show progress bars
        // Show status indicators
        // Show details if expanded
    }
}
```

**Integration**:

1. Add to app.rs:
   - `subagent_component: SubagentComponent`
   - Update on delegation messages
   - Render in UI

2. Handle gateway messages:
   - delegate_task.start → add_subagent
   - delegate_task.progress → update_progress
   - delegate_task.complete → set_result
   - delegate_task.error → set_error

---

## 🧪 TESTING GUIDE

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Show output
cargo test -- --nocapture

# Release build tests
cargo test --release

# Specific module tests
cargo test handlers::
cargo test protocol::
cargo test state::
cargo test ui::
```

### Creating New Tests

All tests are inline in the source files using `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        // Test code here
        assert_eq!(result, expected);
    }
}
```

### Test Coverage

Check coverage using cargo-tarpaulin:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin

# Run specific tests with coverage
cargo tarpaulin -- --test-threads=1
```

---

## 📊 QUALITY GATES

Before merging any code, ensure:

1. **All tests pass**
   ```bash
   cargo test
   ```

2. **No clippy warnings**
   ```bash
   cargo clippy
   ```

3. **Code formatted**
   ```bash
   cargo fmt --check
   ```

4. **Documentation builds**
   ```bash
   cargo doc --no-deps
   ```

5. **Build succeeds**
   ```bash
   cargo build --release
   ```

---

## 🔄 COMMIT STRATEGY

### Atomic Commits

Each commit should be:
1. **Small**: One logical change
2. **Complete**: All related files included
3. **Tested**: All tests pass
4. **Documented**: Clear commit message

### Commit Message Format

```
[type] description - details

Body (optional): More details about the change

Generated by Mistral Vibe.
Co-Authored-By: Mistral Vibe <vibe@mistral.ai>
```

### Commit Types

| Type | Use For |
|------|---------|
| FOUNDATION | Phase 1 tasks |
| STATE | Phase 2 tasks |
| UI | Phase 3 tasks |
| HANDLERS | Phase 4 tasks |
| INTEGRATION | Phase 5 tasks |
| FEATURES | Phase 6 tasks |
| TESTING | Phase 7 tasks |
| POLISH | Phase 8 tasks |
| FIX | Bug fixes |
| DOCS | Documentation only |
| REF | Refactoring |
| CHORE | Maintenance tasks |

### Making a Commit

```bash
# Stage changes
git add <files>

# Check status
git status

# Commit with message
git commit -m "[HANDLERS] keys: Implement KeyHandler with default bindings

Generated by Mistral Vibe.
Co-Authored-By: Mistral Vibe <vibe@mistral.ai>"

# Verify commit
git show HEAD
```

---

## 📞 TROUBLESHOOTING

### Common Issues

**Issue: Compilation Errors**
```bash
# Run cargo check first
cargo check

# Fix errors one at a time
# Most common: missing imports, type mismatches, borrow checker issues
```

**Issue: Tests Failing**
```bash
# Run specific failing test
cargo test test_name -- --nocapture

# Check what the test expects
# Fix the implementation or the test
```

**Issue: Gateway Not Responding**
```bash
# Check gateway is running
# Check connection with manual test
# Compare protocol types between Rust and Python
```

**Issue: Rollback Needed**
```bash
# List recent commits
git log --oneline -5

# Rollback specific commit
git revert <commit-hash>

# Or reset to known good state
git reset --hard 8dda25155
```

---

## 🎯 RECOMMENDED WORKFLOW

### For New Developers

1. **Read Documentation** (1-2 hours)
   - Start with `00_INDEX.md`
   - Read `01_EXECUTIVE_SUMMARY.md`
   - Read `02_CODEBASE_STRUCTURE.md`
   - Read `PHASE2_STATUS.md`

2. **Set Up Environment** (10 minutes)
   ```bash
   cd hermes-tui-rust
   cargo build --release
   cargo test
   ```

3. **Start with First Task** (Pick one)
   - Implement handlers (recommended)
   - Add CLI integration
   - Test gateway integration

4. **Work in Small Increments**
   - Implement one function at a time
   - Test after each change
   - Commit after each logical change

5. **Follow Best Practices**
   - Write tests
   - Document code
   - Follow naming conventions
   - Keep commits atomic

---

## 📚 ADDITIONAL RESOURCES

### Internal Documentation
- `ARCHITECTURE.md` - Technical architecture
- `IMPLEMENTATION_PLAN.md` - Full implementation plan
- `PHASE2_ORCHESTRATION.md` - Subagent orchestration
- `PHASE2_STATUS.md` - Current status

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [ratatui Documentation](https://ratatui.rs/)
- [crossterm Documentation](https://docs.rs/crossterm)
- [Hermes TypeScript TUI](https://github.com/NousResearch/hermes-agent/tree/main/ui-tui)
- [Hermes Gateway Protocol](https://github.com/NousResearch/hermes-agent/tree/main/tui_gateway)
- [oh-my-pi Repository](https://github.com/can1357/oh-my-pi)

---

## ✅ CHECKLIST FOR COMPLETION

### Before Moving to New Agent

- [ ] All handlers implemented and tested
- [ ] CLI integration complete
- [ ] Gateway integration tested
- [ ] oh-my-pi features implemented (at least partially)
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] Documentation updated
- [ ] Commit history clean (atomic commits)

### For Full Project Completion

- [ ] All 8 phases complete
- [ ] All oh-my-pi features implemented
- [ ] Full test coverage (>85%)
- [ ] Performance optimized
- [ ] Documentation complete
- [ ] User testing done
- [ ] Ready for production use

---

*Document Version: 2.0*  
*Last Updated: 2026-06-14*  
*Purpose: Guide new developers in continuing the project*
