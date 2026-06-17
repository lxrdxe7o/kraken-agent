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

        // ── Pane Navigation ──
        lines.push(section("Pane Navigation  (Normal mode)"));
        lines.push(key_desc("h", "Focus pane left"));
        lines.push(key_desc("j", "Focus pane down"));
        lines.push(key_desc("k", "Focus pane up"));
        lines.push(key_desc("l", "Focus pane right"));
        lines.push(Line::from(""));

        // ── Pane Resize ──
        lines.push(section("Pane Resize"));
        lines.push(key_desc("Alt+←/→/↑/↓", "Resize pane 5 cells"));
        lines.push(Line::from(""));

        // ── Session Management ──
        lines.push(section("Session Management"));
        lines.push(key_desc("Ctrl+n", "New session"));
        lines.push(key_desc("Ctrl+r", "Resume last session"));
        lines.push(key_desc("Ctrl+l", "List sessions"));
        lines.push(key_desc("Ctrl+k", "Kill current session"));
        lines.push(key_desc("Ctrl+d", "Detach / close session"));
        lines.push(key_desc("R", "Rename session"));
        lines.push(key_desc("Ctrl+a", "Switch to session…"));
        lines.push(Line::from(""));

        // ── View Management ──
        lines.push(section("View / Window Management"));
        lines.push(key_desc("c", "New view"));
        lines.push(key_desc(",", "Rename view"));
        lines.push(key_desc("x", "Close pane"));
        lines.push(key_desc("&", "Kill view"));
        lines.push(Line::from(""));

        // ── Input Modes ──
        lines.push(section("Input Modes"));
        lines.push(key_desc("i", "Insert mode  (from Normal)"));
        lines.push(key_desc("Esc", "Normal mode  (from Insert / Cmd)"));
        lines.push(key_desc("/", "Command mode  (from Normal)"));
        lines.push(key_desc("Enter", "Submit prompt"));
        lines.push(Line::from(""));

        // ── Views (top bar) ──
        lines.push(section("Views"));
        lines.push(key_desc("1", "Dashboard"));
        lines.push(key_desc("2", "IDE"));
        lines.push(key_desc("3", "Kanban"));
        lines.push(key_desc("4", "Chat"));
        lines.push(Line::from(""));

        // ── Other ──
        lines.push(section("Other"));
        lines.push(key_desc("?", "Toggle this help screen"));
        lines.push(key_desc("r", "Reload config"));
        lines.push(key_desc("Ctrl+m", "Model picker"));
        lines.push(key_desc("Ctrl+c / q", "Quit"));

        // Footer hint
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            " Press ? or Esc to close ",
            Style::default().fg(DIM).add_modifier(Modifier::DIM),
        )]));

        let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .style(Style::default().bg(BG));

        frame.render_widget(paragraph, inner);
    }
}
