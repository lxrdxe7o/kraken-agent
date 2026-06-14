# Hermes TUI Rust - Architecture Document

## Overview

This document describes the architecture for a Rust-based TUI for Hermes Agent, inspired by the interface of oh-my-pi.

## Goals

1. **Shell-like command interface** - Terminal shell with command execution
2. **Rich chat transcript display** - Formatted chat messages with syntax highlighting
3. **Multi-line text input** - Composer with multi-line editing support
4. **Customizable themes** - Color schemes and UI styling
5. **stdio JSON-RPC communication** - Communication with Hermes gateway
6. **ratatui-based** - Using the mature ratatui library

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Hermes TUI Rust (Binary)                       │
├─────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │      App         │  │    Protocol      │  │    State     │  │
│  │  (main.rs)       │  │   (mod.rs)       │  │   (mod.rs)    │  │
│  └────────┬────────┘  └────────┬────────┘  └──────┬──────┘  │
│           │                   │                    │           │
│           ▼                   ▼                    ▼           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    Event Loop (app.rs)                      ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐   ││
│  │  │   Handlers  │  │    UI       │  │   Transport       │   ││
│  │  │ (mod.rs)    │  │  (mod.rs)    │  │   (transport.rs)  │   ││
│  │  └──────┬──────┘  └──────┬──────┘  └───────┬────────┘   ││
│  │         │                │                 │            ││
│  │         ▼                ▼                 ▼            ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   ││
│  │  │   keys.rs   │  │  chat.rs     │  │   JSON-RPC   │   ││
│  │  └─────────────┘  └──────┬──────┘  └─────────────┘   ││
│  │                           │                          ││
│  │                           ▼                          ││
│  │                    ┌─────────────────┐               ││
│  │                    │  composer.rs     │               ││
│  │                    │  toolbar.rs      │               ││
│  │                    │  prompts.rs      │               ││
│  │                    │  cards.rs        │               ││
│  │                    └─────────────────┘               ││
│  └─────────────────────────────────────────────────────────┘│
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    stdio (Python Gateway)                  ││
│  │  JSON-RPC messages ←───────────────────────► Rust TUI     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                         ↓
              ┌─────────────────────┐
              │   Hermes Core        │
              │   (run_agent.py)     │
              └─────────────────────┘
```

## Module Structure

```
hermes-tui-rust/
├── Cargo.toml                    # Project configuration
├── build.rs                      # Build script (for version info)
├── src/
│   ├── main.rs                   # Binary entry point
│   ├── lib.rs                    # Library entry point
│   ├── app.rs                    # Main App struct and event loop
│   ├── protocol/
│   │   ├── mod.rs                # Protocol module exports
│   │   ├── types.rs              # JSON-RPC message types
│   │   ├── client.rs             # JSON-RPC client implementation
│   │   └── transport.rs          # stdio transport layer
│   ├── state/
│   │   ├── mod.rs                # State module exports
│   │   ├── session.rs            # Session management
│   │   ├── messages.rs           # Message history and management
│   │   └── config.rs             # TUI configuration and themes
│   ├── ui/
│   │   ├── mod.rs                # UI module exports
│   │   ├── chat.rs               # Chat transcript display
│   │   ├── composer.rs           # Multi-line text input
│   │   ├── toolbar.rs            # Status toolbar
│   │   ├── prompts.rs            # User prompt dialogs
│   │   └── cards.rs              # UI card components
│   ├── handlers/
│   │   ├── mod.rs                # Handler module exports
│   │   ├── keys.rs               # Keyboard event handling
│   │   ├── mouse.rs              # Mouse event handling
│   │   └── input.rs              # Text input handling
│   └── utils/
│       ├── mod.rs                # Utility module exports
│       ├── text.rs               # Text processing utilities
│       └── ansi.rs               # ANSI code handling
├── tests/
│   ├── protocol_test.rs          # Protocol layer tests
│   ├── state_test.rs             # State management tests
│   ├── ui_test.rs                # UI component tests
│   └── integration_test.rs       # Integration tests
└── README.md                     # Project documentation
```

## Component Details

### 1. App Module (app.rs)

The main application struct that orchestrates everything:

```rust
pub struct App {
    /// Protocol client for gateway communication
    client: ProtocolClient,
    /// Application state
    state: AppState,
    /// Terminal interface
    terminal: Terminal<CrosstermBackend<std::io::Stderr>>,
    /// Event stream
    events: EventStream,
}

