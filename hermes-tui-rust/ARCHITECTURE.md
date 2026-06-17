# Hermes TUI Rust - Architecture Document

## Overview

This document describes the architecture for a Rust-based TUI for Hermes Agent, inspired by the interface of oh-my-pi. The design follows an **Asynchronous Event-Driven Component Architecture** with four primary layers: the Event/Action Bus, the Central App Engine, the Hierarchical Component Tree, and the Async I/O Worker Pool. This architecture has been refined through a comprehensive architectural critique (see References) to address critical concerns around bounded channels, immutable rendering, demand-driven updates, and biased event prioritisation.

## Goals

1. **Shell-like command interface** - Terminal shell with command execution
2. **Rich chat transcript display** - Formatted chat messages with syntax highlighting, chat bubbles (oatmeal-style)
3. **Multi-line text input** - Composer with multi-line editing support
4. **Customizable themes** - Color schemes and UI styling (24-bit TrueColor, Tailwind palette)
5. **stdio JSON-RPC communication** - Communication with Hermes gateway
6. **ratatui-based** - Using the mature ratatui library
7. **Embedded modal editor** - Vim-inspired editor backed by Rope data structure
8. **Multi-agent orchestration** - Resizable split panes for sub-agent telemetry
9. **60-FPS animation with demand-driven rendering** - Smooth, tear-free transitions using tachyonfx shaders

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                    Hermes TUI Rust (Binary)                            │
├──────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │               Event / Action Bus (tokio)                      │      │
│  │  Bounded channels (capacity 1024) + Watch channels        │      │
│  │  Biased tokio::select! — user input > ticks > background    │      │
│  └──────────┬────────────────────────────────────────────────┘      │
│             │                                                       │
│  ┌──────────▼────────────────────────────────────────────────┐      │
│  │              Central App Engine                               │      │
│  │  - AppState (immutable refs to components)                     │      │
│  │  - Focus state machine (DashboardMode / EditorMode / ...)      │      │
│  │  - Active animation counter → demand-driven ticker              │      │
│  └──────────┬────────────────────────────────────────────────┘      │
│             │                                                       │
│  ┌──────────▼────────────────────────────────────────────────┐      │
│  │            Hierarchical Component Tree                        │      │
│  │  - WidgetRef / StatefulWidget (immutable render)               │      │
│  │  - Chat (oatmeal-style bubbles, tachyonfx fade-in)              │      │
│  │  - Composer (multi-line, Tab completion)                        │      │
│  │  - Editor (Rope-backed, edtui, syntax cache)                    │      │
│  │  - Memory explorer (tree view, async fetch)                     │      │
│  │  - Sub-agent panes (resizable splits)                           │      │
│  │  - Toolbar, Prompts, Cards                                      │      │
│  └─────────────────────────────────────────────────────────────┘      │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │              Async I/O Worker Pool (Tokio)                      │      │
│  │  - JSON-RPC IPC to Hermes Python daemon                         │      │
│  │  - LLM streaming deserialisation → immutable Action enum        │      │
│  │  - SQLite blocking thread for memory fetches                     │      │
│  │  - Sub-agent telemetry routing                                   │      │
│  └──────────┬────────────────────────────────────────────────┘      │
│             │                                                       │
│  ┌──────────▼────────────────────────────────────────────────┐      │
│  │              Synchronised Output (DEC ?2026)                    │      │
│  │  - Atomic frame swap to eliminate terminal tearing               │      │
│  │  - Ratatui cell diffing + ANSI escape sequence emission          │      │
│  └─────────────────────────────────────────────────────────────┘      │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │                    stdio (Python Gateway)                       │      │
│  │  JSON-RPC messages ←───────────────────────────► Rust TUI        │      │
│  └──────────────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────────────┘
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
│   ├── app.rs                    # Main App struct, event loop, biased select
│   ├── bus/
│   │   ├── mod.rs                # Event/Action bus module
│   │   ├── action.rs             # Action enum and channel definitions
│   │   ├── dispatcher.rs         # Central dispatcher with biased polling
│   │   └── animation.rs          # Animation ticker with demand-driven control
│   ├── protocol/
│   │   ├── mod.rs                # Protocol module exports
│   │   ├── types.rs              # JSON-RPC message types
│   │   ├── client.rs             # JSON-RPC client implementation
│   │   └── transport.rs          # stdio transport layer
│   ├── state/
│   │   ├── mod.rs                # State module exports
│   │   ├── session.rs            # Session management
│   │   ├── messages.rs           # Message history and management
│   │   ├── config.rs             # TUI configuration and themes
│   │   └── focus.rs              # Focus state machine (modes → component routing)
│   ├── ui/
│   │   ├── mod.rs                # UI module exports
│   │   ├── chat.rs               # Chat transcript display (oatmeal-style bubbles)
│   │   ├── composer.rs           # Multi-line text input
│   │   ├── editor.rs             # Embedded modal editor (Rope-backed, edtui)
│   │   ├── memory_explorer.rs    # Memory tree view (async SQLite fetch)
│   │   ├── agent_panes.rs        # Sub-agent resizable split panes
│   │   ├── toolbar.rs            # Status toolbar
│   │   ├── prompts.rs            # User prompt dialogs
│   │   └── cards.rs              # UI card components
│   ├── handlers/
│   │   ├── mod.rs                # Handler module exports
│   │   ├── keys.rs               # Keyboard event handling (hierarchical input maps)
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

