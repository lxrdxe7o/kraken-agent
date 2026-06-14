//! Prompts module - Approval/clarify/sudo prompt UI components
//!
//! This module provides UI components for handling various prompt types that
//! require user interaction, such as approval requests, clarifications, etc.

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::state::config::ChatColorsRgb;

/// Prompt type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptType {
    /// Approval prompt (yes/no)
    Approval,
    /// Clarification prompt (user input)
    Clarify,
    /// Sudo/secret prompt (hidden input)
    Secret,
}

/// Prompt state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptState {
    /// Prompt is waiting for user input
    Pending,
    /// User approved
    Approved,
    /// User denied/rejected
    Denied,
}

/// Approval prompt component
///
/// This component displays a prompt that requires user approval (yes/no).
#[derive(Debug, Clone)]
pub struct ApprovalPrompt {
    /// Prompt message
    message: String,
    /// Optional tool name
    tool_name: Option<String>,
    /// Prompt state
    state: PromptState,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl ApprovalPrompt {
    /// Create a new approval prompt
    pub fn new(message: impl Into<String>, tool_name: Option<String>, colors: ChatColorsRgb) -> Self {
        Self {
            message: message.into(),
            tool_name,
            state: PromptState::Pending,
            colors,
        }
    }

    /// Create a new approval prompt with defaults
    pub fn with_defaults(message: impl Into<String>) -> Self {
        Self::new(
            message,
            None,
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

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Get the tool name
    pub fn tool_name(&self) -> Option<&String> {
        self.tool_name.as_ref()
    }

    /// Set the tool name
    pub fn set_tool_name(&mut self, tool_name: Option<String>) {
        self.tool_name = tool_name;
    }

    /// Get the state
    pub fn state(&self) -> PromptState {
        self.state
    }

    /// Set the state
    pub fn set_state(&mut self, state: PromptState) {
        self.state = state;
    }

    /// Approve the prompt
    pub fn approve(&mut self) {
        self.state = PromptState::Approved;
    }

    /// Deny the prompt
    pub fn deny(&mut self) {
        self.state = PromptState::Denied;
    }

    /// Check if approved
    pub fn is_approved(&self) -> bool {
        self.state == PromptState::Approved
    }

    /// Check if denied
    pub fn is_denied(&self) -> bool {
        self.state == PromptState::Denied
    }

    /// Check if pending
    pub fn is_pending(&self) -> bool {
        self.state == PromptState::Pending
    }

    /// Set the colors
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
    }

    /// Render the approval prompt
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.state {
            PromptState::Approved => Style::new().fg(Color::Green),
            PromptState::Denied => Style::new().fg(Color::Red),
            PromptState::Pending => Style::new().fg(self.colors.border),
        };

        // Create a block for the prompt
        let block = Block::default()
            .title(" Approval Required ".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style);

        // Inner area
        let inner_area = block.inner(area);
        
        // Render the block
        frame.render_widget(block, area);

        // Build content
        let mut lines = Vec::new();
        
        // Add tool name if present
        if let Some(tool) = &self.tool_name {
            lines.push(Line::from(Span::styled(
                format!("Tool: {} ", tool),
                Style::new().fg(self.colors.tool_text).bold(),
            )));
        }
        
        // Add message
        lines.push(Line::from(Span::raw(&self.message)));
        
        // Add instructions
        match self.state {
            PromptState::Pending => {
                lines.push(Line::from(Span::raw("")));
                lines.push(Line::from(Span::styled(
                    "Press Y to approve, N to deny",
                    Style::new().fg(self.colors.timestamp).dim(),
                )));
            }
            PromptState::Approved => {
                lines.push(Line::from(Span::styled(
                    "APPROVED",
                    Style::new().fg(ratatui::style::Color::Green).bold(),
                )));
            }
            PromptState::Denied => {
                lines.push(Line::from(Span::styled(
                    "DENIED",
                    Style::new().fg(ratatui::style::Color::Red).bold(),
                )));
            }
        }

        // Create paragraph with content
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::new().fg(self.colors.code_text))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)))
            .alignment(Alignment::Left);
        
        frame.render_widget(paragraph, inner_area);
    }
}

