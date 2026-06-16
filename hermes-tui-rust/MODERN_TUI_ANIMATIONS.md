# Modern TUI Animations: The Hermes Surface Paradigm (2025/2026)

## 1. Vision: The "Aetheric Command Center"
Hermes is a multi-tier, modular AI agent. The TUI is its **Primary Command Surface**. The aesthetic must reflect the agent's core identity: **Professional, Secure, and Intelligent.** We aim for a "Bioluminescent Deep-Sea" look—dark backgrounds with soft, organic glows that signal cognitive activity without distracting from the data.

---

## 2. Industry Benchmarks & Inspiration
We draw inspiration from the most visually advanced open-source TUIs of 2024–2025:

| Project | Key Innovation | Takeaway for Hermes |
| :--- | :--- | :--- |
| **Tek (TachyonFX)** | Buffer-level shader effects | Use for "Glitch" transitions and HSL color cycles. |
| **Yazi** | High-refresh-rate async UI | Ensure non-blocking render loops for Sixel assets. |
| **Slumber** | "Material Design" TUI layout | Clean, collapsible structures for tool cards. |
| **Oatmeal** | Polished LLM chat bubbles | Smooth streaming animations for agent responses. |
| **Tui-Skeleton** | Animated loading states | Use "Shimmer" for tool execution & history fetch. |

---

## 3. Agent-Integrated Animation Stack
Animations in Hermes are not just "flair"; they are **visual telemetry** linked directly to the JSON-RPC protocol events from the Gateway.

| Event (Protocol) | Visual Effect | Functional Purpose |
| :--- | :--- | :--- |
| `message.delta` | **Neural Evolve** | Signals "decryption" of thought into language. |
| `tool.start` | **Bioluminescent Pulse** | Indicates active background execution/sandboxing. |
| `message.complete`| **Coalesce** | Finalizes the response, signaling the end of the turn. |
| `status.change` | **Color Sweep** | Smoothly updates connectivity (e.g., Online -> Offline). |
| `error` | **Aetheric Shake** | High-visibility signal for Tirith security/logic errors. |

---

## 4. protocol-Linked Visual Patterns

### A. Non-Blocking "Neural Evolve"
When `MessageDelta` arrives, we apply a "cipher" effect to the latest word chunk. To ensure we don't break the streaming experience, the animation must be **stateless and additive**.

```rust
// Logic: Animate only the most recent chunk of a streaming message
fn render_streaming_delta(manager: &mut EffectManager, text: &str, area: Rect) {
    if is_new_delta(text) {
        manager.add_effect(
            fx::evolve(300), // Quick 300ms evolve
            area,
            CellFilter::AllNonEmpty
        );
    }
}
```

### B. Spring-Based "Material" Transitions
Inspired by **Slumber**, panels slide and "bounce" using `animate` spring physics. This gives the TUI a tactile, high-end feel compared to static layouts.

```rust
#[animate]
struct SidebarState {
    #[animate(spring(stiffness = 100.0, damping = 10.0))]
    width_percent: f32, // Smoothly transitions from 0.0 to 30.0
}
```

### C. "Tirith" Security Approval Pulse
Hermes uses the **Tirith** security layer. When an approval is required, the UI should "demand" attention using a high-fidelity, high-contrast glow.

**TachyonFX DSL for Approval:**
```rust
let approval_glow = fx::sequence(&[
    fx::hsl_shift_fg(20.0, 1.0, 0.5, 400), // Brighten
    fx::hsl_shift_fg(-20.0, -1.0, -0.5, 400), // Dim
]).repeat(); // Pulse until approved
```

---

## 5. Architectural Safeguards (Don't Break the Agent)

### 1. The "EffectManager" Isolation
The `EffectManager` must process frames on a **separate budget** from the JSON-RPC parser.
- **Rule:** If the protocol thread is saturated, the `EffectManager` should drop frames (graceful degradation) to ensure no message latency.

### 2. Stateless Rendering
Widgets remain **pure functions of the state**. Animations are "post-processing" overlays.
- **Why:** This ensures that if the animation engine fails, the TUI remains 100% functional with standard Ratatui rendering.

### 3. Sixel/Kitty Buffer Safety
Since Hermes supports **SIXEL GIFs**, the animation loop must be aware of the "SIXEL lock."
- **Rule:** Do not apply `tachyonfx` filters to SIXEL-occupied cells to avoid artifacts.

---

## 6. Implementation Plan (Phased Rollout)

### Phase 1: Foundation & Post-Processing
- [ ] Integrate `tachyonfx` and initialize the `EffectManager` in `App`.
- [ ] Implement the `Banner` fade-in and `Toolbar` bioluminescent pulse.
- [ ] Add the `--no-animation` fallback flag.

### Phase 2: Protocol-Linked Effects
- [ ] Bind `Neural Evolve` to `message.delta` streaming.
- [ ] Implement `Shimmer` loading states for `SessionManager` fetch.
- [ ] Connect `Tirith Approval Pulse` to tool execution requests.

### Phase 3: Physics & High-Fi Assets
- [ ] Integrate `animate` for spring-based panel transitions (Chat <-> Dashboard).
- [ ] Optimize Sixel GIF rendering with non-blocking crossfades.

---

*Refined for Hermes Agent Tier-1 Surface Compliance.*
*Synthesized from Elite TUI Benchmarks (Yazi, Tek, Slumber, Oatmeal).*