### 1. Event/Action Bus (bus/)

The central nervous system of the TUI. All communication between layers is mediated by bounded Tokio channels to enforce backpressure and prevent memory exhaustion.

```rust
/// Central action enum — every state change in the system
pub enum Action {
    // Hardware input
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Paste(String),

    // Animation
    Tick(Duration),  // only emitted when animations are active

    // Protocol responses (from Hermes gateway)
    MessageDelta { content: String, message_id: String },
    MessageComplete { message: Message, message_id: String },
    ToolStart { tool_name: String, tool_id: String },
    ToolProgress { tool_id: String, output: String },
    ToolComplete { tool_id: String, result: String },
    GatewayReady { capabilities: Capabilities },
    Error { error: String, code: i32 },

    // Session
    SessionList { sessions: Vec<SessionInfo> },
    SessionSwitched { session_id: String },

    // Approvals
    ApprovalRequest { request_id: String, prompt: String },

    // Internal
    ActiveAnimationCount(u32),  // 0 → suspend ticker
    SetMode(InputMode),
    FocusComponent(ComponentId),
    Quit,
}

/// Channel capacity — MUST be bounded to prevent OOM
const ACTION_BUS_CAPACITY: usize = 1024;

/// High-priority channel for user input (smaller, never drops keystrokes)
const INPUT_CHANNEL_CAPACITY: usize = 64;
```

**Critical design rule**: All event bus channels use `tokio::sync::mpsc::channel` (bounded), never `UnboundedSender`. Bounded channels enforce backpressure: when capacity is reached, senders suspend until the receiver drains a message. This prevents unbounded memory growth during rendering stalls or massive LLM token floods.

**Watch channels** (`tokio::sync::watch`) are used for metrics that only need the latest state — progress bars, CPU/RAM telemetry, agent status indicators — eliminating queue buildup entirely for these values.

#### Dispatcher (dispatcher.rs)

The central dispatcher runs a biased `tokio::select!` loop that polls channels in strict priority order:

```rust
// Biased polling — hardware input always first
tokio::select! {
    biased;  // ← CRITICAL: enforces priority order

    // 1. Hardware input (highest priority)
    Some(event) = input_receiver.recv() => {
        handle_input(event).await;
    }

    // 2. Animation ticks (only active when animations are running)
    Some(tick) = animation_receiver.recv() => {
        handle_tick(tick).await;
    }

    // 3. Internal state mutations
    Some(action) = internal_receiver.recv() => {
        handle_internal(action).await;
    }

    // 4. Background Hermes data (lowest priority)
    Some(response) = gateway_receiver.recv() => {
        handle_gateway(response).await;
    }
}
```

This mirrors the browser event-loop prioritisation model (microtasks before macrotasks), ensuring that user keystrokes and rendering ticks are never starved by high-throughput LLM streaming.

### 2. App Module (app.rs)

The main application struct that orchestrates everything:

```rust
pub struct App {
    /// Dispatchers and channel transmitters
    dispatcher: Dispatcher,
    /// Bound senders for each channel priority tier
    input_tx: mpsc::Sender<Action>,
    gateway_tx: mpsc::Sender<Action>,
    /// Application state (passed immutably to components)
    state: AppState,
    /// Terminal interface
    terminal: Terminal<CrosstermBackend<std::io::Stderr>>,
    /// Animation ticker (suspended when no animations active)
    animator: AnimationController,
}

impl App {
    pub fn new() -> Result<Self>;
    pub fn run(&mut self) -> Result<()>;
    pub fn handle_action(&mut self, action: Action) -> Result<()>;
    pub fn draw(&mut self) -> Result<()>;
}
```

