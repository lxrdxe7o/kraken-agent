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
//! Call `render_wave_footer(frame, area, tick, usage)` in your draw path whenever
//! `thinking == true`.  The component respects `ACTIVE_ANIMATIONS` so the
//! event loop stays demand-driven.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::engine;

const WAVE_STR: [&str; 5] = [" ", "▂", "▃", "▄", "▅"];

const COLOR_PROMPT: Color = Color::Rgb(102, 217, 239); // Cyan
const COLOR_TOOL: Color = Color::Rgb(250, 189, 47);    // Yellow
const COLOR_REASON: Color = Color::Rgb(174, 129, 255); // Purple
const COLOR_OUTPUT: Color = Color::Rgb(166, 226, 46);  // Green
const COLOR_FAILED: Color = Color::Rgb(249, 38, 114);  // Red

const COLORS: [Color; 5] = [
    COLOR_PROMPT,
    COLOR_TOOL,
    COLOR_REASON,
    COLOR_OUTPUT,
    COLOR_FAILED,
];

const LABELS: [&str; 5] = ["Prompt", "Tool", "Reason", "Output", "Fail"];

/// Evaluate a phase-shifted dual-sine wave at column `x` for animation tick
/// `tick`.
///
/// Two sine waves are summed so the pattern feels organic rather than a single
/// boring sinusoid. Amplitude and speed vary based on the token `count`.
#[must_use]
pub fn wave_glyph(x: usize, tick: usize, count: u32) -> &'static str {
    // Speed proportional to token count.
    let speed_mult = 1.0 + (count as f64).ln_1p() * 0.5;
    // Amplitude increases with tokens, but starts low.
    let amp_mult = 0.15 + ((count as f64).ln_1p() / 6.0).clamp(0.0, 0.85);

    let t = (tick as f64 / 100.0) * speed_mult;
    let x_f = x as f64;
    
    // Evaluate base wave in [-1.5, 1.5]
    let val = (x_f * 0.5 - t * 8.0).sin() + (x_f * 0.2 + t * 3.0).sin() * 0.5;
    
    // Normalize to [0, 1] then apply amplitude mapping to bias toward bottom when count=0
    let normalized_base = (val + 1.5) / 3.0; 
    let normalized = (normalized_base * amp_mult).clamp(0.0, 1.0);

    let idx = (normalized * 4.0).round() as usize;
    WAVE_STR[idx.clamp(0, 4)]
}

/// Render an animated wave footer across the full width of `area`.
///
/// - **Renders backwards** (`iter().rev()`) so the rightmost edge is always
///   the freshest data.
/// - Calls `engine::animation_start()` / `engine::animation_end()` to keep the
///   demand-driven render counter correct.
pub fn render_wave_footer(frame: &mut Frame, area: Rect, tick: usize, usage: (u32, u32, u32, u32, u32)) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let text_height = 1.min(area.height);
    let wave_height = area.height.saturating_sub(text_height);

    let wave_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: wave_height,
    };
    
    let text_area = Rect {
        x: area.x,
        y: area.y + wave_height,
        width: area.width,
        height: text_height,
    };

    let width = area.width as usize;
    let counts = [usage.0, usage.1, usage.2, usage.3, usage.4];
    
    // Calculate pillar widths
    let mut col_widths = [0_u16; 5];
    for x in 0..width {
        let segment_idx = (x * 5) / width;
        col_widths[segment_idx] += 1;
    }

    let bg_color = Color::Rgb(27, 32, 33);

    // Render wave
    if wave_height > 0 {
        let mut lines: Vec<Line> = Vec::with_capacity(wave_height as usize);
        
        let styles = [
            Style::default().fg(COLORS[0]).bg(bg_color),
            Style::default().fg(COLORS[1]).bg(bg_color),
            Style::default().fg(COLORS[2]).bg(bg_color),
            Style::default().fg(COLORS[3]).bg(bg_color),
            Style::default().fg(COLORS[4]).bg(bg_color),
        ];

        for _row in 0..wave_height {
            let mut spans: Vec<Span> = Vec::with_capacity(width);

            for x in (0..width).rev() {
                let segment_idx = (x * 5) / width;
                let count = counts[segment_idx];
                let style = styles[segment_idx];
                
                let ch = wave_glyph(x, tick, count);
                spans.push(Span::styled(ch, style));
            }
            
            spans.reverse();
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, wave_area);
    }

    // Render text footer
    if text_height > 0 {
        let mut current_x = text_area.x;
        for (i, &w) in col_widths.iter().enumerate() {
            if w == 0 { continue; }
            
            let chunk = Rect {
                x: current_x,
                y: text_area.y,
                width: w,
                height: 1,
            };
            current_x += w;

            let is_active = counts[i] > 0;
            let mut text_style = Style::default().fg(COLORS[i]).bg(bg_color);
            if is_active {
                text_style = text_style.add_modifier(Modifier::BOLD);
            } else {
                text_style = text_style.add_modifier(Modifier::DIM);
            }

            let label_str = format!("{} {}", LABELS[i], counts[i]);
            let paragraph = Paragraph::new(label_str)
                .style(text_style)
                .alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(paragraph, chunk);
        }
    }
}

/// A ticker that manages the wave animation lifecycle.
#[derive(Debug)]
pub struct WaveTicker {
    tick: usize,
    active: bool,
}

impl WaveTicker {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tick: 0,
            active: false,
        }
    }

    pub fn advance(&mut self) -> usize {
        self.tick = self.tick.wrapping_add(1);
        if !self.active {
            engine::animation_start();
            self.active = true;
        }
        self.tick
    }

    pub fn stop(&mut self) {
        if self.active {
            engine::animation_end();
            self.active = false;
        }
        self.tick = 0;
    }

    #[must_use]
    pub fn current_tick(&self) -> usize {
        self.tick
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_glyph_returns_valid_char() {
        for x in 0..50 {
            for tick in [0, 16, 100, 500] {
                for count in [0, 10, 100, 1000] {
                    let ch = wave_glyph(x, tick, count);
                    assert!(WAVE_STR.contains(&ch), "unexpected glyph {ch:?}");
                }
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
    }
}
