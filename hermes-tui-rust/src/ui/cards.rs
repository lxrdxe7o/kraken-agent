//! Cards module - Tool/message card UI components
//!
//! This module provides card-based UI components for displaying tool results,
//! errors, warnings, and other special message types.

use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::state::config::ChatColorsRgb;

/// Card type for different message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    /// Information card (blue)
    Info,
    /// Success card (green)
    Success,
    /// Warning card (yellow)
    Warning,
    /// Error card (red)
    Error,
    /// Tool result card
    Tool,
}

/// Tool execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    /// Tool is currently running
    Running,
    /// Tool completed successfully
    Completed,
    /// Tool failed with an error
    Failed,
    /// Tool is pending execution
    Pending,
}

/// Tool card data for displaying tool execution details
#[derive(Debug, Clone)]
pub struct ToolCardData {
    /// Name of the tool
    pub tool_name: String,
    /// Current execution status
    pub status: ToolStatus,
    /// Execution duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Serialized input arguments
    pub arguments: Option<String>,
    /// Serialized output result
    pub result: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

impl ToolCardData {
    /// Create tool card data for a running tool
    pub fn running(tool_name: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            status: ToolStatus::Running,
            duration_ms: None,
            arguments: None,
            result: None,
            error: None,
        }
    }

    /// Mark the tool as completed with result
    pub fn completed(&mut self, result: impl Into<String>) {
        self.status = ToolStatus::Completed;
        self.result = Some(result.into());
    }

    /// Mark the tool as failed with error
    pub fn failed(&mut self, error: impl Into<String>) {
        self.status = ToolStatus::Failed;
        self.error = Some(error.into());
    }

    /// Set tool arguments
    pub fn with_arguments(mut self, arguments: impl Into<String>) -> Self {
        self.arguments = Some(arguments.into());
        self
    }

    /// Set duration in milliseconds
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}

/// Card component for displaying special messages
///
/// This component renders a styled card with a title, content, and appropriate
/// colors based on the card type.
#[derive(Debug, Clone)]
pub struct CardComponent {
    /// Card type (determines styling)
    card_type: CardType,
    /// Card title
    title: String,
    /// Card content
    content: String,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl CardComponent {
    /// Create a new card with the given type, title, and content
    pub fn new(card_type: CardType, title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self {
            card_type,
            title: title.into(),
            content: content.into(),
            colors,
        }
    }

    /// Create a new info card
    pub fn info(title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self::new(CardType::Info, title, content, colors)
    }

    /// Create a new success card
    pub fn success(title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self::new(CardType::Success, title, content, colors)
    }

    /// Create a new warning card
    pub fn warning(title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self::new(CardType::Warning, title, content, colors)
    }

    /// Create a new error card
    pub fn error(title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self::new(CardType::Error, title, content, colors)
    }

    /// Create a new tool card
    pub fn tool(title: impl Into<String>, content: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self::new(CardType::Tool, title, content, colors)
    }

    /// Create a new tool card from ToolCardData
    pub fn tool_card(data: &ToolCardData, colors: ChatColorsRgb) -> Self {
        let title = Self::tool_card_title(data);
        let content = Self::tool_card_content(data);
        Self::new(CardType::Tool, title, content, colors)
    }

    /// Build the title for a tool card
    fn tool_card_title(data: &ToolCardData) -> String {
        let status_icon = match data.status {
            ToolStatus::Running => " ▶ ",
            ToolStatus::Completed => " ✓ ",
            ToolStatus::Failed => " ✗ ",
            ToolStatus::Pending => " ○ ",
        };
        let duration = data.duration_ms.map_or(String::new(), |ms| format!(" ({:.1}s)", ms as f64 / 1000.0));
        format!("{} {}{}", status_icon, data.tool_name, duration)
    }

    /// Build the content for a tool card
    fn tool_card_content(data: &ToolCardData) -> String {
        let mut parts = Vec::new();

        if let Some(args) = &data.arguments {
            parts.push(format!("Args: {}", args));
        }

        match data.status {
            ToolStatus::Running => {
                parts.push("Running...".to_string());
            }
            ToolStatus::Completed => {
                if let Some(result) = &data.result {
                    // Truncate long results
                    if result.len() > 500 {
                        parts.push(format!("Result: {}...", &result[..497]));
                    } else {
                        parts.push(format!("Result: {}", result));
                    }
                }
            }
            ToolStatus::Failed => {
                if let Some(error) = &data.error {
                    parts.push(format!("Error: {}", error));
                } else {
                    parts.push("Failed (no error message)".to_string());
                }
            }
            ToolStatus::Pending => {
                parts.push("Waiting...".to_string());
            }
        }

        parts.join("\n")
    }
    /// Get the card type
    pub fn card_type(&self) -> CardType {
        self.card_type
    }

    /// Set the card type
    pub fn set_card_type(&mut self, card_type: CardType) {
        self.card_type = card_type;
    }

    /// Get the title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Get the content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set the content
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Set the colors
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
    }