impl App {
    pub fn new() -> Result<Self>;
    pub fn run(&mut self) -> Result<()>;
    pub fn handle_event(&mut self, event: Event) -> Result<()>;
    pub fn draw(&mut self) -> Result<()>;
}
```

**Responsibilities:**
- Initialize all components
- Main event loop
- Coordinate between UI, state, and protocol
- Graceful shutdown

### 2. Protocol Module

Handles communication with the Hermes gateway via JSON-RPC over stdio.

#### Types (types.rs)

Defines all JSON-RPC message types that the TUI can send/receive:

```rust
// Request types (TUI → Gateway)
#[derive(Serialize, Deserialize, Debug)]
pub enum TuiRequest {
    /// Submit a user message
    PromptSubmit { message: String },
    /// Request session list
    SessionList,
    /// Resume a session
    SessionResume { session_id: String },
    /// Create new session
    SessionNew,
    /// Slash command execution
    SlashExec { command: String },
    /// Approval response
    ApprovalRespond { approve: bool, request_id: String },
    /// Completion request
    Complete { query: String },
    // ... more
}

// Response types (Gateway → TUI)
#[derive(Serialize, Deserialize, Debug)]
pub enum TuiResponse {
    /// Stream delta for assistant message
    MessageDelta { content: String, message_id: String },
    /// Complete message
    MessageComplete { message: Message, message_id: String },
    /// Session list
    SessionList { sessions: Vec<SessionInfo> },
    /// Approval request
    ApprovalRequest { request_id: String, prompt: String },
    /// Completion results
    Complete { items: Vec<String> },
    /// Tool start
    ToolStart { tool_name: String, tool_id: String },
    /// Tool progress
    ToolProgress { tool_id: String, output: String },
    /// Tool complete
    ToolComplete { tool_id: String, result: String },
    /// Gateway ready
    GatewayReady { capabilities: Capabilities },
    /// Error
    Error { error: String, code: i32 },
    // ... more
}

// Message structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub message_id: String,
    // ... more fields
}
```

#### Transport (transport.rs)

Low-level stdio transport for JSON-RPC:

```rust
pub struct StdioTransport {
    reader: BufReader<std::io::Stdin>,
    writer: std::io::Stdout,
    // ...
}

impl StdioTransport {
    pub fn new() -> Self;
    pub fn read_message(&mut self) -> Result<TuiResponse>;
    pub fn write_message(&mut self, message: &TuiRequest) -> Result<()>;
    pub fn start_reader_thread(&mut self, sender: Sender<TuiResponse>) 
        -> thread::JoinHandle<()>;
}
```

**Key Design Decisions:**
- Use background thread for reading stdin (non-blocking)
- Synchronous writes to stdout
- Channel-based message passing to main thread
- JSON serialization for all messages

#### Client (client.rs)

High-level JSON-RPC client:

```rust
pub struct ProtocolClient {
    transport: StdioTransport,
    receiver: Receiver<TuiResponse>,
    request_id: AtomicU64,
    // ...
}

impl ProtocolClient {
    pub fn new() -> Result<Self>;
    pub fn send_request(&mut self, request: TuiRequest) -> Result<()>;
    pub fn recv_response(&mut self) -> Result<Option<TuiResponse>>;
    pub fn is_connected(&self) -> bool;
    pub fn disconnect(&mut self);
}
```

**Responsibilities:**
- Manage request/response lifecycle
- Generate request IDs
- Handle connection state
- Provide async-friendly interface

### 3. State Module

Manages all application state.

#### Session (session.rs)

Session management:

```rust
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub name: Option<String>,
    pub messages: Vec<Message>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub struct SessionManager {
    sessions: HashMap<String, Session>,
    current_session_id: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self;
    pub fn get_current(&self) -> Option<&Session>;
    pub fn switch_session(&mut self, session_id: &str) -> Result<()>;
    pub fn add_message(&mut self, message: Message) -> Result<()>;
    pub fn clear_current(&mut self);
}
```

#### Messages (messages.rs)

Message history and management:

```rust
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

