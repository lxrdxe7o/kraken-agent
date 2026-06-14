# Rust TUI Orchestration Tracker

## 🎯 Project Overview

Building a Rust-based TUI for Hermes Agent that replicates oh-my-pi's TUI features.

**Status**: Phase 1 - Foundation (In Progress)  
**Start Date**: 2026-06-14  
**Timeline**: 16 weeks to production-ready  

---

## 📊 Commit History

### Commit 1: Initial Project Structure ✅

**Hash**: `b8c2c342a`  
**Date**: 2026-06-14  
**Message**: feat(rust-tui): Initial project structure and protocol types

**Contents**:
- ✅ `hermes-tui-rust/` directory structure
- ✅ `Cargo.toml` with all dependencies (ratatui, crossterm, tokio, serde, etc.)
- ✅ `README.md` with comprehensive documentation
- ✅ `.gitignore` for Rust projects
- ✅ Full module structure (src/{app,protocol,state,handlers,ui,utils})
- ✅ Protocol types implementation (`src/protocol/types.rs`)
  - GatewayMessage enum with all variants
  - TuiRequest enum for requests
  - All message types from gatewayTypes.ts
  - Session, tool, approval, completion types
- ✅ Transport and client stubs
- ✅ TDD tests for protocol types (10 tests, all passing)
- ✅ Placeholder files for all modules

**Quality Gates**:
- ✅ Compiles without errors
- ✅ All tests pass (10/10)
- ✅ Protocol types match TypeScript definitions
- ✅ Serialization/deserialization working
- ⚠️ 24 warnings (mostly unused code - expected for stubs)

---

## 🏗️ Current Architecture

```
hermes-tui-rust/
├── Cargo.toml              # Dependencies configured
├── README.md               # Full documentation
├── .gitignore              # Rust-specific ignores
├── src/
│   ├── main.rs            # Entry point (✅ compiles)
│   ├── lib.rs             # Library exports (✅ compiles)
│   ├── app.rs             # App struct stub (✅ compiles)
│   ├── protocol/
│   │   ├── mod.rs         # Module exports (✅ compiles)
│   │   ├── types.rs       # ✅ FULLY IMPLEMENTED - All gateway types
│   │   ├── client.rs      # Client stub (✅ compiles)
│   │   └── transport.rs   # Transport stub (✅ compiles)
│   ├── state/             # State modules (stubs)
│   │   ├── mod.rs
│   │   ├── session.rs
│   │   ├── config.rs
│   │   └── messages.rs
│   ├── ui/                # UI modules (stubs)
│   │   ├── mod.rs
│   │   ├── chat.rs
│   │   ├── composer.rs
│   │   ├── toolbar.rs
│   │   ├── cards.rs
│   │   └── prompts.rs
│   ├── handlers/          # Event handlers (stubs)
│   │   ├── mod.rs
│   │   ├── input.rs
│   │   ├── keys.rs
│   │   └── mouse.rs
│   └── utils/             # Utilities (stubs)
│       ├── mod.rs
│       ├── text.rs
│       └── ansi.rs
└── tests/                  # Test directory (empty for now)
```

---

## 📋 Next Tasks (Phase 1 Continued)

### Task 2: Implement Protocol Client & Transport

**Priority**: HIGH  
**Estimated Time**: 3-5 days  
**Status**: Pending  

**Subtasks**:
- [ ] Implement `StdioTransport` with async message reading
- [ ] Implement `GatewayClient` with request/response handling
- [ ] Add message queue for incoming messages
- [ ] Implement request ID tracking
- [ ] Add error handling for broken pipes
- [ ] Add connection lifecycle management

**Deliverables**:
- `src/protocol/transport.rs` - Full implementation
- `src/protocol/client.rs` - Full implementation
- TDD tests for transport and client
- All tests passing

**Quality Gates**:
- ✅ Compiles without errors
- ✅ All tests pass
- ✅ No memory leaks
- ✅ Proper error handling
- ✅ Thread-safe implementation

---

### Task 3: Implement Terminal Setup with ratatui

**Priority**: HIGH  
**Estimated Time**: 2-3 days  
**Status**: Pending  
**Depends On**: Task 2

**Subtasks**:
- [ ] Initialize crossterm terminal
- [ ] Set up ratatui with proper layout
- [ ] Implement basic event loop
- [ ] Handle terminal resize events
- [ ] Handle Ctrl+C, Ctrl+D gracefully
- [ ] Add mouse support detection
- [ ] Add Kitty terminal protocol support (initial focus)

**Deliverables**:
- `src/app.rs` - Full terminal setup and event loop
- `src/handlers/input.rs` - Basic input handling
- TDD tests for terminal setup
- All tests passing

---

### Task 4: Implement Gateway Communication

**Priority**: HIGH  
**Estimated Time**: 3-5 days  
**Status**: Pending  
**Depends On**: Task 2, Task 3

**Subtasks**:
- [ ] Send `gateway.ready` handshake
- [ ] Parse initial state from gateway
- [ ] Handle session list from gateway
- [ ] Implement message delta streaming
- [ ] Implement message complete handling
- [ ] Implement tool start/progress/complete handling
- [ ] Implement approval request handling
- [ ] Implement completion response handling

**Deliverables**:
- Full gateway communication working
- All message types handled
- TDD tests for message handling
- All tests passing

---

## 👥 Agent Roles

### 1. Orchestrator Agent (YOU - Current)
- **Role**: Overall project coordination
- **Responsibilities**:
  - Track progress across all agents
  - Ensure quality gates are met
  - Coordinate dependencies between tasks
  - Review and merge pull requests
  - Maintain the orchestration tracker
- **NOT**:
  - Does NOT write implementation code
  - Does NOT modify core Hermes files (tui_gateway/server.py)