    /// Get the appropriate border color for the card type
    fn border_color(&self) -> ratatui::style::Color {
        match self.card_type {
            CardType::Info => self.colors.assistant_text, // Use a distinct color for info
            CardType::Success => ratatui::style::Color::Green,
            CardType::Warning => ratatui::style::Color::Yellow,
            CardType::Error => ratatui::style::Color::Red,
            CardType::Tool => self.colors.tool_text,
        }
    }

    /// Get the appropriate background color for the card type
    fn bg_color(&self) -> ratatui::style::Color {
        match self.card_type {
            CardType::Info => self.colors.code_bg,
            CardType::Success => self.colors.code_bg,
            CardType::Warning => self.colors.code_bg,
            CardType::Error => self.colors.code_bg,
            CardType::Tool => self.colors.tool_bg,
        }
    }

    /// Get the appropriate text color for the card type
    fn text_color(&self) -> ratatui::style::Color {
        match self.card_type {
            CardType::Info => self.colors.assistant_text,
            CardType::Success => ratatui::style::Color::Green,
            CardType::Warning => ratatui::style::Color::Yellow,
            CardType::Error => ratatui::style::Color::Red,
            CardType::Tool => self.colors.tool_text,
        }
    }

    /// Render the card
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_color = self.border_color();
        
        // Create a block for the card
        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(border_color))
            .bg(self.bg_color());

        // Inner area
        let inner_area = block.inner(area);
        
        // Render the block
        frame.render_widget(block, area);

        // Create content text
        let content_text = Text::from(Line::from(Span::styled(
            &self.content,
            Style::new().fg(self.text_color()),
        )));
        
        // Create paragraph with content
        let paragraph = Paragraph::new(content_text)
            .style(Style::new().bg(self.bg_color()))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)));
        
        frame.render_widget(paragraph, inner_area);
    }
}

/// Card manager for handling multiple visible cards
#[derive(Debug, Clone)]
pub struct CardManager {
    /// List of active cards
    cards: Vec<CardComponent>,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl CardManager {
    /// Create a new card manager
    pub fn new(colors: ChatColorsRgb) -> Self {
        Self {
            cards: Vec::new(),
            colors,
        }
    }

    /// Create a new card manager with defaults
    pub fn with_defaults() -> Self {
        Self::new(ChatColorsRgb {
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
        })
    }

    /// Add a card
    pub fn add_card(&mut self, card: CardComponent) {
        self.cards.push(card);
    }

    /// Remove a card by index
    pub fn remove_card(&mut self, index: usize) -> Option<CardComponent> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    /// Clear all cards
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Get the number of cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if there are any cards
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Get all cards
    pub fn cards(&self) -> &[CardComponent] {
        &self.cards
    }

    /// Add an info card
    pub fn add_info(&mut self, title: impl Into<String>, content: impl Into<String>) {
        self.add_card(CardComponent::info(title, content, self.colors));
    }

    /// Add a success card
    pub fn add_success(&mut self, title: impl Into<String>, content: impl Into<String>) {
        self.add_card(CardComponent::success(title, content, self.colors));
    }

    /// Add a warning card
    pub fn add_warning(&mut self, title: impl Into<String>, content: impl Into<String>) {
        self.add_card(CardComponent::warning(title, content, self.colors));
    }

    /// Add an error card
    pub fn add_error(&mut self, title: impl Into<String>, content: impl Into<String>) {
        self.add_card(CardComponent::error(title, content, self.colors));
    }

    /// Add a tool card
    pub fn add_tool(&mut self, title: impl Into<String>, content: impl Into<String>) {
        self.add_card(CardComponent::tool(title, content, self.colors));
    }

    /// Add a tool card from ToolCardData
    pub fn add_tool_card(&mut self, data: ToolCardData) {
        self.add_card(CardComponent::tool_card(&data, self.colors));
    }

    /// Update the status of an existing tool card by tool name
    pub fn update_tool_status(
        &mut self,
        tool_name: &str,
        status: ToolStatus,
        result: Option<String>,
        error: Option<String>,
    ) -> bool {
        if let Some(card) = self.cards.iter_mut().find(|c| c.title().contains(tool_name)) {
            let data = ToolCardData {
                tool_name: tool_name.to_string(),
                status,
                duration_ms: None,
                arguments: None,
                result,
                error,
            };
            *card = CardComponent::tool_card(&data, self.colors);
            true
        } else {
            false
        }
    }

    /// Set the colors for all future cards
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
    }

