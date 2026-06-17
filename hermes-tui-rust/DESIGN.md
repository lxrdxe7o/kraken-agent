# Hermes TUI: Visual Design & Aesthetics Architecture

This document outlines the visual design language, layout strategies, animation paradigms, and aesthetic constraints for the Hermes TUI. It synthesizes the cutting-edge terminal UX capabilities showcased in the [Ratatui Ecosystem](https://ratatui.rs/) (e.g., *Slumber*, *Yazi*, *CodeWhale*) with the strict performance and stability boundaries defined by our [Architectural Critique](https://gemini.google.com/share/856119b3ad9c).

Our goal is to deliver a zero-latency, visually striking, and completely memory-safe terminal application.

---

## 1. Layout & Composition (The Cassowary Grid)

Ratatui uses a Constraint-based Layout System powered by the Cassowary solver. Hermes will leverage this to create a responsive, IDE-like workspace rather than a flat, scrolling terminal log.

### 1.1. Adaptive Grid & Flexbox Paradigms
*   **Nested Layouts:** The core UI uses a root App container that splits vertically (Toolbar, Main Body, Status) and horizontally (Left Sidebar, Chat, Right Sidebar).
*   **Flex Alignment:** Utilize Ratatui's `Flex` features (`SpaceBetween`, `SpaceAround`, `Center`) to align widgets dynamically. Modal windows (e.g., login, configuration) will use `Flex::Center` to remain perfectly centered regardless of the terminal dimensions.
*   **Responsive Collapsing:** Use `Constraint::Min` and `Constraint::Max` to define boundaries. If the terminal width drops below 100 columns, sidebars will collapse into a vertical "Kanban" stack or hide entirely behind a Tab navigation system.
*   **Breathability:** Enforce `.spacing(1)` or `.spacing(2)` on layouts to add critical visual gaps between Panes, preventing the UI from feeling claustrophobic.

### 1.2. Floating Overlays & Modals
*   **Layered Modals:** Complex, focused interactions (like File Picking, Model Selection, or Settings) will utilize the `Clear` widget.
*   *Implementation:* Calculate a centered `Constraint::Percentage` `Rect`, render `Clear` to erase the background, and draw a `Block` with `BorderType::Thick` and an active shadow effect.

---

## 2. Advanced Widget Repertoire

To achieve a "Dashboard" aesthetic, Hermes will utilize Ratatui's high-fidelity widgets to represent data and state visually, minimizing text parsing for the user.

### 2.1. Telemetry & Analytics
*   **Canvas Widget:** Use for "drawing" high-resolution, Braille-character (8 dots per cell) charts. Ideal for visualizing real-time metrics like agent "thought space" embeddings or complex network topologies.
*   **Sparklines:** Deploy compact, in-line trend lines (using `Sparkline`) within table cells or small corner blocks to visualize historical data, such as CPU/Memory usage over the last 60 seconds, without sacrificing real estate.
*   **Gauges & LineGauges:** Use solid block characters (`█`, `▓`, `▒`, `░`).
    *   `Gauge` for large, prominent tasks (e.g., pulling a massive LLM model).
    *   `LineGauge` for subtle, single-line progress indicators nested in the bottom Status bar.

### 2.2. Stateful Navigation & Data Grids
*   **Stateful Tables & Lists:** Utilize `TableState` and `ListState` to manage scrolling through agent logs, file trees, or configuration properties.
*   **Visual Selection:** Highlight selected rows with an inversion of background color (e.g., `Color::Blue` bg, `Color::Black` fg) and prefix active elements with indicators like `>>` or a custom Nerd Font icon.
*   **Tabs:** Positioned at the very top (Toolbar) to provide high-level context switching (e.g., Chat, Editor, Graph, Settings) with distinct active/inactive `Modifier::BOLD` / `Modifier::DIM` styling.

---

## 3. Typography & Glyph Design

*   **Nerd Fonts Integration:** Extensive use of icons for file types, agent statuses, and navigation. 
*   **Inline Border Titles:** To maximize vertical space, place titles and labels directly within the top or bottom border of a `Block` (e.g., `Block::default().title(" Modal Editor ")`).
*   **Sub-cell Resolution:** 
    *   **Braille** (`⡀`, `⡄`, `⡆`, `⡇`, `⣇`, `⣧`, `⣷`, `⣿`) strictly for continuous curves, maps, and high-density line charts.
    *   **Blocks** (` `, `▂`, `▃`, `▄`, `▅`, `▆`, `▇`, `█`) strictly for volume, bars, audio waveforms, and loading geometry.

---

## 4. O(1) Theming & Color Science

*   **Pre-computed Palettes:** To satisfy the strict 8.33ms (120 FPS) render constraint, runtime theme calculation is forbidden. We utilize a thread-local `SCOPE_COLOR_CACHE` mapped to an 11-color static ANSI palette (e.g., Catppuccin Macchiato or Nord).
*   **24-bit RGB & Gradients:** Where terminal support allows, utilize full RGB for soft gradient headers or telemetry heatmaps.
*   **Semantic Modifiers:**
    *   Heavy use of `Modifier::BOLD` for active focus areas.
    *   Heavy use of `Modifier::DIM` to recede secondary information or inactive panes into the background.
    *   *Alerts:* Alternating `Color::Red` and `Modifier::DIM` via a timer for pulsing/blinking critical alerts (e.g., bounded channel saturation).

---

## 5. Motion Design & Animations

Animations in Hermes provide crucial state feedback (loading, streaming, erroring) without relying on text.

### 5.1. Mathematical Visualizers (Inspired by *CodeWhale*)
*   Instead of static loading spinners, background agent thought processes will be represented by **phase-shifted sine-waves** mapped to block characters.
*   *Algorithm:* Render organic loaders using sinusoidal RGB lerping calculated backwards (`iter().rev()`) based on the active `Rect` width.

### 5.2. Smooth Scrolling & State Changes
*   Scrolling through `ropey` text buffers or stateful lists should adjust offset incrementally to provide a sense of continuous motion, rather than jumping paginations.

### 5.3. Hardware-Accelerated Shaders (via *TachyonFX*)
*   **Post-processing Effects:** Use `tachyonfx` for UI transitions.
    *   *Slide In:* Chat bubbles appearing smoothly from the bottom.
    *   *Fade:* Dimming the background IDE when a Modal Editor opens.
    *   *Glitch:* A brief, stylized visual glitch when an AI agent encounters a severe hallucination or error.
*   **Cell Safety Constraint:** TachyonFX filters must **never** be applied to `Rect` spaces currently rendering SIXEL graphics to prevent buffer corruption.

---

## 6. The Performance Mandate: Beauty without Starvation

The aesthetics detailed above *must* operate within the bounds of the architectural critique to prevent OOM errors, CPU starvation, and UI tearing.

### 6.1. Demand-Driven Rendering
We cannot poll animations at 120Hz indefinitely. The TUI employs a **Demand-Driven Render State**:
*   An `AtomicUsize` tracks active animations (e.g., `ACTIVE_ANIMATIONS.fetch_add(1)`).
*   If `ACTIVE_ANIMATIONS == 0`, the render loop suspends itself, dropping CPU usage to ~0%.
*   Any incoming event over the bounded `mpsc` channel immediately wakes the renderer.

### 6.2. Zero-Flicker Architecture
To support complex shader transitions and layered modals without visual tearing (especially on GPU-accelerated emulators like Ghostty/Kitty/WezTerm):
*   Every frame swap is strictly wrapped in **DEC 2026 Synchronized Output** sequences (`\x1b[?2026h` before draw, `\x1b[?2026l` after draw).

### 6.3. Zero-Copy `WidgetRef`
*   Dynamic visual states (like an animated loading bar within a chat bubble) must use `WidgetRef` and `StatefulWidget`.
*   We absolutely forbid `&mut self` mutations in the `draw()` method to allow components to cache their layout computations.

---

## 7. Rich Media: Terminal as a Canvas

### 7.1. Sixel & Kitty Graphics
*   Hermes will render AI-generated images natively into the TUI using the `icy_sixel` crate.
*   **Scaling:** Images will be processed asynchronously on a background worker thread using `imageops::FilterType::Lanczos3` to scale them cleanly into terminal cell coordinates before passing them to the UI thread.

### 7.2. Embedded Modal Editor
*   For deep code editing or prompt crafting, a `ropey`-backed modal editor floats over the UI.
*   It features strict Vim-like Normal and Insert modes.
*   **Cursor Mapping:** Uses the `unicode-width` crate to mathematically align the logical `ropey` coordinates with the physical hardware cursor, ensuring seamless integration of the terminal's native text selection capabilities.

---

## 8. Summary of Fallbacks & Graceful Degradation

1.  **Strict Backpressure:** If the LLM streams tokens faster than the terminal can render them, the bounded `mpsc` channel will exert backpressure, prioritizing the UI thread's responsiveness over the text stream.
2.  **Graceful Degradation:** All motion design and Sixel visuals are strictly feature-gated. If the user passes `--no-animation`, `--low-motion`, or if the terminal does not support SIXEL/DEC 2026, the UI falls back to an elegant, static layout using standard ASCII/ANSI primitives (e.g., replacing Braille charts with simple text percentages).