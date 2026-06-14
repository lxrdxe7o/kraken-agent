# Implementation Plan - Hermes TUI Rust

## Overview

This document provides a detailed implementation plan for building the Rust TUI for Hermes Agent, organized into phases with atomic commits at each step.

## Phase Structure

Each phase consists of:
1. **Tasks**: Individual work items
2. **Deliverables**: Files to create/modify
3. **Tests**: Test files to create
4. **Commit Points**: Atomic commits with clear messages

## Phase 1: Foundation (Priority: CRITICAL)

### Goal: Establish the basic project structure and core infrastructure

### Tasks

#### Task 1.1: Project Structure Setup
- **Deliverables**: 
  - `hermes-tui-rust/src/lib.rs` - Library exports
  - Directory structure for all modules
- **Tests**: None (structure only)
- **Commit**: `[FOUNDATION] Project structure - create all module directories`

#### Task 1.2: Error Types
- **Deliverables**:
  - `hermes-tui-rust/src/error.rs` - Custom error types
- **Tests**: 
  - `tests/error_test.rs` - Error conversion tests
- **Commit**: `[FOUNDATION] Error handling - define TuiError and conversions`

#### Task 1.3: Protocol Types (CORE - Must match TypeScript TUI exactly)
- **Deliverables**:
  - `hermes-tui-rust/src/protocol/types.rs` - All JSON-RPC message types
- **Tests**:
  - `tests/protocol/types_test.rs` - Serialization/deserialization tests
- **Commit**: `[FOUNDATION] Protocol types - JSON-RPC message definitions`

#### Task 1.4: Transport Layer (CORE)
- **Deliverables**:
  - `hermes-tui-rust/src/protocol/transport.rs` - stdio transport with background reader
- **Tests**:
  - `tests/protocol/transport_test.rs` - Read/write/thread tests
- **Commit**: `[FOUNDATION] Transport layer - stdio JSON-RPC transport`

#### Task 1.5: Protocol Client (CORE)
- **Deliverables**:
  - `hermes-tui-rust/src/protocol/client.rs` - High-level client
  - `hermes-tui-rust/src/protocol/mod.rs` - Module exports
- **Tests**:
  - `tests/protocol/client_test.rs` - Client request/response tests
- **Commit**: `[FOUNDATION] Protocol client - JSON-RPC client implementation`

#### Task 1.6: Basic App Skeleton
- **Deliverables**:
  - `hermes-tui-rust/src/app.rs` - App struct with event loop skeleton
- **Tests**:
  - `tests/app_test.rs` - App creation test
- **Commit**: `[FOUNDATION] App skeleton - basic structure with event loop`

### Acceptance Criteria
- [ ] Protocol types match TypeScript TUI exactly
- [ ] Transport can read/write JSON-RPC messages
- [ ] Client can send/receive messages
- [ ] App can be created and run (even if it does nothing)
- [ ] All tests pass

### Dependencies
- All Cargo.toml dependencies are already specified

---

## Phase 2: State Management (Priority: CRITICAL)

### Goal: Implement application state and message management

### Tasks

#### Task 2.1: Message Types
- **Deliverables**:
  - `hermes-tui-rust/src/state/messages.rs` - Message structure and history
- **Tests**:
  - `tests/state/messages_test.rs` - Message operations tests
- **Commit**: `[STATE] Message types - role, content, timestamp, metadata`

#### Task 2.2: Session Management
- **Deliverables**:
  - `hermes-tui-rust/src/state/session.rs` - Session struct and manager
- **Tests**:
  - `tests/state/session_test.rs` - Session operations tests
- **Commit**: `[STATE] Session management - create, switch, list sessions`

#### Task 2.3: App State
- **Deliverables**:
  - `hermes-tui-rust/src/state/mod.rs` - AppState struct
  - Update `app.rs` to use AppState
- **Tests**:
  - `tests/state/app_state_test.rs` - State update tests
- **Commit**: `[STATE] App state - central state management`

#### Task 2.4: Configuration
- **Deliverables**:
  - `hermes-tui-rust/src/state/config.rs` - TUI configuration
  - Config loading from `~/.hermes/config.yaml`
- **Tests**:
  - `tests/state/config_test.rs` - Config loading tests
