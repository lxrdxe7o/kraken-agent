//! Help overlay — TUI keybinding reference
//!
//! Renders a centered overlay showing all active keyboard shortcuts,
//! matching the user's tmux configuration conventions.

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Help overlay — stateless keybinding reference.
pub struct HelpOverlay;

const DIM: Color = Color::Rgb(146, 131, 116);    // gruvbox gray
const YELLOW: Color = Color::Rgb(250, 189, 47);   // gruvbox yellow
const GREEN: Color = Color::Rgb(184, 187, 38);     // gruvbox green
const LIGHT: Color = Color::Rgb(235, 219, 178);    // gruvbox light
const BG: Color = Color::Rgb(40, 40, 40);           // gruvbox dark bg

impl HelpOverlay {
    /// Render the help overlay centered over `area`.
    pub fn render(frame: &mut Frame, area: Rect) {
        if area.width < 50 || area.height < 24 {
            return;
        }

        let width = area.width.min(74);
        let height = area.height.min(36);
        let x = area.width.saturating_sub(width) / 2;
        let y = area.height.saturating_sub(height) / 2;
        let help_area = Rect::new(x, y, width, height);

        // Clear background beneath the overlay
        frame.render_widget(Clear, help_area);

        let block = Block::default()
            .title(" ⌨️  Hermes TUI — Keybindings ")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(YELLOW))
            .style(Style::default().bg(BG));

        let inner = block.inner(help_area);
        frame.render_widget(block, help_area);

        let mut lines: Vec<Line> = Vec::new();

        fn key_desc(key: &'static str, desc: &'static str) -> Line<'static> {
            Line::from(vec![
                Span::styled(format!(" {:15}", key), Style::default().fg(GREEN)),
                Span::styled(desc, Style::default().fg(LIGHT)),
            ])
        }

        fn section(title: &str) -> Line<'static> {
            Line::from(Span::styled(
                format!(" {} ", title),
                Style::default()
                    .fg(YELLOW)
                    .add_modifier(Modifier::BOLD),
            ))
        }

        // ── Prefix Key ──
        lines.push(section("Prefix Key  (tmux-style)"));

        let prefix_line = Line::from(vec![
            Span::styled("  Alt+A  ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("then command key  —  ", Style::default().fg(DIM)),
            Span::styled("Esc", Style::default().fg(GREEN)),
            Span::styled(" cancels prefix", Style::default().fg(DIM)),
        ]);
        lines.push(prefix_line);
        lines.push(Line::from(""));
        lines.push(key_desc("Alt+A then ?", "Toggle this help screen"));
        lines.push(key_desc("Esc (from help)", "Close help"));
        lines.push(Line::from(""));

        // ── Pane Navigation ──
        lines.push(section("Pane Navigation"));

        let nav_prefix = Line::from(vec![
            Span::styled("  (prefix) ", Style::default().fg(DIM)),
            Span::styled("h/j/k/l", Style::default().fg(GREEN)),
            Span::styled("  —  Focus panes", Style::default().fg(LIGHT)),
        ]);
        lines.push(nav_prefix);
        lines.push(key_desc("Alt+←/→/↑/↓", "Resize pane 5 cells"));
        lines.push(Line::from(""));

        // ── Session Management ──
        lines.push(section("Session Management"));
        lines.push(key_desc("Ctrl+n", "New session"));
        lines.push(key_desc("Ctrl+r", "Resume last session"));
        lines.push(key_desc("Ctrl+l", "List sessions"));
        lines.push(key_desc("Ctrl+k", "Kill current session"));
        lines.push(key_desc("Ctrl+d", "Detach / close session"));
        lines.push(key_desc("Ctrl+a", "Switch to session…"));
        lines.push(key_desc("(prefix) R", "Rename session"));
        lines.push(key_desc("(prefix) r", "Reload config"));
        lines.push(Line::from(""));

        // ── View / Window Management (prefix) ──
        lines.push(section("View / Window Management  (prefix)"));
        lines.push(key_desc("(prefix) 1…4", "Switch views  (1=Dashboard, 4=Chat)"));
        lines.push(key_desc("(prefix) c", "New view"));
        lines.push(key_desc("(prefix) ,", "Rename view"));
        lines.push(key_desc("(prefix) \"", "New view (split)"));
        lines.push(key_desc("(prefix) x", "Close pane"));
        lines.push(key_desc("(prefix) &", "Kill view"));
        lines.push(Line::from(""));

        // ── Input Modes ──
        lines.push(section("Input Modes"));
        lines.push(key_desc("i", "Insert mode  (from Normal)"));
        lines.push(key_desc("Esc", "Normal mode  (from Insert / Cmd)"));
        lines.push(key_desc("/", "Command mode  (from Normal)"));
        lines.push(key_desc("Enter", "Submit prompt"));
        lines.push(Line::from(""));

        // ── Other ──
        lines.push(section("Other"));
        lines.push(key_desc("Ctrl+m", "Model picker"));
        lines.push(key_desc("Ctrl+c / q", "Quit"));

        // Footer hint
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            " Press Alt+A ? or Esc to close ",
            Style::default().fg(DIM).add_modifier(Modifier::DIM),
        )]));

        let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .style(Style::default().bg(BG));

        frame.render_widget(paragraph, inner);
    }
}