/// Clarification prompt component
///
/// This component displays a prompt that requires user text input.
#[derive(Debug, Clone)]
pub struct ClarifyPrompt {
    /// Prompt message
    message: String,
    /// User's response
    response: String,
    /// Whether the prompt is active
    active: bool,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl ClarifyPrompt {
    /// Create a new clarification prompt
    pub fn new(message: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self {
            message: message.into(),
            response: String::new(),
            active: true,
            colors,
        }
    }

    /// Create a new clarification prompt with defaults
    pub fn with_defaults(message: impl Into<String>) -> Self {
        Self::new(
            message,
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

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Get the response
    pub fn response(&self) -> &str {
        &self.response
    }

    /// Set the response
    pub fn set_response(&mut self, response: impl Into<String>) {
        self.response = response.into();
    }

    /// Append to response
    pub fn append_response(&mut self, text: &str) {
        self.response.push_str(text);
    }

    /// Clear the response
    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Set the colors
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
    }

    /// Submit the response
    pub fn submit(&mut self) -> String {
        let response = self.response.clone();
        self.response.clear();
        self.active = false;
        response
    }

    /// Cancel the prompt
    pub fn cancel(&mut self) {
        self.response.clear();
        self.active = false;
    }

    /// Render the clarification prompt
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_color = if self.active {
            self.colors.border
        } else {
            // Manually darken the color for inactive state
            match self.colors.border {
                Color::Rgb(r, g, b) => Color::Rgb(
                    (r / 2).min(255),
                    (g / 2).min(255),
                    (b / 2).min(255),
                ),
                Color::Indexed(i) => Color::Indexed(i.saturating_sub(8)),
                other => other,
            }
        };
        let border_style = Style::new().fg(border_color);

        // Create a block for the prompt
        let block = Block::default()
            .title(" Clarification Needed ".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style);

        // Inner area
        let inner_area = block.inner(area);
        
        // Render the block
        frame.render_widget(block, area);

        // Build content
        let mut lines = Vec::new();
        
        // Add message
        lines.push(Line::from(Span::raw(&self.message)));
        lines.push(Line::from(Span::raw("")));
        
        // Add response
        let response_text = if self.response.is_empty() {
            "Enter your response..."
        } else {
            &self.response
        };
        lines.push(Line::from(Span::styled(
            response_text,
            Style::new().fg(self.colors.code_text),
        )));

        // Create paragraph with content
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::new().fg(self.colors.code_text))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)))
            .alignment(Alignment::Left);
        
        frame.render_widget(paragraph, inner_area);
    }
}

