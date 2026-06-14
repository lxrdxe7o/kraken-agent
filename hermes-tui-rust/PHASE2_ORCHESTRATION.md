# Phase 2 Orchestration - Rust TUI with oh-my-pi Features

## 🎯 Project Goal

Build a production-ready Rust TUI for Hermes Agent that incorporates the best features from [oh-my-pi](https://github.com/can1357/oh-my-pi), specifically:
- **Hashline edits** - Content-hash anchored patch system
- **Tool cards** - Visual tool call cards and outputs  
- **Subagent UI** - Visual management of parallel subagents
- **LSP/debugger UI** - Code intelligence visualization

## 📦 User Requirements (From Context)

- **Type**: Alternative TUI option (--tui-rust flag)
- **Platform**: Linux first
- **Framework**: Ratatui + Crossterm
- **Features**: Hashline edits, Tool cards, Subagent UI, LSP/debugger UI
- **Quality**: Atomic commits, TDD, don't break Hermes functionality

## 🚀 Subagent Orchestration Plan

### Subagent 1: Protocol & Foundation Team (CRITICAL) — ✅ COMPLETED
**Lead**: Protocol compatibility with Hermes gateway
**Priority**: CRITICAL - Must be done first
**Estimated Duration**: 2-3 days

#### Tasks:
1. **Complete StdioTransport implementation**
   - Background thread for non-blocking stdin reads
   - JSON-RPC message parsing with serde_json
   - Error handling for broken pipes
   - Connection state management

2. **Complete GatewayClient**
   - Request/response lifecycle management
   - Request ID tracking (AtomicU64)
   - Message queue with mpsc channel
   - Reconnection logic

3. **Protocol Compatibility Verification**
   - Test all message types against TypeScript TUI
   - Verify serialization/deserialization matches exactly
   - Integration test with actual tui_gateway

4. **Error Types**
   - Comprehensive TuiError enum with thiserror
   - Conversions from std::io::Error, serde_json::Error
   - Contextual error messages

#### Deliverables:
- `src/protocol/transport.rs` - Complete implementation
- `src/protocol/client.rs` - Complete implementation  
- `src/error.rs` - Comprehensive error handling
- `tests/protocol/*` - All protocol tests passing

#### Dependencies:
- None (can start immediately)

#### Commit Points:
- `[PROTOCOL] transport: Background reader thread for stdin`
- `[PROTOCOL] client: Request/response lifecycle with IDs`
- `[PROTOCOL] error: Comprehensive TuiError types`
- `[PROTOCOL] tests: Protocol serialization and transport tests`

---

### Subagent 2: State Management Team (CRITICAL) — ✅ COMPLETED
**Lead**: Application state and session management
**Priority**: CRITICAL - Depends on Subagent 1
**Estimated Duration**: 2-3 days

#### Tasks:
1. **Message History**
   - Message struct with role, content, timestamp, metadata
   - MessageHistory with max message limit
   - Scrollable history with efficient storage

2. **Session Management**
   - Session struct with messages, metadata
   - SessionManager with create/switch/list/clear
   - Session persistence (optional for Phase 2)

3. **App State**
   - Central AppState struct
   - Theme configuration
   - Keybindings configuration
   - Display settings

4. **Configuration Loading**
   - Load from ~/.hermes/config.yaml
   - Merge with defaults
   - Theme loading from files

#### Deliverables:
- `src/state/messages.rs` - Complete implementation
- `src/state/session.rs` - Complete implementation
- `src/state/config.rs` - Theme and config loading
- `src/state/mod.rs` - Module consolidation
- `tests/state/*` - All state tests passing

#### Dependencies:
- Subagent 1 (Protocol types)

#### Commit Points:
- `[STATE] messages: Message struct and history management`
- `[STATE] session: Session create/switch/list operations`
- `[STATE] config: TUI configuration and theme loading`
- `[STATE] app_state: Central application state`

---

### Subagent 3: UI Core Team (HIGH) — ✅ COMPLETED
**Lead**: Core UI components
**Priority**: HIGH - Depends on Subagent 2
**Estimated Duration**: 3-4 days

#### Tasks:
1. **Theme System**
   - ThemeConfig with colors and styles
   - Built-in themes (default, dark, light, solarized, dracula)
   - Custom theme loading from YAML
   - Color conversion to ratatui Color

2. **Chat Widget**
   - Scrollable message list
   - Message formatting (role indicators, timestamps)
   - Code block detection
   - Markdown basic formatting

3. **Composer Widget**
   - Multi-line text input with tui-textarea
   - Keyboard navigation (arrows, home, end)
   - Input history (up/down)
   - Syntax highlighting as-you-type

4. **Toolbar Widget**
   - Status display (connected, session, model)
   - Input mode indicator
   - Progress indicators

#### Deliverables:
- `src/ui/theme.rs` - Theme system
- `src/ui/chat.rs` - Chat transcript
- `src/ui/composer.rs` - Text input
- `src/ui/toolbar.rs` - Status bar
- `src/ui/mod.rs` - Module consolidation
- `tests/ui/*` - UI component tests

#### Dependencies:
- Subagent 2 (State management)

#### Commit Points:
- `[UI] theme: Color scheme and style system`
- `[UI] chat: Scrollable message display`
- `[UI] composer: Multi-line input with history`
- `[UI] toolbar: Status and mode display`

---

### Subagent 4: Advanced UI Team — oh-my-pi Features (HIGH) — ✅ COMPLETED
**Lead**: Hashline edits, Tool cards, Subagent UI
**Priority**: HIGH - Can start in parallel with Subagent 3

#### Tasks (Hashline Edits):
1. **Content Hash Anchors**
   - Extract hash anchors from agent messages
   - Hash-based message identification
   - Anchor validation and stale detection

2. **Hashline Patch System**
   - Display messages with hash anchors
   - Visual indicator for anchored content
   - Stale anchor warnings

3. **Edit Preview**
   - Show proposed changes before application
   - Diff visualization
   - Accept/reject UI

#### Tasks (Tool Cards):
4. **Tool Card Component**
   - Visual representation of tool calls
   - Status indicators (running, completed, error)
   - Progress bars for long-running tools
   - Output display with syntax highlighting

5. **Tool Card Layout**
   - Compact card design
   - Expandable details
   - Multiple cards in chat flow

#### Tasks (Subagent UI):
6. **Subagent Status Display**
   - Visual representation of active subagents
   - Progress tracking per subagent
   - Task assignment display

7. **Subagent Management**
   - Spawn subagent UI
   - Status updates in real-time
   - Result aggregation display

8. **Parallel Subagent Visualization**
   - Multiple subagents in parallel
   - Progress bars for each
   - Coordinated completion display

#### Deliverables:
- `src/ui/cards.rs` - Tool card and UI card components
- `src/ui/hashline.rs` - Hashline edit display
- `src/ui/subagents.rs` - Subagent visualization
- `tests/ui/advanced/*` - Advanced UI tests

#### Dependencies:
- Subagent 2 (State for tracking subagents)
- Subagent 3 (Core UI components)

#### Commit Points:
- `[FEATURE] hashline: Content hash anchor system`
- `[FEATURE] tool_cards: Visual tool call representation`
- `[FEATURE] subagent_ui: Parallel subagent management UI`

---

### Subagent 5: Handlers Team (MEDIUM) — ✅ COMPLETED
**Lead**: User input handling
**Priority**: MEDIUM - Depends on Subagent 3

#### Tasks:
1. **Key Handler**
   - Keybinding configuration
   - Mode-specific key mappings (Normal, Insert, Command)
   - Default keybindings for common actions

2. **Mouse Handler**
   - Click handling for UI elements
   - Scroll wheel for chat
   - Drag and drop support (future)

3. **Input Handler**
   - Text input processing
   - Completion triggering
   - Command parsing

4. **Action System**
   - Action enum with all possible actions
   - Action dispatch to appropriate handlers
   - Undo/redo support

#### Deliverables:
- `src/handlers/keys.rs` - Complete implementation
- `src/handlers/mouse.rs` - Complete implementation
- `src/handlers/input.rs` - Complete implementation
- `src/handlers/mod.rs` - Module consolidation
- `tests/handlers/*` - Handler tests

#### Dependencies:
- Subagent 3 (UI components)

#### Commit Points:
- `[HANDLERS] keys: Keybinding system with default mappings`
- `[HANDLERS] mouse: Click and scroll handling`
- `[HANDLERS] input: Text input and completion handling`

---

### Subagent 6: Integration Team (MEDIUM) — ⏳ IN PROGRESS (primary remaining work)
**Lead**: Connect all components to gateway
**Priority**: MEDIUM - Depends on Subagent 1, 2, 3, 5
   - Handshake and capability negotiation
   - Auto-reconnect logic

3. **Message Flow**
   - Prompt submission
   - Message delta streaming
   - Message complete handling
   - Session management integration

4. **Approval Flow**
   - Approval request display
   - User response handling
   - Tool call gating

5. **Completion Flow**
   - Slash command completions
   - Path completions
   - Display in composer

#### Deliverables:
- `src/app.rs` - Complete event loop and integration
- Gateway connection tests
- End-to-end message flow tests

#### Dependencies:
- Subagent 1 (Protocol)
- Subagent 2 (State)
- Subagent 3 (UI)
- Subagent 5 (Handlers)

#### Commit Points:
- `[INTEGRATION] event_loop: Main event loop with all handlers`
- `[INTEGRATION] gateway: stdio connection and message flow`
- `[INTEGRATION] sessions: Session create/resume/list flow`
- `[INTEGRATION] approvals: Approval prompt handling`
- `[INTEGRATION] completions: Slash and path completions`

---
### Subagent 7: LSP/Debugger UI Team — ⏳ NOT STARTED (planned for later phase)
**Lead**: Code intelligence visualization

#### Tasks:
1. **LSP Integration**
   - LSP message types for protocol
   - Diagnostic display in chat
   - Code action suggestions

2. **LSP UI Components**
   - Diagnostic cards (errors, warnings)
   - Symbol navigation
   - Hover information display

3. **Debugger UI**
   - Debug session visualization
   - Breakpoint display
   - Variable inspection

4. **Code Lens**
   - Inline code intelligence
   - Reference counting
   - Definition navigation

#### Deliverables:
- `src/ui/lsp.rs` - LSP visualization components
- `src/ui/debugger.rs` - Debugger UI
- Protocol extensions for LSP messages
- `tests/ui/lsp/*` - LSP UI tests

#### Dependencies:
- Subagent 3 (Core UI)
- Subagent 4 (Advanced UI patterns)

#### Commit Points:
- `[FEATURE] lsp: Diagnostic and code intelligence display`
- `[FEATURE] debugger: Debug session visualization`

---

### Subagent 8: QA & Polish Team — ⏳ NOT STARTED (after integration) 
**Lead**: Testing, quality assurance, polish
**Estimated Duration**: Ongoing

#### Tasks:
1. **Unit Tests**
   - 100% coverage for protocol module
   - 100% coverage for state module
   - 90%+ coverage for all other modules

2. **Integration Tests**
   - TUI ↔ Gateway message flow
   - Session lifecycle
   - Message streaming

3. **E2E Tests**
   - Full interaction scenarios
   - Keybinding validation
   - UI rendering validation

4. **Quality Gates**
   - `cargo test` - All tests pass
   - `cargo clippy` - No warnings
   - `cargo fmt` - No changes
   - `cargo doc` - No warnings
   - `cargo build --release` - Success

5. **Performance**
   - Rendering benchmarks
   - Message processing benchmarks
   - Memory usage profiling

6. **Documentation**
   - All public items documented
   - README with usage instructions
   - Example configurations

#### Deliverables:
- `tests/unit/*.rs` - Comprehensive unit tests
- `tests/integration/*.rs` - Integration tests
- `tests/e2e/*.rs` - End-to-end tests
- `benches/*.rs` - Performance benchmarks
- Complete documentation

#### Dependencies:
- All other subagents (tests each phase as it completes)

#### Commit Points:
- `[TESTING] unit: 100% coverage for protocol and state`
- `[TESTING] integration: Message flow and session tests`
- `[TESTING] e2e: Full interaction scenarios`
- `[POLISH] quality: clippy clean, fmt clean, doc clean`
- `[POLISH] performance: Rendering and processing benchmarks`

---

## 📅 Phase 2 Timeline (8-10 Weeks)

| Week | Subagent | Focus | Dependencies |
|------|----------|-------|--------------|
| 1 | 1 | Protocol & Transport | None |
| 1-2 | 2 | State Management | 1 |
| 2-3 | 3 | UI Core | 2 |
| 2-3 | 4 | Advanced UI (oh-my-pi features) | 2 |
| 3-4 | 5 | Handlers | 3 |
| 4-5 | 6 | Integration | 1,2,3,5 |
| 5-6 | 7 | LSP/Debugger UI | 3,4 |
| 6-8 | 8 | QA & Polish | 6 |

**Note**: Weeks overlap intentionally - subagents can work in parallel once dependencies are met.

---

## 🎯 Success Criteria for Phase 2

### Functional Requirements
- [ ] All core features work correctly
- [ ] Hashline edits implemented
- [ ] Tool cards displayed
- [ ] Subagent UI functional
- [ ] LSP/debugger visualization working
- [ ] Compatible with all Hermes gateway features

### Non-Functional Requirements
- [ ] No breaking changes to existing Hermes functionality
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt
- [ ] All public items documented
- [ ] Atomic commits at each step
- [ ] Can rollback any commit

### Performance Targets
- [ ] UI rendering < 16ms per frame (60fps)
- [ ] Message processing < 100ms per message
- [ ] Memory usage < 100MB for typical session
- [ ] Startup time < 500ms

---

## 🚨 Risk Mitigation

### High Risk Items
1. **Protocol Compatibility**: Must match TypeScript TUI exactly
   - Mitigation: Test with actual gateway, compare message formats
   - Subagent: 1, 6

2. **Performance**: Slow rendering or message processing
   - Mitigation: Benchmark early, optimize incrementally
   - Subagent: 8

3. **Memory Usage**: Unbounded message history
   - Mitigation: Implement limits from the start
   - Subagent: 2, 8

4. **Breaking Hermes**: Changes affect existing functionality
   - Mitigation: Isolate Rust TUI, no changes to core files
   - Subagent: All

### Medium Risk Items
1. **Cross-platform Compatibility**: Windows/Linux/macOS differences
   - Mitigation: Test on all platforms, use crossterm
   - Subagent: 3, 6

2. **Terminal Compatibility**: Different terminal capabilities
   - Mitigation: Graceful degradation, feature detection
   - Subagent: 3

3. **Dependency Updates**: Breaking changes in dependencies
   - Mitigation: Pin dependencies, test with updates
   - Subagent: 8

---

## 📊 Monitoring & Reporting

### Daily Standups (Async)
Each subagent provides daily status:
- Tasks completed
- Tasks in progress
- Blockers
- Estimated completion

### Weekly Reviews
Full team sync every Friday:
- Demo of completed features
- Review of commit history
- Planning for next week
- Risk assessment

### Commit Metrics
Track per subagent:
- Commits per day
- Lines of code added/removed
- Tests added/passed
- Clippy warnings fixed

---

## 🎉 Phase 2 Completion Criteria

All of the following must be true:

1. **Core Features**: All Phase 1 + 2 tasks complete
2. **oh-my-pi Features**: Hashline, Tool cards, Subagent UI, LSP/Debugger UI implemented
3. **Tests**: All tests pass, coverage targets met
4. **Quality**: No clippy warnings, fmt clean, doc complete
5. **Integration**: Works with Hermes gateway
6. **No Regressions**: Existing Hermes functionality unaffected
7. **Documentation**: All public items documented, README complete
8. **Performance**: Meets all performance targets

---

## 📚 References

- [ARCHITECTURE.md](ARCHITECTURE.md) - Full architecture document
- [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) - Detailed 8-phase plan
- [Hermes TypeScript TUI](https://github.com/NousResearch/hermes-agent/tree/main/ui-tui) - Reference implementation
- [Hermes Gateway Protocol](https://github.com/NousResearch/hermes-agent/tree/main/tui_gateway) - Protocol specification
- [oh-my-pi Repository](https://github.com/can1357/oh-my-pi) - Feature inspiration
- [ratatui Documentation](https://ratatui.rs/) - TUI framework
- [crossterm Documentation](https://docs.rs/crossterm) - Terminal I/O

---

## 🔄 Rollback Strategy

Each commit is atomic and can be rolled back independently:

```bash
# Rollback a specific commit
git revert <commit-hash>

# Rollback entire phase
git revert --no-commit <first-commit>^..<last-commit>
git commit -m "Revert Phase X"

# Verify rollback
cargo test
cargo clippy
cargo build
```

All subagents must ensure their commits are:
1. Small and focused
2. Complete (all related files)
3. Tested (all tests pass)
4. Documented (clear commit message)

---

*Document Version: 1.1*
*Last Updated: 2026-06-14 (updated - subagents 1-5 completed)*
*Status: Subagents 1-5 completed. Subagent 6 (Integration) is primary remaining work.*