    /// Update colors for all existing cards
    pub fn update_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
        for card in &mut self.cards {
            card.set_colors(colors);
        }
    }

    /// Render all cards in a stack (vertical layout)
    pub fn render_stack(&self, frame: &mut Frame, area: Rect) {
        if self.cards.is_empty() {
            return;
        }

        // Calculate card heights (each card gets equal height)
        let card_height = area.height / self.cards.len() as u16;
        
        for (i, card) in self.cards.iter().enumerate() {
            let card_area = Rect {
                x: area.x,
                y: area.y + (i as u16 * card_height),
                width: area.width,
                height: card_height,
            };
            card.render(frame, card_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_colors() -> ChatColorsRgb {
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
    fn test_card_new() {
        let colors = create_test_colors();
        let card = CardComponent::new(CardType::Info, "Test", "Content", colors);
        assert_eq!(card.card_type(), CardType::Info);
        assert_eq!(card.title(), "Test");
        assert_eq!(card.content(), "Content");
    }

    #[test]
    fn test_card_info() {
        let colors = create_test_colors();
        let card = CardComponent::info("Info", "Info content", colors);
        assert_eq!(card.card_type(), CardType::Info);
    }

    #[test]
    fn test_card_success() {
        let colors = create_test_colors();
        let card = CardComponent::success("Success", "Success content", colors);
        assert_eq!(card.card_type(), CardType::Success);
    }

    #[test]
    fn test_card_warning() {
        let colors = create_test_colors();
        let card = CardComponent::warning("Warning", "Warning content", colors);
        assert_eq!(card.card_type(), CardType::Warning);
    }

    #[test]
    fn test_card_error() {
        let colors = create_test_colors();
        let card = CardComponent::error("Error", "Error content", colors);
        assert_eq!(card.card_type(), CardType::Error);
    }

    #[test]
    fn test_card_tool() {
        let colors = create_test_colors();
        let card = CardComponent::tool("Tool", "Tool content", colors);
        assert_eq!(card.card_type(), CardType::Tool);
    }

    #[test]
    fn test_card_setters() {
        let colors = create_test_colors();
        let mut card = CardComponent::new(CardType::Info, "Test", "Content", colors);
        
        card.set_card_type(CardType::Success);
        assert_eq!(card.card_type(), CardType::Success);
        
        card.set_title("New Title");
        assert_eq!(card.title(), "New Title");
        
        card.set_content("New Content");
        assert_eq!(card.content(), "New Content");
    }

    #[test]
    fn test_card_manager_new() {
        let colors = create_test_colors();
        let manager = CardManager::new(colors);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_card_manager_add() {
        let colors = create_test_colors();
        let mut manager = CardManager::new(colors);
        
        manager.add_info("Info", "Content");
        assert_eq!(manager.len(), 1);
        
        manager.add_success("Success", "Content");
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_card_manager_remove() {
        let colors = create_test_colors();
        let mut manager = CardManager::new(colors);
        
        manager.add_info("Info", "Content");
        manager.add_success("Success", "Content");
        
        let removed = manager.remove_card(0);
        assert!(removed.is_some());
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_card_manager_clear() {
        let colors = create_test_colors();
        let mut manager = CardManager::new(colors);
        
        manager.add_info("Info", "Content");
        manager.add_success("Success", "Content");
        
        assert_eq!(manager.len(), 2);
        manager.clear();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_card_manager_with_defaults() {
        let manager = CardManager::with_defaults();
        assert!(manager.is_empty());
    }
}