/// Secret prompt component (for sudo/password input)
///
/// This component displays a prompt that hides the user's input.
#[derive(Debug, Clone)]
pub struct SecretPrompt {
    /// Prompt message
    message: String,
    /// Hidden input
    secret: String,
    /// Display text (masks the secret)
    display_text: String,
    /// Whether the prompt is active
    active: bool,
    /// Mask character
    mask_char: char,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl SecretPrompt {
    /// Create a new secret prompt
    pub fn new(message: impl Into<String>, colors: ChatColorsRgb) -> Self {
        Self {
            message: message.into(),
            secret: String::new(),
            display_text: String::new(),
            active: true,
            mask_char: '*',
            colors,
        }
    }

    /// Create a new secret prompt with defaults
    pub fn with_defaults(message: impl Into<String>) -> Self {
        Self::new(
            message,
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

    /// Get the message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Get the secret (use with caution!)
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Append to secret
    pub fn append_secret(&mut self, c: char) {
        self.secret.push(c);
        self.display_text.push(self.mask_char);
    }

    /// Remove last character from secret
    pub fn pop_secret(&mut self) {
        self.secret.pop();
        self.display_text.pop();
    }

    /// Clear the secret
    pub fn clear_secret(&mut self) {
        self.secret.clear();
        self.display_text.clear();
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Set the mask character
    pub fn set_mask_char(&mut self, mask_char: char) {
        self.mask_char = mask_char;
    }

    /// Set the colors
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
    }

    /// Submit the secret
    pub fn submit(&mut self) -> String {
        let secret = self.secret.clone();
        self.secret.clear();
        self.display_text.clear();
        self.active = false;
        secret
    }

    /// Cancel the prompt
    pub fn cancel(&mut self) {
        self.secret.clear();
        self.display_text.clear();
        self.active = false;
    }

    /// Render the secret prompt
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_color = if self.active {
            self.colors.border
        } else {
            // Manually darken the color for inactive state
            match self.colors.border {
                Color::Rgb(r, g, b) => Color::Rgb(
                    (r / 2).min(255),
                    (g / 2).min(255),
                    (b / 2).min(255),
                ),
                Color::Indexed(i) => Color::Indexed(i.saturating_sub(8)),
                other => other,
            }
        };
        let border_style = Style::new().fg(border_color);

        // Create a block for the prompt
        let block = Block::default()
            .title(" Secure Input ".bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style);

        // Inner area
        let inner_area = block.inner(area);
        
        // Render the block
        frame.render_widget(block, area);

        // Build content
        let mut lines = Vec::new();
        
        // Add message
        lines.push(Line::from(Span::raw(&self.message)));
        lines.push(Line::from(Span::raw("")));
        
        // Add masked input
        let input_text = if self.display_text.is_empty() {
            "Enter secure input..."
        } else {
            &self.display_text
        };
        lines.push(Line::from(Span::styled(
            input_text,
            Style::new().fg(self.colors.code_text),
        )));

        // Create paragraph with content
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::new().fg(self.colors.code_text))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)))
            .alignment(Alignment::Left);
        
        frame.render_widget(paragraph, inner_area);
    }
}

/// Prompt manager for handling multiple prompts
#[derive(Debug, Clone)]
pub struct PromptManager {
    /// Current approval prompt
    approval_prompt: Option<ApprovalPrompt>,
    /// Current clarification prompt
    clarify_prompt: Option<ClarifyPrompt>,
    /// Current secret prompt
    secret_prompt: Option<SecretPrompt>,
    /// Colors from configuration
    colors: ChatColorsRgb,
}

impl PromptManager {
    /// Create a new prompt manager
    pub fn new(colors: ChatColorsRgb) -> Self {
        Self {
            approval_prompt: None,
            clarify_prompt: None,
            secret_prompt: None,
            colors,
        }
    }

    /// Create a new prompt manager with defaults
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

    /// Check if any prompt is active
    pub fn has_active_prompt(&self) -> bool {
        self.approval_prompt.as_ref().map_or(false, |p| p.is_pending())
            || self.clarify_prompt.as_ref().map_or(false, |p| p.is_active())
            || self.secret_prompt.as_ref().map_or(false, |p| p.is_active())
    }

    /// Show an approval prompt
    pub fn show_approval(&mut self, message: impl Into<String>, tool_name: Option<String>) {
        self.clarify_prompt = None;
        self.secret_prompt = None;
        self.approval_prompt = Some(ApprovalPrompt::new(message, tool_name, self.colors));
    }

    /// Show a clarification prompt
    pub fn show_clarify(&mut self, message: impl Into<String>) {
        self.approval_prompt = None;
        self.secret_prompt = None;
        self.clarify_prompt = Some(ClarifyPrompt::new(message, self.colors));
    }

    /// Show a secret prompt
    pub fn show_secret(&mut self, message: impl Into<String>) {
        self.approval_prompt = None;
        self.clarify_prompt = None;
        self.secret_prompt = Some(SecretPrompt::new(message, self.colors));
    }

    /// Approve the current approval prompt
    pub fn approve(&mut self) -> bool {
        if let Some(prompt) = &mut self.approval_prompt {
            prompt.approve();
            true
        } else {
            false
        }
    }

    /// Deny the current approval prompt
    pub fn deny(&mut self) -> bool {
        if let Some(prompt) = &mut self.approval_prompt {
            prompt.deny();
            true
        } else {
            false
        }
    }

