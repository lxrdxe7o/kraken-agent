//! Chat module - Chat transcript UI component
//!
//! This module provides the chat display component for rendering conversation messages.

use chrono::{DateTime, Utc};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    symbols::scrollbar,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::protocol::types::MessageRole;
use crate::state::{config::ChatColorsRgb, messages::Message};

/// Chat component for displaying conversation messages
///
/// This component renders a scrollable chat transcript with proper
/// formatting, colors, and timestamps.
#[derive(Debug, Clone)]
pub struct ChatComponent {
    /// Messages to display
    messages: Vec<Message>,
    /// Current scroll position (in message height units)
    scroll_position: u16,
    /// Visible height of the chat area in lines
    visible_height: u16,
    /// Chat colors from configuration
    colors: ChatColorsRgb,
    /// Whether to show timestamps
    show_timestamps: bool,
}

impl ChatComponent {
    /// Create a new chat component with the given colors and timestamp setting
    pub fn new(colors: ChatColorsRgb, show_timestamps: bool) -> Self {
        Self {
            messages: Vec::new(),
            scroll_position: 0,
            visible_height: 0,
            colors,
            show_timestamps,
        }
    }

    /// Create a new chat component with all defaults
    pub fn with_defaults() -> Self {
        Self::new(
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
            true,
        )
    }

    /// Set the messages to display
    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    /// Set the visible height of the chat area
    pub fn set_visible_height(&mut self, height: u16) {
        self.visible_height = height;
    }

    /// Scroll down by the given amount (in lines)
    pub fn scroll_down(&mut self, amount: u16) {
        let max = self.max_scroll_position();
        self.scroll_position = self.scroll_position.saturating_add(amount).min(max);
    }

