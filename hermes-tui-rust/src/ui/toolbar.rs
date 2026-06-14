//! Toolbar module - Status bar UI component
//!
//! This module provides the status bar component for displaying information
//! like the current model, session, input mode, etc.

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::state::config::{ChatColorsRgb, InputMode, ThemeColorsRgb};

/// Status bar item for displaying information
#[derive(Debug, Clone)]
pub struct ToolbarItem {
    /// Display text
    text: String,
    /// Style for this item
    style: Style,
}

impl ToolbarItem {
    /// Create a new toolbar item
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    /// Create a new toolbar item with default style
    pub fn with_defaults(text: impl Into<String>) -> Self {
        Self::new(text, Style::new())
    }
}

/// Toolbar component for displaying status information
///
/// This component shows information like:
/// - Current model
/// - Current session
/// - Input mode
/// - Connection status
/// - Time
#[derive(Debug, Clone)]
pub struct Toolbar {
    /// List of items to display (left to right)
    items: Vec<ToolbarItem>,
    /// Theme colors from configuration
    theme_colors: ThemeColorsRgb,
    /// Chat colors from configuration (for border)
    chat_colors: ChatColorsRgb,
    /// Current input mode
    input_mode: InputMode,
}

impl Toolbar {
    /// Create a new toolbar with the given colors
    pub fn new(theme_colors: ThemeColorsRgb, chat_colors: ChatColorsRgb) -> Self {
        Self {
            items: Vec::new(),
            theme_colors,
            chat_colors,
            input_mode: InputMode::Normal,
        }
    }

    /// Create a new toolbar with all defaults
    pub fn with_defaults() -> Self {
        Self::new(
            ThemeColorsRgb {
                background: ratatui::style::Color::Reset,
                text: ratatui::style::Color::Reset,
                primary: ratatui::style::Color::Indexed(13),
                secondary: ratatui::style::Color::Indexed(14),
                accent: ratatui::style::Color::Indexed(11),
                error: ratatui::style::Color::Red,
                success: ratatui::style::Color::Green,
                warning: ratatui::style::Color::Yellow,
            },
            ChatColorsRgb {
                user_bg: ratatui::style::Color::Indexed(238),
                user_text: ratatui::style::Color::Indexed(252),
                assistant_bg: ratatui::style::Color::Indexed(236),
                assistant_text: ratatui::style::Color::Indexed(248),
                system_bg: ratatui::style::Color::Indexed(235),
                system_text: ratatui::style::Color::Indexed(245),
                tool_bg: ratatui::style::Color::Indexed(237),
                tool_text: ratatui::style::Color::Indexed(243),
                code_bg: ratatui::style::Color::Indexed(233),
                code_text: ratatui::style::Color::Indexed(252),
                border: ratatui::style::Color::Indexed(240),
                timestamp: ratatui::style::Color::Indexed(246),
            },
        )
    }

    /// Set the input mode
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    /// Set the theme colors
    pub fn set_theme_colors(&mut self, colors: ThemeColorsRgb) {
        self.theme_colors = colors;
    }

    /// Set the chat colors
    pub fn set_chat_colors(&mut self, colors: ChatColorsRgb) {
        self.chat_colors = colors;
    }

    /// Add an item to the toolbar
    pub fn add_item(&mut self, item: ToolbarItem) {
        self.items.push(item);
    }

    /// Add a text item with the given style
    pub fn add_text(&mut self, text: impl Into<String>, style: Style) {
        self.items.push(ToolbarItem::new(text, style));
    }

    /// Add a text item with default style
    pub fn add_default_text(&mut self, text: impl Into<String>) {
        self.items.push(ToolbarItem::with_defaults(text));
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get the input mode
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    /// Update the status based on connection state and current mode
    pub fn update_status(&mut self, connected: bool, model: Option<&str>, session: Option<&str>) {
        self.items.clear();
        
        // Add connection status
        if connected {
            self.add_text(
                "● ".to_string(),
                Style::new().fg(self.theme_colors.success),
            );
        } else {
            self.add_text(
                "○ ".to_string(),
                Style::new().fg(self.theme_colors.error),
            );
        }
        
        // Add model name
        if let Some(model_name) = model {
            self.add_text(
                format!("Model: {} ", model_name),
                Style::new().fg(self.theme_colors.text),
            );
        }
        
        // Add session name
        if let Some(session_name) = session {
            self.add_text(
                format!("Session: {} ", session_name),
                Style::new().fg(self.theme_colors.text),
            );
        }
        
        // Add input mode
        let mode_text = match self.input_mode {
            InputMode::Normal => "Normal",
            InputMode::Insert => "Insert",
            InputMode::Command => "Command",
        };
        self.add_text(
            format!("Mode: {} ", mode_text),
            Style::new().fg(self.theme_colors.accent),
        );
    }

    /// Render the toolbar
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create a block for the toolbar
        let block = Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Plain)
            .border_style(Style::new().fg(self.chat_colors.border));

