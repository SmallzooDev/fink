use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

pub struct TagFilterDialog {
    available_tags: Vec<String>,
    selected_index: usize,
    active_filter: Option<String>,
}

impl TagFilterDialog {
    pub fn new(available_tags: Vec<String>, active_filter: Option<String>) -> Self {
        Self {
            available_tags,
            selected_index: 0,
            active_filter,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let modal_width = 50;
        let modal_height = 15;
        
        let modal_area = centered_rect(modal_width, modal_height, area);
        
        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);
        
        // Create the main dialog block
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Filter by Tag")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, modal_area);
        
        // Split inner area
        let inner_area = Rect {
            x: modal_area.x + 1,
            y: modal_area.y + 1,
            width: modal_area.width - 2,
            height: modal_area.height - 2,
        };
        
        // Show current filter status
        let status_text = if let Some(filter) = &self.active_filter {
            format!("Active filter: {}", filter)
        } else {
            "No filter active".to_string()
        };
        
        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(status, Rect {
            x: inner_area.x,
            y: inner_area.y,
            width: inner_area.width,
            height: 1,
        });
        
        // Show available tags
        let tag_list_area = Rect {
            x: inner_area.x,
            y: inner_area.y + 2,
            width: inner_area.width,
            height: inner_area.height.saturating_sub(4),
        };
        
        if self.available_tags.is_empty() {
            let empty_msg = Paragraph::new("No tags available")
                .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
                .alignment(Alignment::Center);
            f.render_widget(empty_msg, tag_list_area);
        } else {
            let items: Vec<ListItem> = self.available_tags
                .iter()
                .enumerate()
                .map(|(_, tag)| {
                    let is_active = self.active_filter.as_ref() == Some(tag);
                    
                    let style = if is_active {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    
                    let prefix = if is_active { "▶ " } else { "  " };
                    ListItem::new(Line::from(vec![
                        Span::raw(prefix),
                        Span::styled(tag, style),
                    ]))
                })
                .collect();
            
            let list = List::new(items)
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                );
            
            let mut list_state = ListState::default();
            list_state.select(Some(self.selected_index));
            
            f.render_stateful_widget(list, tag_list_area, &mut list_state);
        }
        
        // Help text
        let help_text = "↑↓: Navigate  Enter: Apply filter  c: Clear filter  Esc: Close";
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help, Rect {
            x: inner_area.x,
            y: inner_area.y + inner_area.height - 1,
            width: inner_area.width,
            height: 1,
        });
    }
    
    pub fn move_up(&mut self) {
        if !self.available_tags.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.available_tags.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }
    
    pub fn move_down(&mut self) {
        if !self.available_tags.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.available_tags.len();
        }
    }
    
    pub fn get_selected_tag(&self) -> Option<&String> {
        self.available_tags.get(self.selected_index)
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