pub struct MessageHistory {
    messages: Vec<Message>,
    max_messages: usize,
}

impl MessageHistory {
    pub fn new(max_messages: usize) -> Self;
    pub fn push(&mut self, message: Message);
    pub fn get(&self, index: usize) -> Option<&Message>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &Message>;
    pub fn clear(&mut self);
}
```

#### Config (config.rs)

TUI configuration and themes:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Theme settings
    pub theme: ThemeConfig,
    /// Keybindings
    pub keybindings: Keybindings,
    /// Display settings
    pub display: DisplayConfig,
    /// Editor settings
    pub editor: EditorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ThemeColors,
    pub styles: ThemeStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
    // ... more
}

/// Predefined themes
pub enum BuiltinTheme {
    Default,
    Dark,
    Light,
    Solarized,
    Dracula,
    // ... more
}

impl BuiltinTheme {
    pub fn to_config(&self) -> ThemeConfig;
    pub fn all() -> &'static [Self];
}
```

### 4. UI Module

All UI components using ratatui.

#### Chat (chat.rs)

Chat transcript display:

```rust
pub struct ChatWidget<'a> {
    messages: &'a [Message],
    scroll: usize,
    width: u16,
    height: u16,
}

impl<'a> ChatWidget<'a> {
    pub fn new(messages: &'a [Message]) -> Self;
    pub fn scroll_up(&mut self);
    pub fn scroll_down(&mut self);
    pub fn scroll_to_bottom(&mut self);
    pub fn render(&self, frame: &mut Frame) -> Result<()>;
}

impl<'a> Widget for ChatWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut State);
}
```

**Features:**
- Scrollable message history
- Syntax highlighting for code blocks
- Message formatting (markdown support)
- Timestamps
- Author indicators
- Tool call display

#### Composer (composer.rs)

Multi-line text input:

```rust
pub struct ComposerWidget {
    input: String,
    cursor: usize,
    scroll: usize,
    history: Vec<String>,
    history_index: Option<usize>,
}

impl ComposerWidget {
    pub fn new() -> Self;
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<ComposerAction>;
    pub fn submit(&mut self) -> String;
    pub fn is_empty(&self) -> bool;
    pub fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}

pub enum ComposerAction {
    None,
    Submit,
    Cancel,
    HistoryUp,
    HistoryDown,
}
```

**Features:**
- Multi-line editing
- Keyboard navigation
- History (up/down arrows)
- Syntax highlighting (as you type)
- Auto-indentation
- Tab completion

#### Toolbar (toolbar.rs)

Status and information toolbar:

```rust
pub struct ToolbarWidget {
    status: String,
    mode: InputMode,
    session_name: Option<String>,
    model_name: Option<String>,
    tool_progress: Option<ToolProgress>,
}

pub enum InputMode {
    Normal,
    Insert,
    Command,
}

impl ToolbarWidget {
    pub fn new() -> Self;
    pub fn set_status(&mut self, status: impl Into<String>);
    pub fn set_mode(&mut self, mode: InputMode);
    pub fn set_session(&mut self, session: Option<String>);
    pub fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}
```

#### Prompts (prompts.rs)

User prompt dialogs:

```rust
pub struct PromptWidget {
    title: String,
    message: String,
    options: Vec<String>,
    selected: usize,
}

impl PromptWidget {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self;
    pub fn with_options(mut self, options: Vec<String>) -> Self;
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<PromptAction>;
    pub fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}

pub enum PromptAction {
    Select(usize),
    Cancel,
    None,
}
```

**Prompt Types:**
- Confirmation (Yes/No)
- Multiple choice
- Text input
- Password input
- Approval prompts

#### Cards (cards.rs)

UI card components for various display purposes:

```rust
pub struct Card {
    pub title: Option<String>,
    pub content: String,
    pub border_style: BorderStyle,
    pub padding: Padding,
}

impl Widget for Card {
    fn render(self, area: Rect, buf: &mut Buffer);
}

// Specialized cards
pub struct ToolCard {
    pub tool_name: String,
    pub status: ToolStatus,
    pub output: String,
}

pub struct ErrorCard {
    pub title: String,
    pub message: String,
    pub details: Option<String>,
}

pub struct LoadingCard {
    pub message: String,
    pub spinner: Spinner,
}
```

### 5. Handlers Module

Event handling for user input.

#### Keys (keys.rs)

Keyboard event handling:

```rust
pub struct KeyHandler {
    config: Keybindings,
}

impl KeyHandler {
    pub fn new(config: Keybindings) -> Self;
    pub fn handle(&self, key: KeyEvent, state: &mut AppState) -> Result<Action>;
}

pub enum Action {
    /// Composer actions
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
    
    /// Navigation actions
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    GoToTop,
    GoToBottom,
    
    /// Session actions
    NewSession,
    PreviousSession,
    NextSession,
    ListSessions,
    
    /// Command actions
    OpenCommandPalette,
    
    /// System actions
    Quit,
    Suspend,
    ToggleTheme,
    ToggleHelp,
    
    /// Other
    Noop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybindings {
    pub normal: HashMap<KeyEvent, Action>,
    pub insert: HashMap<KeyEvent, Action>,
    pub command: HashMap<KeyEvent, Action>,
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut normal = HashMap::new();
        normal.insert(KeyEvent::Char('i'), Action::ComposerInsert);
        normal.insert(KeyEvent::Char('q'), Action::Quit);
        // ... more default bindings
        
        Self {
            normal,
            insert: HashMap::new(),
            command: HashMap::new(),
        }
    }
}
```

#### Mouse (mouse.rs)

Mouse event handling:

```rust
pub struct MouseHandler;

impl MouseHandler {
    pub fn new() -> Self;
    pub fn handle(&self, mouse: MouseEvent, area: Rect) -> Result<Option<Action>>;
}
```

**Features:**
- Click to focus composer
- Scroll wheel for chat
- Click on messages for actions

#### Input (input.rs)

Text input handling:

```rust
pub struct InputHandler {
    composer: ComposerWidget,
}

impl InputHandler {
    pub fn new() -> Self;
    pub fn handle_char(&mut self, c: char);
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<ComposerAction>;
    pub fn get_text(&self) -> &str;
    pub fn clear(&mut self);
    pub fn submit(&mut self) -> String;
}
```

### 6. Utils Module

Utility functions and helpers.

#### Text (text.rs)

Text processing utilities:

```rust
pub mod text {
    /// Wrap text to fit width
    pub fn wrap_text(text: &str, width: u16) -> Vec<String>;
    
    /// Truncate text with ellipsis
    pub fn truncate(text: &str, max_len: usize) -> String;
    
    /// Extract code blocks from markdown
    pub fn extract_code_blocks(text: &str) -> Vec<(String, String)>;
    
    /// Apply syntax highlighting to code
    pub fn highlight_code(code: &str, language: &str) -> Result<String>;
    
    /// Format markdown for terminal display
    pub fn format_markdown(text: &str, width: u16) -> Result<Vec<Line>>;
}
```

#### ANSI (ansi.rs)

ANSI code handling:

```rust
pub mod ansi {
    /// Remove ANSI escape codes from text
    pub fn strip_ansi(text: &str) -> String;
    
    /// Get text width accounting for ANSI codes
    pub fn ansi_width(text: &str) -> usize;
    
    /// Truncate ANSI text
    pub fn truncate_ansi(text: &str, max_len: usize) -> String;
}
```

## Event Loop

The main event loop in `app.rs`:

```rust
impl App {
    pub fn run(&mut self) -> Result<()> {
        // Initial draw
        self.terminal.draw(|f| self.draw(f))?;
        
        // Main loop
        loop {
            // Check for events with timeout
            if let Ok(event) = self.events.next()? {
                if !self.handle_event(event)? {
                    break;
                }
            }
            
            // Check for gateway messages
            while let Ok(Some(response)) = self.client.recv_response() {
                self.handle_gateway_response(response)?;
            }
            
            // Draw UI
            self.terminal.draw(|f| self.draw(f))?;
            
            // Check for quit
            if self.should_quit {
                break;
            }
        }
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Key(key) => {
                let action = self.key_handler.handle(key, &mut self.state)?;
                self.handle_action(action)?;
            }
            Event::Mouse(mouse) => {
                // Handle mouse
            }
            Event::Resize(w, h) => {
                self.terminal.resize(w, h)?;
            }
            Event::Paste(text) => {
                // Handle paste
            }
        }
        Ok(true)
    }
    
    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::ComposerSubmit => {
                let text = self.composer.submit();
                if !text.is_empty() {
                    self.send_prompt(text)?;
                }
            }
            Action::Quit => {
                self.should_quit = true;
            }
            // ... handle other actions
        }
        Ok(())
    }
    
    fn handle_gateway_response(&mut self, response: TuiResponse) -> Result<()> {
        match response {
            TuiResponse::MessageDelta { content, message_id } => {
                self.state.add_delta(content, message_id);
            }
            TuiResponse::MessageComplete { message, message_id } => {
                self.state.complete_message(message, message_id);
            }
            TuiResponse::ApprovalRequest { request_id, prompt } => {
                self.show_approval_prompt(request_id, prompt);
            }
            // ... handle other responses
        }
        Ok(())
    }
}
```

## Communication Protocol

### Message Flow

```
TUI → Gateway:
- PromptSubmit: User submits a message
- SessionList: Request list of sessions
- SessionResume: Resume a specific session
- SessionNew: Create new session
- SlashExec: Execute slash command
- ApprovalRespond: User responds to approval request
- Complete: Request completions

Gateway → TUI:
- MessageDelta: Streaming message content
- MessageComplete: Complete message
- SessionList: Response with session list
- ApprovalRequest: Request user approval
- Complete: Completion results
- ToolStart: Tool execution started
- ToolProgress: Tool output update
- ToolComplete: Tool execution completed
- GatewayReady: Gateway capabilities
- Error: Error occurred
```

### Message Format

All messages are JSON-RPC 2.0 compliant:

```json
// Request from TUI
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "prompt.submit",
  "params": {
    "message": "Hello, Hermes!"
  }
}

// Notification from TUI (no response expected)
{
  "jsonrpc": "2.0",
  "method": "approval.respond",
  "params": {
    "request_id": "abc123",
    "approve": true
  }
}

// Response from Gateway
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "session_id": "xyz789"
  }
}

// Notification from Gateway
{
  "jsonrpc": "2.0",
  "method": "message.delta",
  "params": {
    "content": "Hello",
    "message_id": "msg123"
  }
}
```

## Theme System

The theme system allows customization of the TUI appearance:

```rust
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
    pub styles: ThemeStyles,
}

pub struct ThemeColors {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
    pub chat: ChatColors,
    pub composer: ComposerColors,
    pub toolbar: ToolbarColors,
}

pub struct ChatColors {
    pub user_bg: Color,
    pub user_text: Color,
    pub assistant_bg: Color,
    pub assistant_text: Color,
    pub system_bg: Color,
    pub system_text: Color,
    pub tool_bg: Color,
    pub tool_text: Color,
    pub code_bg: Color,
    pub code_text: Color,
    pub border: Color,
    pub timestamp: Color,
}

// Predefined themes
pub fn default_theme() -> Theme { ... }
pub fn dark_theme() -> Theme { ... }
pub fn light_theme() -> Theme { ... }
pub fn solarized_theme() -> Theme { ... }
pub fn dracula_theme() -> Theme { ... }
```

## Configuration

Configuration files:
- `~/.hermes/tui-rust/config.yaml` - Main configuration
- `~/.hermes/tui-rust/themes/*.yaml` - Custom themes

Example config:

```yaml
# TUI Configuration
theme: solarized
keybindings:
  normal:
    i: composer_insert
    q: quit
    k: scroll_up
    j: scroll_down
  insert:
    Esc: normal_mode
    Enter: submit

display:
  show_timestamps: true
  show_session_name: true
  syntax_highlighting: true
  max_message_width: 80

editor:
  auto_indent: true
  tab_width: 4
  history_size: 100
```

