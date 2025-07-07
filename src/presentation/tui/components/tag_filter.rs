use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use std::collections::HashSet;

pub struct TagFilterDialog {
    available_tags: Vec<String>,
    selected_index: usize,
    selected_tags: HashSet<String>,
    search_query: String,
    cursor_position: usize,
    pub is_searching: bool,
}

impl TagFilterDialog {
    pub fn new(available_tags: Vec<String>, active_filters: HashSet<String>) -> Self {
        Self {
            available_tags,
            selected_index: 0,
            selected_tags: active_filters,
            search_query: String::new(),
            cursor_position: 0,
            is_searching: true,
        }
    }
    
    /// Get filtered tags based on search query
    pub fn get_filtered_tags(&self) -> Vec<String> {
        if self.search_query.is_empty() {
            self.available_tags.clone()
        } else {
            let query_lower = self.search_query.to_lowercase();
            self.available_tags
                .iter()
                .filter(|tag| tag.to_lowercase().contains(&query_lower))
                .cloned()
                .collect()
        }
    }
    
    /// Toggle selection of the currently highlighted tag
    pub fn toggle_selected_tag(&mut self) {
        let filtered_tags = self.get_filtered_tags();
        if let Some(tag) = filtered_tags.get(self.selected_index) {
            if self.selected_tags.contains(tag) {
                self.selected_tags.remove(tag);
            } else {
                self.selected_tags.insert(tag.clone());
            }
        }
    }
    
    /// Get the currently selected tags
    pub fn get_selected_tags(&self) -> HashSet<String> {
        self.selected_tags.clone()
    }
    
    /// Clear all selected tags
    pub fn clear_selection(&mut self) {
        self.selected_tags.clear();
    }
    
    /// Add character to search query
    pub fn add_char(&mut self, c: char) {
        self.search_query.insert(self.cursor_position, c);
        self.cursor_position += 1;
        self.selected_index = 0; // Reset selection when search changes
    }
    
    /// Remove character from search query
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.search_query.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.selected_index = 0; // Reset selection when search changes
        }
    }
    
    /// Clear search query
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.cursor_position = 0;
        self.selected_index = 0;
    }
    
    /// Toggle between search and selection mode
    pub fn toggle_mode(&mut self) {
        self.is_searching = !self.is_searching;
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let modal_width = 60;
        let modal_height = 20;
        
        let modal_area = centered_rect(modal_width, modal_height, area);
        
        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);
        
        // Create the main dialog block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Tag Filter - Multi-select ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, modal_area);
        
        // Split inner area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Search box
                Constraint::Length(1), // Status line
                Constraint::Min(5),    // Tag list
                Constraint::Length(2), // Help text
            ])
            .split(modal_area);
        
        // Render search box
        let search_block = Block::default()
            .borders(Borders::ALL)
            .title("Search Tags")
            .border_style(if self.is_searching {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            });
        
        let search_text = Paragraph::new(self.search_query.as_str())
            .block(search_block);
        f.render_widget(search_text, chunks[0]);
        
        // Show cursor in search box when in search mode
        if self.is_searching {
            let cursor_x = chunks[0].x + 1 + self.cursor_position as u16;
            let cursor_y = chunks[0].y + 1;
            f.set_cursor(cursor_x, cursor_y);
        }
        
        // Show current filter status
        let selected_count = self.selected_tags.len();
        let status_text = if selected_count > 0 {
            let tags: Vec<String> = self.selected_tags.iter().cloned().collect();
            format!("Selected {} tag(s): {}", selected_count, tags.join(", "))
        } else {
            "No tags selected".to_string()
        };
        
        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Cyan))
            .wrap(Wrap { trim: true });
        f.render_widget(status, chunks[1]);
        
        // Show filtered tags
        let filtered_tags = self.get_filtered_tags();
        
        if filtered_tags.is_empty() {
            let empty_msg = Paragraph::new("No tags match your search")
                .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
                .alignment(Alignment::Center);
            f.render_widget(empty_msg, chunks[2]);
        } else {
            let items: Vec<ListItem> = filtered_tags
                .iter()
                .enumerate()
                .map(|(i, tag)| {
                    let is_selected = self.selected_tags.contains(tag);
                    let is_highlighted = i == self.selected_index && !self.is_searching;
                    
                    let checkbox = if is_selected { "[✓]" } else { "[ ]" };
                    let style = match (is_highlighted, is_selected) {
                        (true, true) => Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                        (true, false) => Style::default().bg(Color::DarkGray),
                        (false, true) => Style::default().fg(Color::Green),
                        (false, false) => Style::default(),
                    };
                    
                    ListItem::new(Line::from(vec![
                        Span::raw(format!("{} ", checkbox)),
                        Span::styled(tag, style),
                    ]))
                })
                .collect();
            
            let list = List::new(items);
            
            let mut list_state = ListState::default();
            if !self.is_searching {
                list_state.select(Some(self.selected_index));
            }
            
            f.render_stateful_widget(list, chunks[2], &mut list_state);
        }
        
        // Help text
        let help_text = if self.is_searching {
            "Type to search • Tab: Switch to selection • Enter: Apply • Esc: Cancel"
        } else {
            "↑↓: Navigate • Space: Toggle • Tab: Back to search • Enter: Apply • c: Clear all • Esc: Cancel"
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(help, chunks[3]);
    }
    
    pub fn move_up(&mut self) {
        let filtered_tags = self.get_filtered_tags();
        if !filtered_tags.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = filtered_tags.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }
    
    pub fn move_down(&mut self) {
        let filtered_tags = self.get_filtered_tags();
        if !filtered_tags.is_empty() {
            self.selected_index = (self.selected_index + 1) % filtered_tags.len();
        }
    }
    
    pub fn get_selected_tag(&self) -> Option<String> {
        let filtered_tags = self.get_filtered_tags();
        filtered_tags.get(self.selected_index).cloned()
    }
}

// Helper function to create centered rect
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let width = width.min(r.width.saturating_sub(2));
    let height = height.min(r.height.saturating_sub(2));
    
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    
    Rect {
        x,
        y,
        width,
        height,
    }
}