    /// Get the approval response (if any)
    pub fn get_approval_response(&self) -> Option<bool> {
        self.approval_prompt.as_ref().map(|p| p.is_approved())
    }

    /// Submit the clarify response
    pub fn submit_clarify(&mut self) -> Option<String> {
        self.clarify_prompt.as_mut().map(|p| p.submit())
    }

    /// Submit the secret response
    pub fn submit_secret(&mut self) -> Option<String> {
        self.secret_prompt.as_mut().map(|p| p.submit())
    }

    /// Cancel all prompts
    pub fn cancel_all(&mut self) {
        if let Some(prompt) = &mut self.clarify_prompt {
            prompt.cancel();
        }
        if let Some(prompt) = &mut self.secret_prompt {
            prompt.cancel();
        }
        self.approval_prompt = None;
        self.clarify_prompt = None;
        self.secret_prompt = None;
    }

    /// Set the colors for all future prompts
    pub fn set_colors(&mut self, colors: ChatColorsRgb) {
        self.colors = colors;
        if let Some(prompt) = &mut self.approval_prompt {
            prompt.set_colors(colors);
        }
        if let Some(prompt) = &mut self.clarify_prompt {
            prompt.set_colors(colors);
        }
        if let Some(prompt) = &mut self.secret_prompt {
            prompt.set_colors(colors);
        }
    }

    /// Render the current prompt (if any)
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Find the first active prompt and render it
        if let Some(prompt) = &self.secret_prompt {
            if prompt.is_active() {
                prompt.render(frame, area);
                return;
            }
        }
        
        if let Some(prompt) = &self.clarify_prompt {
            if prompt.is_active() {
                prompt.render(frame, area);
                return;
            }
        }
        