**Responsibilities:**
- Initialize all components and channels
- Main biased-select event loop
- Coordinate between UI, state, and protocol
- Drive demand-controlled animation
- Graceful shutdown

### 3. Protocol Module

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

### 4. State Module

Manages all application state. The Central App Engine holds a single source of truth for global state; it does NOT own the layout tree. Focus routing and layout are handled via a declarative state machine, not hardcoded coordinates.

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

#### Focus (focus.rs)

Declarative focus state machine:

```rust
pub enum AppMode {
    /// Main chat interface — keyboard navigates history, composer focused
    DashboardMode,
    /// Embedded editor active — keystrokes routed to editor component
    EditorMode,
    /// Multi-agent monitoring — splits are active
    AgentChatMode,
    /// Prompt overlay — choices / confirmation
    PromptMode,
}

impl AppMode {
    /// Determine which component receives raw key events
    pub fn focus_target(&self) -> ComponentId;
    /// Whether global keybindings are suspended
    pub fn traps_input(&self) -> bool;
}
```

When the embedded editor enters Insert or Visual modes (`EditorMode`), the central dispatcher suspends global keybinding resolution and pipes all `KeyEvent` payloads directly to the focused editor component. This prevents global hotkeys from triggering while typing code.

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

### 5. UI Module

All UI components using ratatui. **CRITICAL**: rendering must use immutable references (`WidgetRef` trait) or `StatefulWidget` pattern — the `draw` method MUST NOT take `&mut self`. State mutation is handled during the update phase; rendering is strictly read-only, which satisfies the Rust borrow checker and enables aggressive widget caching.

#### Chat (chat.rs)

Chat transcript display with oatmeal-style chat bubbles:

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
}

// Immutable rendering via WidgetRef
impl<'a> WidgetRef for ChatWidget<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer);
}
```

**Features:**
- Oatmeal-style chat bubbles: user messages right-aligned, assistant messages left-aligned
- `tachyonfx` fade-in for streaming tokens (`fx::fade_to_fg` or `fx::coalesce`)
- Scrollable message history
- Syntax highlighting for code blocks
- Message formatting (markdown support)
- Tool call display with progress indicators

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

#### Editor (editor.rs)

Embedded modal editor, powered by `edtui` with Rope backing:

```rust
pub struct EditorWidget {
    /// Rope-based text buffer for O(log n) insert/delete
    buffer: ropey::Rope,
    /// Cached syntax-highlighted lines (invalidated only on mutation)
    syntax_cache: Vec<Vec<Span>>,
    /// Visible viewport bounds
    viewport: Range<usize>,
    mode: EditorMode,
}

pub enum EditorMode {
    Normal,
    Insert,
    Visual,
}

impl EditorWidget {
    pub fn new() -> Self;
    pub fn load_buffer(&mut self, text: &str);
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<EditorAction>;
    pub fn render(&self, area: Rect, buf: &mut Buffer);
}
```

**Critical design rules:**
1. **Rope data structure**: Uses `ropey` crate instead of `String` or `Vec<String>`. A Rope organises text into a balanced tree of string slices, enabling logarithmic time complexity for insertions, deletions, and line lookups regardless of file size (the same architecture underpinning the Helix editor).
2. **Virtual viewports**: Only the subset of lines visible within the physical terminal area are processed every frame — the rest of the document is never touched during rendering.
3. **Syntax caching**: Highlighted span structures are cached. Only lines explicitly altered by user input or LLM generation are invalidated. On rapid scrolling, rendering is reduced to a memory copy of cached spans.
4. **Input trapping**: When in Insert or Visual modes, the central dispatcher routes all key events to the editor, suspending global keybindings.

#### Memory Explorer (memory_explorer.rs)

Tree view for Hermes' persistent memory system:

```rust
pub struct MemoryExplorer {
    /// Async-loaded index tree
    tree: TreeState,
    /// Loading state for non-blocking fetch
    loading: bool,
}