### 2. Protocol Agent
- **Role**: JSON-RPC protocol implementation
- **Responsibilities**:
  - Implement transport layer
  - Implement client layer
  - Ensure protocol compatibility
  - Write TDD tests for protocol
- **Focus**: Phase 1 (Foundation)

### 3. UI Agent
- **Role**: Terminal UI implementation
- **Responsibilities**:
  - Implement ratatui components
  - Implement chat transcript
  - Implement input composer
  - Implement tool cards
  - Implement prompts
  - Write TDD tests for UI
- **Focus**: Phase 2-4 (Core UI, Tool Integration)

### 4. Integration Agent
- **Role**: Hermes integration
- **Responsibilities**:
  - Modify hermes_cli/main.py
  - Add --tui-rust flag
  - Add configuration options
  - Add fallback logic
  - Write integration tests
- **Focus**: Phase 8 (Integration)

### 5. Test Agent
- **Role**: Quality assurance
- **Responsibilities**:
  - Write TDD tests for all components
  - Ensure test coverage
  - Run tests on all platforms
  - Maintain test infrastructure
- **Focus**: All phases

### 6. Review Agent
- **Role**: Code review
- **Responsibilities**:
  - Review all code before merge
  - Enforce coding standards
  - Check for protocol compatibility
  - Verify no Hermes core modifications
- **Focus**: All phases

---

## ⚠️ Critical Constraints

### MUST NOT BREAK
1. **Hermes Core Functionality** - TOP PRIORITY
   - `tui_gateway/server.py` - DO NOT MODIFY
   - `run_agent.py` - DO NOT MODIFY
   - `model_tools.py` - DO NOT MODIFY
   - Existing TypeScript TUI must continue to work

2. **JSON-RPC Protocol**
   - Must be 100% compatible with existing protocol
   - Must handle all message types from gatewayTypes.ts
   - Must not break existing clients (Desktop, Dashboard)

3. **Fallback Behavior**
   - If Rust TUI fails, must fall back to TypeScript TUI
   - Must not break existing Hermes functionality

### Quality Gates

All code must pass through these gates before being committed:

1. **Compilation**
   - `cargo check` must pass
   - No compilation errors
   - Warnings should be minimized (but acceptable for stubs)

2. **Tests**
   - `cargo test` must pass
   - All tests must pass
   - TDD approach: tests before implementation

3. **Clippy**
   - `cargo clippy` should pass (warnings acceptable for now)

4. **Format**
   - `cargo fmt` must be clean

5. **Review**
   - Code review by at least one other "agent"
   - For now, self-review with checklist

6. **Atomic Commits**
   - Each commit must be atomic (one logical change)
   - Must be able to revert any commit without breaking the project
   - Commit messages follow Conventional Commits

---

## 📈 Progress Tracking

### Phase 1: Foundation (Weeks 1-2)
- [x] Create project structure
- [x] Add Cargo.toml with dependencies
- [x] Implement protocol types
- [x] Add initial TDD tests
- [x] Commit 1: Initial structure
- [x] Implement transport layer
- [x] Implement client layer
- [x] Implement terminal setup
- [x] Implement gateway handshake types
- [x] Implement complete state management (messages, sessions, config, themes)
- [x] Implement all UI components (chat, composer, toolbar, prompts, cards)
- [x] Implement all handlers (keys, mouse, input)
- [x] Implement all utils (text, ANSI, syntax highlighting)
- [x] Implement oh-my-pi features (hashline, tool cards, subagent UI)
- [x] Add Python CLI integration (--tui-rust flag)
- [x] 171 tests passing, clean compilation

### Phase 2: Integration (Current Focus)
- [ ] Wire gateway event loop in `App::run()`
- [ ] Drive handler methods from event loop
- [ ] Connect GatewayClient for message send/receive
- [ ] Integration tests with real tui_gateway
- [ ] Wire remaining no-op handlers (mouse, resize, paste, completions)
- [ ] Implement config loading from disk
- [ ] LSP/Debugger UI (later phase)

**Current Status**: Phase 1 complete. Phase 2 gateway integration in progress.
## 🚀 Quick Commands

### Build & Test
```bash
# Check compilation
cargo check

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Clippy lint
cargo clippy

# Format code
cargo fmt
```

### Git
```bash
# Check status
git status

# Create feature branch
git checkout -b feat/rust-tui-{description}

# Atomic commit
git add {specific files}
git commit -m "{type}: {description}"

# Push branch
git push origin {branch}
```

---

## 📝 Notes

### Platform Focus
- **Initial Focus**: Kitty terminal with tmux
- **Test Early**: All features must work in Kitty + tmux first
- **Then Expand**: Linux (xterm), macOS (iTerm2), Windows (Windows Terminal)

### Protocol Compatibility
- **Reference**: `tui_gateway/server.py` and `ui-tui/src/gatewayTypes.ts`
- **Must Match**: All message types, fields, and behavior
- **Test Strategy**: Write tests that verify protocol compatibility

### TDD Approach
- Write failing test first
- Watch it fail (RED)
- Write minimal code to pass (GREEN)
- Refactor (REFACTOR)
- Repeat

---

## 🔗 References

- **Plan Document**: `/home/lxrdxe7o/.vibe/plans/1781387655-witty-noble-river.md`
- **Hermes AGENTS.md**: `/home/lxrdxe7o/.hermes/hermes-agent/AGENTS.md`
- **TUI Development Guide**: `/home/lxrdxe7o/.hermes/hermes-agent/TUI-DEVELOPMENT.md`
- **oh-my-pi Reference**: https://github.com/can1357/oh-my-pi/tree/main/packages/tui

---

*Last Updated: 2026-06-14*
*Orchestrator: Mistral Vibe*