- **Commit**: `[STATE] Configuration - theme, keybindings, display settings`

### Acceptance Criteria
- [ ] Messages can be added, retrieved, iterated
- [ ] Sessions can be created, switched, listed
- [ ] App state manages all components
- [ ] Config can be loaded from file
- [ ] All tests pass

---

## Phase 3: UI Components (Priority: HIGH)

### Goal: Implement all UI components using ratatui

### Tasks

#### Task 3.1: Theme System
- **Deliverables**:
  - `hermes-tui-rust/src/state/config.rs` (extend) - Theme types
  - `hermes-tui-rust/src/ui/theme.rs` - Theme definitions and colors
- **Tests**:
  - `tests/ui/theme_test.rs` - Theme application tests
- **Commit**: `[UI] Theme system - built-in themes and custom theme support`

#### Task 3.2: Chat Widget
- **Deliverables**:
  - `hermes-tui-rust/src/ui/chat.rs` - Chat transcript display
- **Tests**:
  - `tests/ui/chat_test.rs` - Chat rendering tests
- **Commit**: `[UI] Chat widget - message display with scrolling`

#### Task 3.3: Composer Widget
- **Deliverables**:
  - `hermes-tui-rust/src/ui/composer.rs` - Multi-line text input
- **Tests**:
  - `tests/ui/composer_test.rs` - Input handling tests
- **Commit**: `[UI] Composer widget - multi-line text input`

#### Task 3.4: Toolbar Widget
- **Deliverables**:
  - `hermes-tui-rust/src/ui/toolbar.rs` - Status toolbar
- **Tests**:
  - `tests/ui/toolbar_test.rs` - Toolbar rendering tests
- **Commit**: `[UI] Toolbar widget - status and mode display`

#### Task 3.5: Prompt Widget
- **Deliverables**:
  - `hermes-tui-rust/src/ui/prompts.rs` - User prompts
- **Tests**:
  - `tests/ui/prompts_test.rs` - Prompt handling tests
- **Commit**: `[UI] Prompt widget - confirmation, choice, text prompts`

#### Task 3.6: Cards Widget
- **Deliverables**:
  - `hermes-tui-rust/src/ui/cards.rs` - UI card components
- **Tests**:
  - `tests/ui/cards_test.rs` - Card rendering tests
- **Commit**: `[UI] Cards widget - tool cards, error cards, loading cards`

#### Task 3.7: UI Module
- **Deliverables**:
  - `hermes-tui-rust/src/ui/mod.rs` - UI module exports
- **Tests**: None
- **Commit**: `[UI] Module exports - consolidate UI module`

### Acceptance Criteria
- [ ] All UI components render correctly
- [ ] Chat widget supports scrolling
- [ ] Composer widget supports multi-line editing
- [ ] Toolbar shows status and mode
- [ ] Prompt widgets work for all prompt types
- [ ] Cards display various content types
- [ ] All tests pass

---

## Phase 4: Event Handling (Priority: HIGH)

### Goal: Implement user input handling

### Tasks

#### Task 4.1: Key Handler
- **Deliverables**:
  - `hermes-tui-rust/src/handlers/keys.rs` - Keyboard event handling
- **Tests**:
  - `tests/handlers/keys_test.rs` - Key handling tests
- **Commit**: `[HANDLERS] Key handler - keyboard shortcuts and actions`

#### Task 4.2: Mouse Handler
- **Deliverables**:
  - `hermes-tui-rust/src/handlers/mouse.rs` - Mouse event handling
- **Tests**:
  - `tests/handlers/mouse_test.rs` - Mouse handling tests
- **Commit**: `[HANDLERS] Mouse handler - click and scroll handling`

#### Task 4.3: Input Handler
- **Deliverables**:
  - `hermes-tui-rust/src/handlers/input.rs` - Text input handling
- **Tests**:
  - `tests/handlers/input_test.rs` - Input handling tests
- **Commit**: `[HANDLERS] Input handler - text input management`

#### Task 4.4: Handlers Module
- **Deliverables**:
  - `hermes-tui-rust/src/handlers/mod.rs` - Handler module exports
- **Tests**: None
- **Commit**: `[HANDLERS] Module exports - consolidate handlers module`

