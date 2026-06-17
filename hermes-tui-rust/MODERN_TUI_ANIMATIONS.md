# Project Hermes: Advanced TUI Animation & Architecture Plan

This document serves as the definitive, multi-phase technical specification for implementing a highly dynamic, visually striking, and zero-latency terminal user interface (TUI) for Project Hermes. 

It synthesizes state-of-the-art Rust TUI techniques extracted from industry-leading repositories (`CodeWhale`, `oh-my-pi`, `oha`, `Yazi`, `Helix`) and official `ratatui.rs` best practices. It rigorously enforces architectural backpressure, strict mutability boundaries, and zero-flicker terminal rendering.

---

## 🏗️ Phase 1: Core Engine & Async Foundation

**Goal:** Establish a strictly bounded, backpressure-aware, demand-driven render loop that never starves the UI thread or causes Out-Of-Memory (OOM) errors during chaotic LLM streaming.

### 1.1 Bounded Worker Pools & Backpressure (`Helix` & Gemini Critique)
**Instruction:** Never use unbounded channels (`tokio::sync::mpsc::unbounded_channel`) for the primary event bus. Unbounded queues during a heavy LLM streaming event will cause memory bloat and UI starvation.
- [ ] Initialize `tokio::sync::mpsc::channel` with a calculated buffer size (e.g., 1024) for critical events to enforce backpressure.
- [ ] Use `tokio::sync::watch` channels for state that only requires the latest value (e.g., progress bars, CPU/RAM telemetry).
- [ ] Implement the **Agent Context Protocol (ACP) Adapter** via JSON-RPC. Rust Tokio workers should handle serialization and dispatching, allowing Python to maintain sandbox isolation.

### 1.2 Demand-Driven Animation States (Anti-Polling Pattern)
**Instruction:** Brute-force 60-FPS polling forces the CPU to diff the terminal buffer continuously, consuming up to 50% of CPU time even when idle.
- [ ] Ditch the continuous 16ms tick loop. Instead, implement a **Demand-Driven Render State**.
- [ ] The Central Dispatcher maintains an `AtomicUsize` of active animations.
- [ ] The 60-FPS ticker is *suspended entirely* unless `active_animations > 0`. The event loop otherwise blocks cleanly on user input or async I/O.

### 1.3 Zero-Flicker Synchronization (`CodeWhale` & `Yazi` pattern)
**Instruction:** Ratatui redraws can cause subtle tearing on modern emulators (Kitty, Ghostty) during high-frequency updates. Wrap frames in DEC 2026 sync sequences.
- [ ] Override standard stdout initialization to inject `\x1b[?2026h` (Begin Sync) and `\x1b[?2026l` (End Sync) around the `terminal.draw` call. This guarantees atomic frame swaps.

---

## ⚡ Phase 2: The Component Tree & Mutability Boundaries

**Goal:** Build a hierarchical layout tree that respects idiomatic Rust borrowing rules and supports zero-copy rendering.

### 2.1 The Mutability Trap & WidgetRef
**Instruction:** Do **not** pass `&mut self` to a Component's `draw()` function. Doing so prevents widget caching and triggers mutable aliasing compiler errors when iterating over global state.
- [ ] Components must implement rendering strictly as an immutable operation.
- [ ] Utilize Ratatui's `WidgetRef` trait to build complex component structures once and render them via immutable references multiple times.
- [ ] For components requiring internal rendering state (e.g., visual scroll offsets calculated during layout), implement Ratatui's `StatefulWidget` trait. Pass the mutable `State` struct separately during the `f.render_stateful_widget` call.

### 2.2 Thread-Local O(1) Syntax Highlighting (`oh-my-pi` pattern)
**Instruction:** Standard `syntect` theme resolution is too slow for high-throughput LLM streaming.
- [ ] Implement a thread-local cache (`SCOPE_COLOR_CACHE`).
- [ ] Flatten the Hermes theme into an 11-color palette array.
- [ ] Parse incoming markdown chunks and resolve colors in $O(1)$ time.

```rust
// [Code Template: O(1) Theme Resolution]
thread_local! {
    static PALETTE: [&'static str; 5] = ["\x1b[38;5;244m", "\x1b[38;5;81m", /*...*/];
}

pub fn fast_highlight(text: &str, scope: usize) -> String {
    let color = PALETTE.get(scope).unwrap_or(&"\x1b[39m");
    format!("{}{}\x1b[39m", color, text)
}
```

### 2.3 Terminal-Aware Dynamic Histograms (`oha` pattern)
**Instruction:** Charts must fit perfectly without visual clipping. Do not pre-calculate bins; calculate bins dynamically inside the `render` function based on the exact `Rect` width available.
- [ ] When drawing stats (e.g., token generation speed), determine `bins = block.width / bar_width`.
- [ ] Apply algorithm backwards (`iter().rev()`) and `break` early to save CPU once the visible chart window is filled.