impl MemoryExplorer {
    pub fn new() -> Self;
    /// Dispatches a fetch action via the Action Bus
    pub fn request_fetch(&self, path: &str);
    /// Called when the background pool responds with loaded data
    pub fn on_data_loaded(&mut self, entries: Vec<MemoryEntry>);
}
```

Fetches sub-documents from the SQLite database via the background Tokio pool on a blocking thread. During the round-trip, the component displays a non-blocking spinner animation.

#### Agent Panes (agent_panes.rs)

Resizable split panes for multi-agent orchestration:

```rust
pub struct AgentPaneManager {
    /// One pane per active sub-agent
    panes: Vec<AgentPane>,
    /// Current split layout
    layout: SplitLayout,
}

pub struct AgentPane {
    pub agent_id: String,
    pub title: String,
    pub output: Vec<String>,
    pub status: AgentStatus,
    pub telemetry: Vec<ToolEvent>,
}
```

The central dispatcher tags incoming telemetry from the Async Worker Pool with a specific agent identifier, routing standard output streams and tool-execution statuses to the correct sub-agent viewport. This enables a mission-control interface for managing multi-agent swarms.

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

impl WidgetRef for Card {
    fn render_ref(&self, area: Rect, buf: &mut Buffer);
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

### 6. Handlers Module

Event handling for user input.

#### Keys (keys.rs)

Keyboard event handling with hierarchical input maps:

```rust
pub struct KeyHandler {
    config: Keybindings,
    /// Current focus mode (determines which keymap is active)
    focus: ComponentId,
}

impl KeyHandler {
    pub fn new(config: Keybindings) -> Self;
    /// Routes keystrokes based on current mode
    /// - DashboardMode: global keybindings active
    /// - EditorMode(Normal): editor commands active
    /// - EditorMode(Insert): all keystrokes → editor directly
    /// - PromptMode: only prompt navigation keys
    pub fn handle(&self, key: KeyEvent, mode: &AppMode) -> Result<Action>;
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

    /// Editor actions
    EditorInsertMode,
    EditorNormalMode,
    EditorVisualMode,
    EditorSave,
    EditorFind,

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
    /// Editor mode keybindings (separate from global)
    pub editor: HashMap<KeyEvent, EditorAction>,
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut normal = HashMap::new();
        normal.insert(KeyEvent::Char('i'), Action::EditorInsertMode);
        normal.insert(KeyEvent::Char('q'), Action::Quit);
        // ... more default bindings