## Error Handling

Error handling strategy:

1. **Recoverable errors**: Display error message in UI, continue running
2. **Connection errors**: Show reconnection prompt, auto-retry
3. **Fatal errors**: Display error, offer to restart or quit
4. **Panic handling**: Catch panics, display error, continue if possible

```rust
// Error types
#[derive(Debug, ThisError)]
pub enum TuiError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("State error: {0}")]
    StateError(String),
    
    #[error("Render error: {0}")]
    RenderError(String),
}

// Result type
pub type TuiResult<T> = Result<T, TuiError>;
```

## Testing Strategy

### Unit Tests
- Protocol message serialization/deserialization
- State management operations
- Text processing utilities
- Theme configuration

### Integration Tests
- TUI ↔ Gateway message flow
- Session management
- Message history

### E2E Tests
- Full interaction scenarios
- Keybinding validation
- UI rendering validation

### Test Coverage Goals
- Protocol: 100%
- State: 100%
- Utils: 100%
- UI: 90% (some rendering tests are hard to automate)
- Handlers: 95%
- App: 85%

## Performance Considerations

1. **Message streaming**: Process delta messages immediately for smooth typing effect
2. **Rendering optimization**: Only re-render changed areas when possible
3. **Memory management**: Limit message history, truncate old messages
4. **Input handling**: Non-blocking event loop for responsive UI
5. **Syntax highlighting**: Cache highlighting results, lazy evaluation

## Security Considerations

1. **Input validation**: Validate all gateway messages before processing
2. **Output sanitization**: Sanitize all text before display (remove harmful ANSI codes)
3. **Sensitive data**: Don't log sensitive user input
4. **Command execution**: Validate slash commands before executing

## Compatibility

- **Platforms**: Linux, macOS, Windows (via crossterm)
- **Terminals**: Any terminal supporting ANSI escape codes
- **Minimum Rust version**: 1.75 (as specified in Cargo.toml)

## Future Enhancements

1. **Plugin system**: Allow custom UI components and themes
2. **Multi-pane layout**: Split view for chat + tools
3. **Command palette**: Fuzzy search for commands
4. **History search**: Search message history
5. **Custom keybindings**: User-defined keybindings
6. **Theme editor**: Interactive theme customization
7. **Performance metrics**: Show token usage, response times

## Migration Path

The Rust TUI will coexist with the existing TypeScript TUI initially:

1. **Phase 1**: Basic functionality (chat, composer, sessions)
2. **Phase 2**: Feature parity with TypeScript TUI
3. **Phase 3**: Enhanced features (themes, plugins)
4. **Phase 4**: Performance optimization
5. **Phase 5**: Optional replacement or alternative

The existing TypeScript TUI will remain available via `--tui` flag.
The Rust TUI will be available via `--tui-rust` flag initially.

## Build and Development

### Build
```bash
cd hermes-tui-rust
cargo build --release
```

### Run
```bash
# Development
cargo run

# Release
cargo run --release
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show output
cargo test --release        # Test release build
```

### Clippy
```bash
cargo clippy
cargo clippy -- -W clippy::all
```

### Format
```bash
cargo fmt
```

### Documentation
```bash
cargo doc --open
```

## File Naming Conventions