        if let Some(prompt) = &self.approval_prompt {
            if prompt.is_pending() {
                prompt.render(frame, area);
                return;
            }
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
    fn test_approval_prompt_new() {
        let colors = create_test_colors();
        let prompt = ApprovalPrompt::new("Test message", Some("test_tool".to_string()), colors);
        assert_eq!(prompt.message(), "Test message");
        assert_eq!(prompt.tool_name(), Some(&"test_tool".to_string()));
        assert!(prompt.is_pending());
    }

    #[test]
    fn test_approval_prompt_with_defaults() {
        let prompt = ApprovalPrompt::with_defaults("Test");
        assert_eq!(prompt.message(), "Test");
        assert!(prompt.is_pending());
    }

    #[test]
    fn test_approval_prompt_approve_deny() {
        let colors = create_test_colors();
        let mut prompt = ApprovalPrompt::new("Test", None, colors);
        
        assert!(prompt.is_pending());
        
        prompt.approve();
        assert!(prompt.is_approved());
        
        prompt.deny();
        assert!(prompt.is_denied());
    }

    #[test]
    fn test_approval_prompt_setters() {
        let colors = create_test_colors();
        let mut prompt = ApprovalPrompt::new("Test", None, colors);
        
        prompt.set_message("New message");
        assert_eq!(prompt.message(), "New message");
        
        prompt.set_tool_name(Some("new_tool".to_string()));
        assert_eq!(prompt.tool_name(), Some(&"new_tool".to_string()));
    }

    #[test]
    fn test_clarify_prompt_new() {
        let colors = create_test_colors();
        let prompt = ClarifyPrompt::new("Test message", colors);
        assert_eq!(prompt.message(), "Test message");
        assert!(prompt.response().is_empty());
        assert!(prompt.is_active());
    }

    #[test]
    fn test_clarify_prompt_with_defaults() {
        let prompt = ClarifyPrompt::with_defaults("Test");
        assert_eq!(prompt.message(), "Test");
        assert!(prompt.is_active());
    }

    #[test]
    fn test_clarify_prompt_response() {
        let colors = create_test_colors();
        let mut prompt = ClarifyPrompt::new("Test", colors);
        
        prompt.append_response("Hello");
        assert_eq!(prompt.response(), "Hello");
        
        prompt.append_response(" World");
        assert_eq!(prompt.response(), "Hello World");
        
        prompt.clear_response();
        assert!(prompt.response().is_empty());
    }

    #[test]
    fn test_clarify_prompt_submit() {
        let colors = create_test_colors();
        let mut prompt = ClarifyPrompt::new("Test", colors);
        
        prompt.append_response("Response");
        let submitted = prompt.submit();
        
        assert_eq!(submitted, "Response");
        assert!(prompt.response().is_empty());
        assert!(!prompt.is_active());
    }

    #[test]
    fn test_secret_prompt_new() {
        let colors = create_test_colors();
        let prompt = SecretPrompt::new("Test message", colors);
        assert_eq!(prompt.message(), "Test message");
        assert!(prompt.secret().is_empty());
        assert!(prompt.is_active());
    }

    #[test]
    fn test_secret_prompt_with_defaults() {
        let prompt = SecretPrompt::with_defaults("Test");
        assert_eq!(prompt.message(), "Test");
        assert!(prompt.is_active());
    }

    #[test]
    fn test_secret_prompt_input() {
        let colors = create_test_colors();
        let mut prompt = SecretPrompt::new("Test", colors);
        
        prompt.append_secret('a');
        prompt.append_secret('b');
        prompt.append_secret('c');
        
        assert_eq!(prompt.secret(), "abc");
        assert_eq!(prompt.display_text, "***");
        
        prompt.pop_secret();
        assert_eq!(prompt.secret(), "ab");
        assert_eq!(prompt.display_text, "**");
    }

    #[test]
    fn test_secret_prompt_submit() {
        let colors = create_test_colors();
        let mut prompt = SecretPrompt::new("Test", colors);
        
        prompt.append_secret('s');
        prompt.append_secret('e');
        prompt.append_secret('c');
        prompt.append_secret('r');
        prompt.append_secret('e');
        prompt.append_secret('t');
        
        let submitted = prompt.submit();
        
        assert_eq!(submitted, "secret");
        assert!(prompt.secret().is_empty());
        assert!(prompt.display_text.is_empty());
        assert!(!prompt.is_active());
    }

    #[test]
    fn test_prompt_manager_new() {
        let colors = create_test_colors();
        let manager = PromptManager::new(colors);
        assert!(!manager.has_active_prompt());
    }

    #[test]
    fn test_prompt_manager_with_defaults() {
        let manager = PromptManager::with_defaults();
        assert!(!manager.has_active_prompt());
    }

    #[test]
    fn test_prompt_manager_show_approval() {
        let colors = create_test_colors();
        let mut manager = PromptManager::new(colors);
        
        manager.show_approval("Test", Some("tool".to_string()));
        assert!(manager.has_active_prompt());
    }

    #[test]
    fn test_prompt_manager_show_clarify() {
        let colors = create_test_colors();
        let mut manager = PromptManager::new(colors);
        
        manager.show_clarify("Test");
        assert!(manager.has_active_prompt());
    }

    #[test]
    fn test_prompt_manager_show_secret() {
        let colors = create_test_colors();
        let mut manager = PromptManager::new(colors);
        
        manager.show_secret("Test");
        assert!(manager.has_active_prompt());
    }

    #[test]
    fn test_prompt_manager_approve_deny() {
        let colors = create_test_colors();
        let mut manager = PromptManager::new(colors);
        
        manager.show_approval("Test", None);
        
        assert!(manager.approve());
        assert!(manager.get_approval_response() == Some(true));
        
        manager.show_approval("Test 2", None);
        assert!(manager.deny());
        assert!(manager.get_approval_response() == Some(false));
    }

    #[test]
    fn test_prompt_manager_cancel_all() {
        let colors = create_test_colors();
        let mut manager = PromptManager::new(colors);
        
        manager.show_approval("Test", None);
        manager.show_clarify("Test");
        manager.show_secret("Test");
        
        assert!(manager.has_active_prompt());
        
        manager.cancel_all();
        assert!(!manager.has_active_prompt());
    }
}