    /// Scroll up by the given amount (in lines)
    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_position = self.scroll_position.saturating_sub(amount);
    }

    /// Scroll to the bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_position = self.max_scroll_position();
    }

    /// Scroll to the top
    pub fn scroll_to_top(&mut self) {
        self.scroll_position = 0;
    }

    /// Get the maximum scroll position
    pub fn max_scroll_position(&self) -> u16 {
        let total_height: u16 = self
            .messages
            .iter()
            .map(|m| self.message_height(m))
            .sum();
        total_height.saturating_sub(self.visible_height)
    }

    /// Calculate the height of a message in lines
    pub fn message_height(&self, message: &Message) -> u16 {
        // Count lines in content
        let content_lines = message.content.lines().count() as u16;
        
        // Add role/timestamp line
        let header_lines = if self.show_timestamps { 1 } else { 0 } + 1;
        
        // Add spacing
        header_lines + content_lines + 1
    }

    /// Get the style for a message role
    pub fn get_role_style(&self, role: MessageRole) -> Style {
        match role {
            MessageRole::User => Style::new()
                .bg(self.colors.user_bg)
                .fg(self.colors.user_text),
            MessageRole::Assistant => Style::new()
                .bg(self.colors.assistant_bg)
                .fg(self.colors.assistant_text),
            MessageRole::System => Style::new()
                .bg(self.colors.system_bg)
                .fg(self.colors.system_text),
            MessageRole::Tool => Style::new()
                .bg(self.colors.tool_bg)
                .fg(self.colors.tool_text),
        }
    }

    /// Get the display string for a message role
    pub fn get_role_display(&self, role: MessageRole) -> &'static str {
        match role {
            MessageRole::User => "User",
            MessageRole::Assistant => "Assistant",
            MessageRole::System => "System",
            MessageRole::Tool => "Tool",
        }
    }

    /// Format a timestamp as a relative time string
    pub fn format_timestamp(&self, timestamp: DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = now - timestamp;

        if duration.num_seconds() < 60 {
            format!("{}s ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{}m ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{}h ago", duration.num_hours())
        } else {
            format!("{}d ago", duration.num_days())
        }
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Add a single message
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        // Auto-scroll to bottom when new message is added
        self.scroll_to_bottom();
    }
    
    /// Update an existing message (for streaming deltas)
    pub fn update_message(&mut self, updated_message: Message) {
        if let Some(index) = self.messages.iter().position(|m| m.message_id == updated_message.message_id) {
            self.messages[index] = updated_message;
        }
    }

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.scroll_position = 0;
    }

    /// Set all messages at once
    pub fn set_messages(&mut self, messages: Vec<Message>) {
        self.messages = messages;
        self.scroll_to_bottom();
    }

    /// Render the chat component
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.messages.is_empty() {
            self.render_empty(frame, area);
            return;
        }

        // Create a block for the chat area
        let block = Block::default()
            .title(" Chat ".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(self.colors.border));

        // Inner area for messages
        let inner_area = block.inner(area);

        // Calculate scroll state
        let total_height: u16 = self
            .messages
            .iter()
            .map(|m| self.message_height(m))
            .sum();
        
        let mut scroll_state = ScrollbarState::new(total_height as usize)
            .position(self.scroll_position as usize);

        // Render the main block
        frame.render_widget(block, area);

        // Find starting message based on scroll position
        let mut current_height = 0u16;
        let mut start_idx = 0usize;
        
        for (i, msg) in self.messages.iter().enumerate() {
            let msg_height = self.message_height(msg);
            if current_height + msg_height > self.scroll_position {
                start_idx = i;
                break;
            }
            current_height += msg_height;
        }

        // Render visible messages
        let mut y_offset = 0u16;
        
        for (_i, message) in self.messages.iter().enumerate().skip(start_idx) {
            if y_offset >= self.visible_height {
                break;
            }
            
            let msg_height = self.message_height(message);
            
            // Check if this message is above the visible area
            if y_offset + msg_height < self.scroll_position {
                y_offset += msg_height;
                continue;
            }
            
            // Calculate message area
            let msg_area = Rect {
                x: inner_area.x,
                y: inner_area.y + y_offset - self.scroll_position,
                width: inner_area.width,
                height: msg_height.min(inner_area.height - (y_offset - self.scroll_position)),
            };
            
            if msg_area.height == 0 {
                y_offset += msg_height;
                continue;
            }
            
            // Render the message
            self.render_message(frame, msg_area, message);
            
            y_offset += msg_height;
        }

        // Render scrollbar
        if total_height > inner_area.height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .symbols(scrollbar::VERTICAL)
                .style(Style::new().fg(self.colors.border))
                .track_symbol(Some("│"))
                .thumb_symbol("█");
            
            frame.render_stateful_widget(
                scrollbar,
                area,
                &mut scroll_state,
            );
        }
    }

    /// Render a single message
    fn render_message(&self, frame: &mut Frame, area: Rect, message: &Message) {
        let role_style = self.get_role_style(message.role.clone());
        let role_display = self.get_role_display(message.role.clone());
        
        let mut lines = Vec::new();
        
        // Header line: role indicator and timestamp
        if self.show_timestamps {
            let timestamp = self.format_timestamp(message.timestamp);
            let header_span = Span::styled(
                format!("[{}] {} ", timestamp, role_display),
                role_style,
            );
            lines.push(Line::from(header_span));
        } else {
            let header_span = Span::styled(
                format!("{} ", role_display),
                role_style,
            );
            lines.push(Line::from(header_span));
        }
        
        // Content lines
        for line in message.content.lines() {
            lines.push(Line::from(Span::raw(line)));
        }
        
        // Add empty line between messages
        lines.push(Line::from(Span::raw("")));
        
        // Create paragraph with all lines
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::new().fg(self.colors.code_text))
            .block(Block::new().padding(Padding::horizontal(1)));
        
        frame.render_widget(paragraph, area);
    }

    /// Render empty state
    fn render_empty(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Chat ".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(self.colors.border));

        let inner_area = block.inner(area);
        
        frame.render_widget(block, area);
        
        let empty_text = Text::from(Line::from(Span::styled(
            "No messages yet. Start a conversation!",
            Style::new().fg(self.colors.timestamp).dim(),
        )));
        
        let empty_para = Paragraph::new(empty_text)
            .centered();
        
        frame.render_widget(empty_para, inner_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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

    fn create_test_message(role: MessageRole, content: &str) -> Message {
        Message::new(role, content)
    }

    #[test]
    fn test_new() {
        let colors = create_test_colors();
        let chat = ChatComponent::new(colors, true);
        assert!(chat.messages().is_empty());
        assert_eq!(chat.scroll_position, 0);
    }

    #[test]
    fn test_role_style() {
        let colors = create_test_colors();
        let chat = ChatComponent::new(colors, true);

        let user_style = chat.get_role_style(MessageRole::User);
        assert!(user_style.bg == Some(ratatui::style::Color::Indexed(238)));
        assert!(user_style.fg == Some(ratatui::style::Color::Indexed(252)));
    }

    #[test]
    fn test_role_display() {
        let colors = create_test_colors();
        let chat = ChatComponent::new(colors, false);

        assert_eq!(chat.get_role_display(MessageRole::User), "User");
        assert_eq!(chat.get_role_display(MessageRole::Assistant), "Assistant");
        assert_eq!(chat.get_role_display(MessageRole::System), "System");
        assert_eq!(chat.get_role_display(MessageRole::Tool), "Tool");
    }

    #[test]
    fn test_message_height() {
        let colors = create_test_colors();
        let mut chat = ChatComponent::new(colors, true);
        chat.set_visible_height(80);

        let msg = create_test_message(MessageRole::User, "Hello");
        assert!(chat.message_height(&msg) >= 2);
    }

    #[test]
    fn test_add_message() {
        let colors = create_test_colors();
        let mut chat = ChatComponent::new(colors, true);
        
        let msg = create_test_message(MessageRole::User, "Hello");
        chat.add_message(msg);
        
        assert_eq!(chat.messages().len(), 1);
    }

    #[test]
    fn test_scroll() {
        let colors = create_test_colors();
        let mut chat = ChatComponent::new(colors, false);
        chat.set_visible_height(10);
        
        for i in 0..20 {
            chat.add_message(create_test_message(MessageRole::User, &format!("Message {}", i)));
        }
        
        assert!(chat.max_scroll_position() > 0);
        
        chat.scroll_to_bottom();
        assert_eq!(chat.scroll_position, chat.max_scroll_position());
        
        chat.scroll_to_top();
        assert_eq!(chat.scroll_position, 0);
        
        chat.scroll_down(5);
        assert!(chat.scroll_position >= 5);
        
        chat.scroll_up(3);
        assert!(chat.scroll_position >= 2);
    }

    #[test]
    fn test_clear_messages() {
        let colors = create_test_colors();
        let mut chat = ChatComponent::new(colors, true);
        
        chat.add_message(create_test_message(MessageRole::User, "Hello"));
        chat.add_message(create_test_message(MessageRole::Assistant, "World"));
        
        assert_eq!(chat.messages().len(), 2);
        
        chat.clear_messages();
        
        assert!(chat.messages().is_empty());
        assert_eq!(chat.scroll_position, 0);
    }

    #[test]
    fn test_set_messages() {
        let colors = create_test_colors();
        let mut chat = ChatComponent::new(colors, true);
        
        let messages = vec![
            create_test_message(MessageRole::User, "Hello"),
            create_test_message(MessageRole::Assistant, "World"),
        ];
        
        chat.set_messages(messages);
        
        assert_eq!(chat.messages().len(), 2);
    }

    #[test]
    fn test_format_timestamp() {
        let colors = create_test_colors();
        let chat = ChatComponent::new(colors, true);
        
        // Create a timestamp from 5 seconds ago
        let timestamp = Utc::now() - chrono::Duration::seconds(5);
        let result = chat.format_timestamp(timestamp);
        
        // Should contain "s ago"
        assert!(result.contains("s ago"));
    }

    #[test]
    fn test_with_defaults() {
        let chat = ChatComponent::with_defaults();
        assert!(chat.messages().is_empty());
        assert!(chat.show_timestamps);
    }

    #[test]
    fn test_with_messages() {
        let colors = create_test_colors();
        let messages = vec![
            create_test_message(MessageRole::User, "Hello"),
            create_test_message(MessageRole::Assistant, "World"),
        ];
        
        let chat = ChatComponent::new(colors, true).with_messages(messages);
        
        assert_eq!(chat.messages().len(), 2);
    }
}
