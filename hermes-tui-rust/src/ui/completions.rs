//! Completions popup component for TUI
//!
//! This module provides a popup dialog for autocomplete suggestions
//! (such as slash commands, paths, etc.) displayed above the input composer.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};
use crate::state::config::ChatColorsRgb;
use crate::protocol::types::CompletionItem;

/// Completion popup component
#[derive(Debug, Clone)]
pub struct CompletionPopup {
    /// Autocomplete items
    items: Vec<CompletionItem>,
    /// Visibility state
    visible: bool,
    /// Currently selected index
    selected_index: usize,
    /// UI colors from configuration
    colors: ChatColorsRgb,
}

impl CompletionPopup {
    /// Create a new completion popup instance
    pub fn new(colors: ChatColorsRgb) -> Self {
        Self {
            items: Vec::new(),
            visible: false,
            selected_index: 0,
            colors,
        }
    }

    /// Show the completion popup with items
    pub fn show(&mut self, items: Vec<CompletionItem>) {
        self.items = items;
        self.selected_index = 0;
        self.visible = !self.items.is_empty();
    }

    /// Hide the completion popup and clear items
    pub fn hide(&mut self) {
        self.visible = false;
        self.items.clear();
    }

    /// Check if the popup is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the currently selected completion item
    pub fn selected_item(&self) -> Option<&CompletionItem> {
        if self.visible && !self.items.is_empty() {
            self.items.get(self.selected_index)
        } else {
            None
        }
    }

    /// Select the next item in the completions list
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    /// Select the previous item in the completions list
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Render the completion popup above the composer area
    pub fn render(&self, frame: &mut Frame, composer_area: Rect) {
        if !self.visible || self.items.is_empty() {
            return;
        }

        // Compute popup area above the composer
        let max_height = 8;
        let height = (self.items.len() as u16 + 2).min(max_height);
        let y = composer_area.y.saturating_sub(height);
        let popup_area = Rect::new(
            composer_area.x.saturating_add(2),
            y,
            composer_area.width.saturating_sub(4).max(10),
            height,
        );

        // Clear the area to overlay the popup cleanly
        frame.render_widget(Clear, popup_area);

        // Build list items
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let text = if let Some(desc) = &item.meta {
                    format!(" {} - {}", item.display, desc)
                } else {
                    format!(" {}", item.display)
                };
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(self.colors.user_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.colors.user_text)
                };
                ListItem::new(text).style(style)
            })
            .collect();

        let block = Block::default()
            .title(" Completions ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.colors.border));

        let list = List::new(list_items)
            .block(block);

        frame.render_widget(list, popup_area);
    }
}