### Acceptance Criteria
- [ ] All keybindings work correctly
- [ ] Mouse clicks and scrolls are handled
- [ ] Text input is processed correctly
- [ ] All tests pass

---

## Phase 5: Integration (Priority: HIGH)

### Goal: Integrate all components and connect to Hermes gateway

### Tasks

#### Task 5.1: Update App Event Loop
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Full event loop with handlers
- **Tests**:
  - `tests/app_test.rs` (extend) - Event handling tests
- **Commit**: `[INTEGRATION] Event loop - connect handlers and UI`

#### Task 5.2: Gateway Connection
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Gateway connection logic
  - Update `hermes-tui-rust/src/main.rs` - Initialization
- **Tests**:
  - `tests/integration/gateway_test.rs` - Connection tests
- **Commit**: `[INTEGRATION] Gateway connection - stdio to tui_gateway`

#### Task 5.3: Message Flow
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Message handling
- **Tests**:
  - `tests/integration/message_flow_test.rs` - Message flow tests
- **Commit**: `[INTEGRATION] Message flow - prompt submit, delta, complete`

#### Task 5.4: Session Management Integration
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Session operations
- **Tests**:
  - `tests/integration/session_test.rs` - Session tests
- **Commit**: `[INTEGRATION] Session management - list, resume, new`

### Acceptance Criteria
- [ ] App can connect to gateway
- [ ] Messages flow correctly between TUI and gateway
- [ ] Sessions can be managed
- [ ] All tests pass

---

## Phase 6: Enhanced Features (Priority: MEDIUM)

### Goal: Add advanced features for better UX

### Tasks

#### Task 6.1: Syntax Highlighting
- **Deliverables**:
  - `hermes-tui-rust/src/utils/text.rs` - Code extraction and highlighting
- **Tests**:
  - `tests/utils/text_test.rs` - Syntax highlighting tests
- **Commit**: `[FEATURES] Syntax highlighting - code blocks with syntect`

#### Task 6.2: Markdown Formatting
- **Deliverables**:
  - Update `hermes-tui-rust/src/utils/text.rs` - Markdown to text formatting
- **Tests**:
  - `tests/utils/text_test.rs` (extend) - Markdown formatting tests
- **Commit**: `[FEATURES] Markdown formatting - headers, lists, bold, etc.`

#### Task 6.3: Command Palette
- **Deliverables**:
  - `hermes-tui-rust/src/ui/command_palette.rs` - Command palette widget
- **Tests**:
  - `tests/ui/command_palette_test.rs` - Command palette tests
- **Commit**: `[FEATURES] Command palette - fuzzy search for commands`

#### Task 6.4: Approval Prompts
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Approval handling
  - Update `hermes-tui-rust/src/ui/prompts.rs` - Approval prompt
- **Tests**:
  - `tests/integration/approval_test.rs` - Approval flow tests
- **Commit**: `[FEATURES] Approval prompts - user approval for tool calls`

#### Task 6.5: Completions
- **Deliverables**:
  - Update `hermes-tui-rust/src/app.rs` - Completion requests
  - Update `hermes-tui-rust/src/ui/composer.rs` - Completion display
- **Tests**:
  - `tests/integration/completion_test.rs` - Completion tests
- **Commit**: `[FEATURES] Completions - slash and path completions`

### Acceptance Criteria
- [ ] Code blocks are syntax highlighted
- [ ] Markdown is formatted correctly
- [ ] Command palette works with fuzzy search
- [ ] Approval prompts display and respond correctly
- [ ] Completions work for slash commands and paths
- [ ] All tests pass

---

## Phase 7: Testing & Quality (Priority: HIGH)

### Goal: Comprehensive testing and quality assurance

### Tasks

#### Task 7.1: Unit Test Coverage
- **Deliverables**:
  - All modules have comprehensive unit tests
- **Tests**:
  - All `tests/*.rs` files
- **Commit**: `[TESTING] Unit tests - 100% coverage for protocol, state, utils`

#### Task 7.2: Integration Tests
- **Deliverables**:
  - Integration test suite
- **Tests**:
  - `tests/integration/*.rs` files
- **Commit**: `[TESTING] Integration tests - TUI ↔ Gateway communication`

#### Task 7.3: Error Handling Tests
- **Deliverables**:
  - Error scenario tests