        // Inner area
        let inner_area = block.inner(area);
        
        // Render the block
        frame.render_widget(block, area);

        // Build the text line
        let mut spans = Vec::new();
        for item in &self.items {
            spans.push(Span::styled(item.text.clone(), item.style));
        }
        
        // Create paragraph with all spans
        let paragraph = Paragraph::new(Text::from(Line::from(spans)))
            .style(Style::new().fg(self.theme_colors.text))
            .alignment(Alignment::Left)
            .block(Block::new().padding(Padding::horizontal(1)));
        
        frame.render_widget(paragraph, inner_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_theme_colors() -> ThemeColorsRgb {
        ThemeColorsRgb {
            background: ratatui::style::Color::Reset,
            text: ratatui::style::Color::Indexed(252),
            primary: ratatui::style::Color::Indexed(13),
            secondary: ratatui::style::Color::Indexed(14),
            accent: ratatui::style::Color::Indexed(11),
            error: ratatui::style::Color::Red,
            success: ratatui::style::Color::Green,
            warning: ratatui::style::Color::Yellow,
        }
    }

    fn create_test_chat_colors() -> ChatColorsRgb {
        ChatColorsRgb {
            user_bg: ratatui::style::Color::Indexed(238),
            user_text: ratatui::style::Color::Indexed(252),
            assistant_bg: ratatui::style::Color::Indexed(236),
            assistant_text: ratatui::style::Color::Indexed(248),
            system_bg: ratatui::style::Color::Indexed(235),
            system_text: ratatui::style::Color::Indexed(245),
            tool_bg: ratatui::style::Color::Indexed(237),
            tool_text: ratatui::style::Color::Indexed(243),
            code_bg: ratatui::style::Color::Indexed(233),
            code_text: ratatui::style::Color::Indexed(252),
            border: ratatui::style::Color::Indexed(240),
            timestamp: ratatui::style::Color::Indexed(246),
        }
    }

    #[test]
    fn test_toolbar_new() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let toolbar = Toolbar::new(theme_colors, chat_colors);
        assert!(toolbar.items.is_empty());
    }

    #[test]
    fn test_toolbar_with_defaults() {
        let toolbar = Toolbar::with_defaults();
        assert!(toolbar.items.is_empty());
    }

    #[test]
    fn test_toolbar_add_item() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let mut toolbar = Toolbar::new(theme_colors, chat_colors);
        
        let item = ToolbarItem::new("Test", Style::new());
        toolbar.add_item(item);
        
        assert_eq!(toolbar.items.len(), 1);
    }

    #[test]
    fn test_toolbar_add_text() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let mut toolbar = Toolbar::new(theme_colors, chat_colors);
        
        toolbar.add_text("Test", Style::new().fg(ratatui::style::Color::Red));
        
        assert_eq!(toolbar.items.len(), 1);
    }

    #[test]
    fn test_toolbar_clear() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let mut toolbar = Toolbar::new(theme_colors, chat_colors);
        
        toolbar.add_default_text("Test 1");
        toolbar.add_default_text("Test 2");
        assert_eq!(toolbar.items.len(), 2);
        
        toolbar.clear();
        assert!(toolbar.items.is_empty());
    }

    #[test]
    fn test_toolbar_input_mode() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let mut toolbar = Toolbar::new(theme_colors, chat_colors);
        
        assert_eq!(toolbar.input_mode(), InputMode::Normal);
        
        toolbar.set_input_mode(InputMode::Insert);
        assert_eq!(toolbar.input_mode(), InputMode::Insert);
        
        toolbar.set_input_mode(InputMode::Command);
        assert_eq!(toolbar.input_mode(), InputMode::Command);
    }

    #[test]
    fn test_toolbar_update_status() {
        let theme_colors = create_test_theme_colors();
        let chat_colors = create_test_chat_colors();
        let mut toolbar = Toolbar::new(theme_colors, chat_colors);
        
        toolbar.update_status(true, Some("gpt-4"), Some("session-1"));
        
        // Should have connection indicator + model + session + mode
        assert!(toolbar.items.len() >= 4);
    }

    #[test]
    fn test_toolbar_item_new() {
        let item = ToolbarItem::new("Test", Style::new());
        assert_eq!(item.text, "Test");
    }

    #[test]
    fn test_toolbar_item_with_defaults() {
        let item = ToolbarItem::with_defaults("Test");
        assert_eq!(item.text, "Test");
    }
}