---

## 📝 Phase 3: The Embedded Modal Editor

**Goal:** Integrate a high-performance text editor capable of handling massive, LLM-generated code refactoring and diffing without stalling the event loop.

### 3.1 Rope Data Structures (`Helix` pattern)
**Instruction:** A standard Rust `String` or `Vec<String>` is catastrophic for large text editing.
- [ ] Replace standard strings with a **Rope data structure** (`ropey` crate) for all editable contexts and large chat logs.
- [ ] This ensures logarithmic time complexity for insertions and deletions, regardless of file size.

### 3.2 Render Culling & Syntax Caching
**Instruction:** Re-running Regex or Tree-sitter over a massive buffer on every 16ms animation frame will crash the UI.
- [ ] Implement strict **Render Culling**: Query the Rope *only* for the subset of lines visible within the physical `Rect` constraints.
- [ ] Cache syntax highlighting spans for formatted lines. Only invalidate the cache for lines explicitly altered by user input or LLM generation.

### 3.3 Native Cursor Synchronization & Input Trapping
**Instruction:** The physical hardware cursor must align with multi-width Unicode characters to support screen readers and non-Latin Input Method Editors.
- [ ] Map internal Rope coordinates to terminal coordinates using the `unicode-width` crate.
- [ ] When the editor enters "Insert Mode", the Central Dispatcher must **suspend global hotkey resolution** and pipe key payloads strictly to the focused editor component.

---

## 🎨 Phase 4: Native Math & Aetheric Shaders

**Goal:** Provide constant visual feedback using pure math and `tachyonfx`, requiring minimal memory churn.

### 4.1 Mathematical Block Waves (`CodeWhale` pattern)
**Instruction:** For "Thinking/Working" states, do not use simple dots. Use sine waves mapped to Ratatui block characters (` ' ', '▂', '▃', '▄', '▅' `) to create a fluid, oceanic footer.

```rust
// [Code Template: Phase-Shifted Wave]
const WAVE: [char; 5] = [' ', '▂', '▃', '▄', '▅'];

pub fn wave_glyph(x: usize, tick: usize) -> char {
    let t = tick as f64 / 100.0;
    let x_f = x as f64;
    let val = (x_f * 0.5 - t * 8.0).sin() + (x_f * 0.2 + t * 3.0).sin() * 0.5;
    let normalized = ((val + 1.5) / 3.0).clamp(0.0, 1.0);
    WAVE[(normalized * 4.0).round() as usize]
}
```

### 4.2 Protocol-Linked DSL Shaders (`tachyonfx`)
**Instruction:** Initialize `tachyonfx::EffectManager` in the root `App`. Apply it to the `Frame` buffer *after* all rendering is complete. Optimize for changing ANSI styles over graphemes to reduce buffer diffing overhead.
- [ ] **LLM Delta Streaming:** Trigger `fx::coalesce(300)` over the newly added text block.
- [ ] **Tool Execution:** Trigger `fx::hsl_shift` to sweep a bright cyan/green line across the tool card.

```rust
// [Code Template: Tachyon Post-Processing]
terminal.draw(|frame| {
    let area = frame.area();
    // 1. Native Render via Immutable References
    frame.render_widget(&chat_widget, area);
    
    // 2. Shader Overlay
    if !state.low_motion {
        state.effects.process_effects(tick_duration, frame.buffer_mut(), area);
    }
})?;
```

---

## 🚀 Execution Checklist

### Sprint 1: Foundation (Zero-Flicker & Async)
- [ ] Setup `tokio` worker pools with **Bounded Channels** (`mpsc::channel`) to enforce backpressure.
- [ ] Enable DEC 2026 buffer synchronization for tear-free output.
- [ ] Implement the **Demand-Driven Render State** engine to suspend 60-FPS polling when idle.

### Sprint 2: Core UX (Components & Editor)
- [ ] Refactor the Component trait to use `WidgetRef` and `StatefulWidget` instead of `&mut self` draws.
- [ ] Integrate the `ropey` crate for managing large text inputs and chat histories.
- [ ] Implement `SCOPE_COLOR_CACHE` for $O(1)$ Markdown highlighting.

### Sprint 3: The "Aetheric" Polish (Shaders & Math)
- [ ] Build the CodeWhale-inspired Sine Wave Footer.
- [ ] Integrate `tachyonfx` and bind JSON-RPC `tool.start` to shader pipelines.
- [ ] Implement mathematical cursor syncing via `unicode-width` for the embedded editor.