- **Tests**:
  - `tests/error/*.rs` files
- **Commit**: `[TESTING] Error tests - connection, parsing, validation`

#### Task 7.4: Performance Tests
- **Deliverables**:
  - Performance benchmark tests
- **Tests**:
  - `benches/*.rs` files
- **Commit**: `[TESTING] Performance tests - rendering, message processing`

#### Task 7.5: Documentation
- **Deliverables**:
  - All public items documented
  - README.md with usage instructions
- **Tests**:
  - `cargo doc` builds without warnings
- **Commit**: `[TESTING] Documentation - all items documented, README`

### Acceptance Criteria
- [ ] Unit test coverage: Protocol 100%, State 100%, Utils 100%
- [ ] Integration tests pass
- [ ] Error scenarios handled
- [ ] Performance benchmarks established
- [ ] All documentation complete

---

## Phase 8: Polish & Optimization (Priority: MEDIUM)

### Goal: Final polish and optimization

### Tasks

#### Task 8.1: UI Polish
- **Deliverables**:
  - Smooth scrolling
  - Better theme defaults
  - Responsive layout
- **Tests**:
  - Visual inspection
- **Commit**: `[POLISH] UI polish - smooth scrolling, themes, layout`

#### Task 8.2: Performance Optimization
- **Deliverables**:
  - Caching for syntax highlighting
  - Optimized rendering
  - Memory management
- **Tests**:
  - Performance benchmarks
- **Commit**: `[POLISH] Performance - caching, rendering optimization`

#### Task 8.3: Error Recovery
- **Deliverables**:
  - Auto-reconnect logic
  - Error display improvements
- **Tests**:
  - Error recovery tests
- **Commit**: `[POLISH] Error recovery - auto-reconnect, error display`

#### Task 8.4: Accessibility
- **Deliverables**:
  - Better keyboard navigation
  - Screen reader support
- **Tests**:
  - Accessibility tests
- **Commit**: `[POLISH] Accessibility - keyboard nav, screen reader`

#### Task 8.5: Final Touches
- **Deliverables**:
  - Final bug fixes
  - Version bump
  - Changelog
- **Tests**:
  - Full test suite
- **Commit**: `[POLISH] Final touches - version bump, changelog`

### Acceptance Criteria
- [ ] UI is polished and responsive
- [ ] Performance meets targets
- [ ] Error recovery works
- [ ] Accessible
- [ ] All tests pass

---

## Subagent Tasks

The following tasks can be delegated to subagents for parallel development:

### Subagent 1: Protocol Team
- **Focus**: Phase 1 (Protocol layer)
- **Tasks**: 1.1-1.6
- **Deliverables**: Protocol module complete with tests
- **Dependencies**: None (can start immediately)

### Subagent 2: State Team
- **Focus**: Phase 2 (State management)
- **Tasks**: 2.1-2.4
- **Deliverables**: State module complete with tests
- **Dependencies**: Phase 1 (Protocol types)

### Subagent 3: UI Team
- **Focus**: Phase 3 (UI components)
- **Tasks**: 3.1-3.7
- **Deliverables**: UI module complete with tests
- **Dependencies**: Phase 2 (State)

### Subagent 4: Handlers Team
- **Focus**: Phase 4 (Event handling)
- **Tasks**: 4.1-4.4
- **Deliverables**: Handlers module complete with tests
- **Dependencies**: Phase 3 (UI components)

### Subagent 5: Integration Team
- **Focus**: Phase 5 (Integration)
- **Tasks**: 5.1-5.4
- **Deliverables**: Full integration with gateway
- **Dependencies**: Phase 4 (Handlers)

### Subagent 6: QA Team
- **Focus**: Phase 7-8 (Testing & Polish)
- **Tasks**: 7.1-8.5
- **Deliverables**: Comprehensive testing and polish
- **Dependencies**: Phase 5 (Integration)

---

## Commit Strategy

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
- `FOUNDATION` - Phase 1 tasks
- `STATE` - Phase 2 tasks
- `UI` - Phase 3 tasks
- `HANDLERS` - Phase 4 tasks
- `INTEGRATION` - Phase 5 tasks
- `FEATURES` - Phase 6 tasks
- `TESTING` - Phase 7 tasks
- `POLISH` - Phase 8 tasks
- `FIX` - Bug fixes
- `DOCS` - Documentation only
- `REFCTOR` - Code refactoring
- `CHORE` - Maintenance tasks