        Self {
            normal,
            insert: HashMap::new(),
            command: HashMap::new(),
            editor: HashMap::new(),
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
- Click to switch between sub-agent panes

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

### 7. Animation System (bus/animation.rs)

Demand-driven animation controller. **Never** runs a brute-force 60-FPS tick. The ticker is only active when at least one animation is registered.

```rust
pub struct AnimationController {
    /// Atomic counter of active animations
    active_count: Arc<AtomicU32>,
    /// The 16ms interval ticker (runs only when count > 0)
    ticker: Option<JoinHandle<()>>,
}

impl AnimationController {
    pub fn new(tx: mpsc::Sender<Action>) -> Self;
    /// Register an animation start → increments counter
    pub fn start_animation(&mut self);
    /// Animation resolved → decrements counter; if 0, suspends ticker
    pub fn end_animation(&mut self);
    /// Returns true if the 60-FPS loop is currently active
    pub fn is_active(&self) -> bool;
}
```

**Rendering trigger logic** (inspired by Yazi):
- **No animations active**: Event loop blocks on user input or async I/O. Redraw only on state change.
- **Animations active**: 16ms ticker runs. Uses time-delta progression for `tachyonfx` shader effects.
- **Synchronised Output**: DEC mode `?2026h` / `?2026l` escape sequences wrap every render cycle, instructing the terminal to buffer draw commands and present them in an atomic frame swap — eliminating visual tearing.

### 8. Utils Module

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

    /// Calculate visible width accounting for multi-cell Unicode
    pub fn unicode_display_width(text: &str) -> usize;
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

## Event Loop (app.rs)

The main event loop — structured around a biased `tokio::select!`:

```rust
impl App {
    pub async fn run(&mut self) -> Result<()> {
        // Initial draw
        self.terminal.draw(|f| self.draw(f))?;

        // Main loop — biased select for priority
        loop {
            // Render only when needed: state changed or animation active
            if self.state.is_dirty() || self.animator.is_active() {
                self.terminal.draw(|f| self.draw(f))?;
                self.state.clear_dirty();
            }

            // Biased select — hardware input > ticks > background data
            tokio::select! {
                biased;

                // Highest priority: hardware input
                Some(event) = self.input_rx.recv() => {
                    if !self.handle_input(event).await? {
                        break; // Quit
                    }
                }

                // Animation tick (only active when count > 0)
                Some(tick) = self.anim_rx.recv() => {
                    self.handle_tick(tick).await?;
                }

                // Gateway responses (lowest priority)
                Some(response) = self.gateway_rx.recv() => {
                    self.handle_gateway_response(response).await?;
                }

                // Fallback: yield if nothing ready
                else => {
                    tokio::task::yield_now().await;
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, action: Action) -> Result<bool> {
        match action {
            Action::Key(key) => {
                let mode = self.state.get_mode();
                let action = self.key_handler.handle(key, mode)?;
                self.dispatch(action)?;
            }
            Action::Resize(w, h) => {
                self.terminal.resize(w, h)?;
            }
            // ... mouse, paste
        }
        Ok(true)
    }

    fn dispatch(&mut self, action: Action) -> Result<()> {
        match action {
            Action::ComposerSubmit => {
                let text = self.composer.submit();
                if !text.is_empty() {
                    self.send_prompt(text)?;
                }
            }
            Action::EditorInsertMode => {
                self.state.set_mode(AppMode::EditorMode);
                // Dispatcher suspends global keybindings
            }
            Action::ActiveAnimationCount(count) => {
                // Demand-driven animation control
                if count == 0 {
                    self.animator.suspend();
                }
            }
            Action::Quit => {
                return Ok(false);
            }
            // ... handle other actions
        }
        Ok(true)
    }

    fn handle_gateway_response(&mut self, response: TuiResponse) -> Result<()> {
        match response {
            TuiResponse::MessageDelta { content, message_id } => {
                self.state.add_delta(content, message_id);
                self.state.mark_dirty();
            }
            TuiResponse::MessageComplete { message, message_id } => {
                self.state.complete_message(message, message_id);
            }
            TuiResponse::ApprovalRequest { request_id, prompt } => {
                self.state.set_mode(AppMode::PromptMode);
                self.show_approval_prompt(request_id, prompt);
            }
            TuiResponse::ToolStart { tool_name, tool_id } => {
                self.state.tool_started(tool_name, tool_id);
                self.animator.start_animation(); // activate spinner
            }
            TuiResponse::ToolComplete { tool_id, result } => {
                self.state.tool_completed(tool_id, result);
                self.animator.end_animation(); // may suspend ticker
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

The theme system allows customization of the TUI appearance. Uses **24-bit TrueColor** mode exclusively, leveraging `ratatui::style::palette::tailwind` for a cohesive, modern color scale. Backgrounds use deep SLATE hues; active agent components pulse with SKY or BLUE accents.

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

## Visual Design & Kinetic Feel

The TUI transcends traditional terminal aesthetic limitations with a modern, "post-terminal" visual language:

- **Typography & Color**: 24-bit TrueColor using `ratatui::style::palette::tailwind`. Deep SLATE backgrounds, SKY/BLUE accents for active agent components.
- **Layout**: `ratatui::layout::Flex` with `Flex::SpaceBetween` and `Flex::Center` for fluid, terminal-resize-adaptive layouts. Panels use `Block` with `Borders::ALL` and `BorderType::Rounded`.
- **Chat Bubbles** (oatmeal-inspired): Messages contained in styled bubbles. User messages right-aligned via `.right_aligned()`. Assistant responses left-aligned. Streaming tokens fade in using `tachyonfx` shaders (`fx::coalesce`, `fx::fade_to_fg`).
- **Transitions**: Panel transitions use `fx::slide_out` or `fx::dissolve(800)`. Sub-agent status indicators use `fx::evolve` to cycle through Unicode block progressions (`▁▂▃▄▅▆▇█`).
- **Focus Management**: Modal overlays (e.g., MCP tool confirmations) dim background panels via `CellFilter` + `fx::darken_fg`, drawing the user's eye to the active overlay without heavy CPU recalculations.
- **Editor Aesthetics**: Left gutter for line numbers (dimmed STONE color). Active line highlighted by shifting background color of the active row's `Rect`. Cursor snaps to correct Unicode character width, matching physical hardware cursor to logical Rope offset.

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
    i: editor_insert_mode
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
  rounded_borders: true
  chat_bubble_style: oatmeal

animation:
  fps: 60
  demand_driven: true
  use_synchronized_output: true

editor:
  auto_indent: true
  tab_width: 4
  history_size: 100
  rope_backed: true
  syntax_cache_size: 500
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

    #[error("Animation error: {0}")]
    AnimationError(String),
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
- Action bus channel backpressure
- Focus state machine transitions

### Integration Tests
- TUI ↔ Gateway message flow
- Session management
- Message history
- Animation ticker activate/suspend cycle
- Agent pane routing

### E2E Tests
- Full interaction scenarios
- Keybinding validation (modal modes)
- UI rendering validation
- Terminal tear-free output (DEC ?2026)

### Test Coverage Goals
- Protocol: 100%
- State: 100%
- Utils: 100%
- UI: 90% (some rendering tests are hard to automate)
- Handlers: 95%
- App: 85%
- Animation: 90%

## Performance Considerations

1. **Backpressure**: Bounded channels prevent memory exhaustion during LLM streaming or bulk memory retrieval.
2. **Demand-driven rendering**: 60-FPS ticker suspended when animations resolve — idle CPU consumption near zero.
3. **Synchronised Output**: DEC ?2026 eliminates visual tearing, enabling atomic frame swaps at high frame rates.
4. **Style-only animations**: Animations operate primarily on style modifiers (fading colours, altering alpha) rather than changing underlying text graphemes — accelerates the Ratatui diff process.
5. **Virtual viewports**: Editor and chat only process visible line subsets.
6. **Syntax caching**: Cached highlighted spans; only mutated lines invalidated.
7. **Message streaming**: Delta messages processed immediately for smooth typing effect.
8. **Memory management**: Limit message history, truncate old messages.
9. **Input handling**: Non-blocking event loop for responsive UI.

## Security Considerations

1. **Input validation**: Validate all gateway messages before processing
2. **Output sanitization**: Sanitize all text before display (remove harmful ANSI codes)
3. **Sensitive data**: Don't log sensitive user input
4. **Command execution**: Validate slash commands before executing
5. **Editor sandbox**: Prevent arbitrary file writes from LLM-generated editor commands

## Compatibility

- **Platforms**: Linux, macOS, Windows (via crossterm)
- **Terminals**: Any terminal supporting ANSI escape codes and 24-bit TrueColor
- **Synchronised Output**: Requires terminal that supports DEC mode ?2026 (kitty, Alacritty, WezTerm, iTerm2, Windows Terminal)
- **Minimum Rust version**: 1.75 (as specified in Cargo.toml)

## Future Enhancements

1. **Plugin system**: Allow custom UI components and themes
2. **Multi-pane layout**: Split view for chat + tools
3. **Command palette**: Fuzzy search for commands
4. **History search**: Search message history
5. **Custom keybindings**: User-defined keybindings
6. **Theme editor**: Interactive theme customization
7. **Performance metrics**: Show token usage, response times
8. **Hermes skill browser**: Browse and activate skills from within TUI
9. **Cron job monitor**: Visual dashboard for scheduled automations

## Migration Path

The Rust TUI will coexist with the existing TypeScript TUI initially:

1. **Phase 1**: Foundation — bounded channels, biased select, basic event loop (inspired by Yazi)
2. **Phase 2**: UI Engine — WidgetRef/StatefulWidget, ratatui-interact panes, synchronised output (DEC ?2026)
3. **Phase 3**: Conversational Interface — chat bubbles (oatmeal-style), streaming LLM display, slash commands
4. **Phase 4**: Embedded Modal Editor — Rope-backed edtui, syntax caching, input trapping
5. **Phase 5**: Multi-Agent — resizable split panes, memory explorer, telemetry routing

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
- Use `tokio::sync::mpsc::channel` (bounded) for event buses
- Use `tokio::sync::watch` for latest-value-only metrics
- Use `tokio::select!` with `biased` for priority scheduling

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
- `ropey` 1.6 - Rope data structure for editor
- `tachyonfx` 0.4 - Terminal shader effects
- `ratatui-interact` 0.3 - Managed split panes, tree views
- `edtui` 0.4 - Vim-inspired embedded editor widget
- `ratatui::widgets::Tree` - Tree widget for memory explorer

## Integration with Hermes

The Rust TUI will integrate with Hermes in the following ways:

1. **Gateway communication**: Via JSON-RPC over stdio to `tui_gateway/server.py`
2. **Protocol compatibility**: Must support all message types that the TypeScript TUI supports
3. **Configuration**: Reads from `~/.hermes/config.yaml` (shared with Hermes)
4. **Themes**: Can use Hermes skin system or its own theme system
5. **Memory explorer**: Fetches Hermes memory index from SQLite via background blocking thread
6. **Sub-agent telemetry**: Routes parallel sub-agent output to resizable split panes

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

### Phase 1: Foundation — Event Bus & Async Core (Inspired by Yazi)
- [ ] Project setup and Cargo.toml
- [ ] Bounded channel event bus (capacity 1024 + 64 input priority)
- [ ] Biased `tokio::select!` dispatcher
- [ ] stdio transport implementation
- [ ] Protocol message types
- [ ] Basic App struct and event loop
- [ ] Basic state management
- [ ] Simple chat display
- [ ] Basic text input

### Phase 2: UI Engine — StatefulWidget & Synchronised Output
- [ ] Refactor all widgets to `WidgetRef` / `StatefulWidget` (immutable render)
- [ ] Declarative focus state machine (`AppMode`)
- [ ] Demand-driven animation controller
- [ ] DEC ?2026 synchronised output for tear-free rendering
- [ ] ratatui-interact managed split panes
- [ ] Theme system (24-bit TrueColor, Tailwind palette)
- [ ] `tachyonfx` shader integration
- [ ] Mouse support

### Phase 3: Conversational Interface (Inspired by Oatmeal)
- [ ] Chat bubble widgets (user right-aligned, assistant left-aligned)
- [ ] Streaming LLM token display with `tachyonfx` fade-in
- [ ] Multi-line composer with history
- [ ] Message rendering with markdown + syntax highlighting
- [ ] Scrollable chat history
- [ ] Slash commands
- [ ] Completions (`tab` + `fuzzy-matcher`)
- [ ] Approval prompts

### Phase 4: Embedded Modal Editor (Inspired by Edtui / Helix)
- [ ] Rope-backed text buffer (`ropey` crate)
- [ ] edtui integration for Vim modal editing
- [ ] Virtual viewport (only process visible lines)
- [ ] Syntax highlighting cache (only invalidate mutated lines)
- [ ] Input trapping (suspend global keybindings in Insert/Visual modes)
- [ ] Native cursor synchronisation with Unicode width awareness
- [ ] Left gutter for line numbers

### Phase 5: Multi-Agent & Memory (Hermes-Specific)
- [ ] Memory explorer component (tree view, async SQLite fetch)
- [ ] Resizable split panes for sub-agent telemetry
- [ ] Agent status indicators with `tachyonfx` effects
- [ ] Tool execution display with progress bars
- [ ] Cron job monitor dashboard
- [ ] Integration with Hermes gateway
- [ ] End-to-end testing
- [ ] User testing
- [ ] Final polish

## Success Criteria

1. **Functional**: All core features work correctly
2. **Reliable**: No crashes, graceful error handling, OOM-safe bounded channels
3. **Responsive**: Smooth UI, no lag, biased polling prevents input starvation
4. **Compatible**: Works with all Hermes gateway features
5. **Tested**: Comprehensive test coverage including backpressure tests
6. **Documented**: Clear documentation and comments
7. **Maintainable**: Clean code, good architecture, immutable rendering

## References

- [ratatui documentation](https://ratatui.rs/)
- [crossterm documentation](https://docs.rs/crossterm)
- [Hermes TypeScript TUI](ui-tui/)
- [Hermes gateway protocol](tui_gateway/)
- [oh-my-pi repository](https://github.com/can1357/oh-my-pi)
- [Yazi — blazing-fast async terminal file manager](https://github.com/sxyazi/yazi) (inspiration for bounded channels, biased select)
- [Oatmeal — TUI chat application for LLMs](https://github.com/oatmeal) (inspiration for chat bubbles, conversational interface)
- [Edtui — Vim-inspired editor widget for Ratatui](https://github.com/edtui) (embedded modal editor)
- [Helix — modal terminal editor with Rope backing](https://helix-editor.com/) (Rope architecture reference)
- [Tachyonfx — terminal shader effects library](https://github.com/tachyonfx) (animations, transitions, fade effects)
- [Gemini Architectural Critique — Asynchronous Event-Driven Component TUI for Hermes Agent](https://gemini.google.com/share/15062374214a) (source of the technical mandates incorporated in this document)