- Modules: `snake_case` (e.g., `protocol`, `state`, `ui`)
- Files: `snake_case.rs` (e.g., `transport.rs`, `chat.rs`)
- Structs: `PascalCase` (e.g., `StdioTransport`, `AppState`)
- Enums: `PascalCase` (e.g., `MessageRole`, `TuiRequest`)
- Functions: `snake_case` (e.g., `handle_event`, `render_chat`)
- Variables: `snake_case` (e.g., `current_session`, `message_history`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_MESSAGES`, `DEFAULT_THEME`)

## Code Style

- Use Rustfmt for formatting
- Use Clippy for linting
- Document all public items
- Use `anyhow` for error handling in application code
- Use `thiserror` for library error types
- Prefer immutable patterns where possible
- Use `Arc<Mutex<T>>` for shared state (consider `Arc<RwLock<T>>` for read-heavy)
- Use `tokio::sync` types when in async context

## Dependencies

Current dependencies (from Cargo.toml):
- `ratatui` 0.28 - TUI framework
- `crossterm` 0.28 - Terminal I/O
- `tui-textarea` 0.4 - Multi-line text input
- `serde` 1.0 - Serialization
- `serde_json` 1.0 - JSON serialization
- `tokio` 1 - Async runtime
- `log` 0.4 - Logging
- `env_logger` 0.10 - Environment-based logging
- `anyhow` 1.0 - Error handling
- `thiserror` 1.0 - Error types
- `clap` 4.0 - CLI argument parsing
- `chrono` 0.4 - Date/time handling
- `uuid` 1.0 - UUID generation
- `url` 2.0 - URL parsing
- `base64` 0.22 - Base64 encoding
- `home` 0.5 - Home directory detection
- `syntect` 5.0 - Syntax highlighting
- `fuzzy-matcher` 0.3 - Fuzzy matching for completions
- `textwrap` 0.16 - Text wrapping
- `unicode-width` 0.1 - Unicode width calculation
- `arboard` 3.0 (optional) - Clipboard support

## Integration with Hermes

The Rust TUI will integrate with Hermes in the following ways:

1. **Gateway communication**: Via JSON-RPC over stdio to `tui_gateway/server.py`
2. **Protocol compatibility**: Must support all message types that the TypeScript TUI supports
3. **Configuration**: Reads from `~/.hermes/config.yaml` (shared with Hermes)
4. **Themes**: Can use Hermes skin system or its own theme system
5. **Plugins**: Can potentially use Hermes plugin system

### Protocol Compatibility

The Rust TUI must support all message types defined in:
- `tui_gateway/server.py` - Server-side message types
- `ui-tui/src/gatewayTypes.ts` - TypeScript type definitions

Key message categories:
- Session management (`session.list`, `session.resume`, `session.create`)
- Chat (`prompt.submit`, `message.delta`, `message.complete`)
- Approvals (`approval.request`, `approval.respond`)
- Completions (`complete.slash`, `complete.path`)
- Slash commands (`slash.exec`, `command.dispatch`)
- Tool execution (`tool.start`, `tool.progress`, `tool.complete`)
- Gateway state (`gateway.ready`)

## Implementation Roadmap

### Phase 1: Foundation (Week 1)
- [ ] Project setup and Cargo.toml
- [ ] Basic App struct and event loop
- [ ] stdio transport implementation
- [ ] Protocol message types
- [ ] Basic state management
- [ ] Simple chat display
- [ ] Basic text input

### Phase 2: Core Features (Week 2)
- [ ] Session management
- [ ] Message history
- [ ] Scrollable chat
- [ ] Multi-line composer
- [ ] Toolbar with status
- [ ] Keyboard navigation
- [ ] Mouse support

### Phase 3: Enhanced Features (Week 3)
- [ ] Syntax highlighting
- [ ] Theme system
- [ ] Command palette
- [ ] Approval prompts
- [ ] Completions
- [ ] Slash commands
- [ ] Tool execution display

### Phase 4: Polish (Week 4)
- [ ] Error handling
- [ ] Performance optimization
- [ ] Configuration system
- [ ] Documentation
- [ ] Testing
- [ ] CI/CD integration

### Phase 5: Integration (Week 5)
- [ ] Integration with Hermes gateway
- [ ] End-to-end testing
- [ ] User testing
- [ ] Bug fixes
- [ ] Final polish

## Success Criteria

1. **Functional**: All core features work correctly
2. **Reliable**: No crashes, graceful error handling
3. **Responsive**: Smooth UI, no lag
4. **Compatible**: Works with all Hermes gateway features
5. **Tested**: Comprehensive test coverage
6. **Documented**: Clear documentation and comments
7. **Maintainable**: Clean code, good architecture

## References

- [ratatui documentation](https://ratatui.rs/)
- [crossterm documentation](https://docs.rs/crossterm)
- [Hermes TypeScript TUI](ui-tui/)
- [Hermes gateway protocol](tui_gateway/)
- [oh-my-pi repository](https://github.com/can1357/oh-my-pi)