### Rollback Strategy
Each phase has a clear commit point. If something breaks:
1. Identify the failing commit
2. `git revert <commit>` to roll back
3. Fix the issue
4. Re-commit

---

## Testing Strategy

### Test Pyramid
```
                    /\
                   /  \  E2E Tests (Few, Integration)
                  /----\
                 /      \  Integration Tests (Medium)
                /--------\
               /          \  Unit Tests (Many)
              /------------\
```

### Test Files
- `tests/unit/*.rs` - Unit tests (fast, isolated)
- `tests/integration/*.rs` - Integration tests (slower, multi-component)
- `tests/e2e/*.rs` - End-to-end tests (slowest, full flow)

### Test Coverage Targets
| Module | Coverage Target |
|--------|----------------|
| protocol | 100% |
| state | 100% |
| utils | 100% |
| ui | 90% |
| handlers | 95% |
| app | 85% |

### Running Tests
```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration

# With coverage
cargo tarpaulin

# Specific test
cargo test test_function_name
```

---

## Quality Gates

Before merging to main:
1. **All tests pass**: `cargo test` succeeds
2. **Clippy clean**: `cargo clippy` no warnings
3. **Format clean**: `cargo fmt` no changes
4. **Documentation**: `cargo doc` builds without warnings
5. **Build**: `cargo build --release` succeeds
6. **Test coverage**: Meets targets for changed modules

---

## Risk Mitigation

### High Risk Items
1. **Protocol compatibility**: Must match TypeScript TUI exactly
   - Mitigation: Test with actual gateway, compare message formats
2. **Performance**: Slow rendering or message processing
   - Mitigation: Benchmark early, optimize incrementally
3. **Memory usage**: Unbounded message history
   - Mitigation: Implement limits from the start
4. **Error handling**: Unhandled errors causing crashes
   - Mitigation: Comprehensive error types, tests for error paths

### Medium Risk Items
1. **Cross-platform compatibility**: Windows/Linux/macOS differences
   - Mitigation: Test on all platforms, use crossterm
2. **Terminal compatibility**: Different terminal capabilities
   - Mitigation: Graceful degradation, feature detection
3. **Dependency updates**: Breaking changes in dependencies
   - Mitigation: Pin dependencies, test with updates

### Low Risk Items
1. **Theme customization**: User-defined themes
   - Mitigation: Validate theme configurations
2. **Keybinding conflicts**: User-defined keybindings
   - Mitigation: Detect and warn about conflicts

---

## Success Metrics

### Phase Completion
| Phase | Tasks | Tests | Coverage | Status |
|-------|-------|-------|----------|--------|
| 1 | 6 | 5 | N/A | ⬜ |
| 2 | 4 | 4 | 100% | ⬜ |
| 3 | 7 | 6 | 90% | ⬜ |
| 4 | 4 | 4 | 95% | ⬜ |
| 5 | 4 | 4 | N/A | ⬜ |
| 6 | 5 | 5 | 95% | ⬜ |
| 7 | 5 | 5 | N/A | ⬜ |
| 8 | 5 | 3 | N/A | ⬜ |

### Quality Metrics
- **Test coverage**: >85% overall
- **Clippy warnings**: 0
- **Format issues**: 0
- **Documentation**: 100% public items
- **Build warnings**: 0

---

## Next Steps

1. **Spawn Subagent 1**: Start Phase 1 (Protocol layer)
2. **Spawn Subagent 2**: Can start Phase 1 in parallel (Project structure)
3. **Monitor progress**: Track commit points
4. **Review**: Code review at each phase completion
5. **Merge**: Merge phases to main as they complete

## References

- [ARCHITECTURE.md](ARCHITECTURE.md) - Full architecture document
- [Cargo.toml](../Cargo.toml) - Project dependencies
- [tui_gateway/server.py](../../tui_gateway/server.py) - Gateway server
- [ui-tui/](../../ui-tui/) - TypeScript TUI reference
- [ratatui documentation](https://ratatui.rs/)
- [crossterm documentation](https://docs.rs/crossterm)
