use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

pub struct TagManagementDialog {
    current_tags: Vec<String>,
    input_mode: TagInputMode,
    input_buffer: String,
    selected_tag_index: usize,
}

#[derive(Debug, PartialEq)]
pub enum TagInputMode {
    ViewTags,
    AddingTag,
    RemovingTag,
}

impl TagManagementDialog {
    pub fn new(current_tags: Vec<String>) -> Self {
        Self {
            current_tags,
            input_mode: TagInputMode::ViewTags,
            input_buffer: String::new(),
            selected_tag_index: 0,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Create a centered modal
        let modal_width = 60;
        let modal_height = 20;
        
        let modal_area = centered_rect(modal_width, modal_height, area);
        
        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);
        
        // Create the main dialog block - consistent with confirmation dialog style
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Tag Management")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, modal_area);
        
        // Split the inner area
        let inner_area = Rect {
            x: modal_area.x + 1,
            y: modal_area.y + 1,
            width: modal_area.width - 2,
            height: modal_area.height - 2,
        };
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),     // Instructions
                Constraint::Min(5),        // Tag list
                Constraint::Length(3),     // Input area
                Constraint::Length(2),     // Help text
            ])
            .split(inner_area);
        
        // Render instructions
        let instructions = match self.input_mode {
            TagInputMode::ViewTags => "Current tags for this prompt:",
            TagInputMode::AddingTag => "Type new tag and press Enter:",
            TagInputMode::RemovingTag => "Select tag to remove and press Enter:",
        };
        let instructions_widget = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(instructions_widget, chunks[0]);
        
        // Render tag list
        self.render_tag_list(f, chunks[1]);
        
        // Render input area
        self.render_input_area(f, chunks[2]);
        
        // Render help text
        let help_text = match self.input_mode {
            TagInputMode::ViewTags => "a: Add tag  r: Remove tag  Esc: Close",
            TagInputMode::AddingTag => "Enter: Add  Esc: Cancel",
            TagInputMode::RemovingTag => "↑↓: Select  Enter: Remove  Esc: Cancel",
        };
        let help_widget = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(help_widget, chunks[3]);
    }
    
    fn render_tag_list(&self, f: &mut Frame, area: Rect) {
        if self.current_tags.is_empty() {
            let empty_msg = Paragraph::new("No tags")
                .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
                .alignment(Alignment::Center);
            f.render_widget(empty_msg, area);
        } else {
            let items: Vec<ListItem> = self.current_tags
                .iter()
                .enumerate()
                .map(|(i, tag)| {
                    let style = if self.input_mode == TagInputMode::RemovingTag && i == self.selected_tag_index {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(vec![
                        Span::raw("• "),
                        Span::styled(tag, style),
                    ]))
                })
                .collect();
            
            let list = List::new(items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Tags"))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                );
            
            // Create list state for scrolling
            let mut list_state = ListState::default();
            if self.input_mode == TagInputMode::RemovingTag {
                list_state.select(Some(self.selected_tag_index));
            }
                
            f.render_stateful_widget(list, area, &mut list_state);
        }
    }
    
    fn render_input_area(&self, f: &mut Frame, area: Rect) {
        if self.input_mode == TagInputMode::AddingTag {
            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow));
            
            let input_text = format!("{}_", self.input_buffer);
            let input_widget = Paragraph::new(input_text)
                .block(input_block);
            
            f.render_widget(input_widget, area);
        }
    }
    
    // Public methods for interaction
    pub fn start_adding_tag(&mut self) {
        self.input_mode = TagInputMode::AddingTag;
        self.input_buffer.clear();
    }
    
    pub fn start_removing_tag(&mut self) {
        if !self.current_tags.is_empty() {
            self.input_mode = TagInputMode::RemovingTag;
            self.selected_tag_index = 0;
        }
    }
    
    pub fn cancel_input(&mut self) {
        self.input_mode = TagInputMode::ViewTags;
        self.input_buffer.clear();
    }
    
    pub fn add_char(&mut self, c: char) {
        if self.input_mode == TagInputMode::AddingTag {
            self.input_buffer.push(c);
        }
    }
    
    pub fn delete_char(&mut self) {
        if self.input_mode == TagInputMode::AddingTag && !self.input_buffer.is_empty() {
            self.input_buffer.pop();
        }
    }
    
    pub fn get_new_tag(&self) -> Option<String> {
        if self.input_mode == TagInputMode::AddingTag && !self.input_buffer.trim().is_empty() {
            Some(self.input_buffer.trim().to_string())
        } else {
            None
        }
    }
    
    pub fn get_selected_tag_for_removal(&self) -> Option<String> {
        if self.input_mode == TagInputMode::RemovingTag && self.selected_tag_index < self.current_tags.len() {
            Some(self.current_tags[self.selected_tag_index].clone())
        } else {
            None
        }
    }
    
    pub fn move_selection_up(&mut self) {
        if self.input_mode == TagInputMode::RemovingTag && !self.current_tags.is_empty() {
            if self.selected_tag_index == 0 {
                self.selected_tag_index = self.current_tags.len() - 1;
            } else {
                self.selected_tag_index -= 1;
            }
        }
    }
    
    pub fn move_selection_down(&mut self) {
        if self.input_mode == TagInputMode::RemovingTag && !self.current_tags.is_empty() {
            self.selected_tag_index = (self.selected_tag_index + 1) % self.current_tags.len();
        }
    }
    
    pub fn is_in_input_mode(&self) -> bool {
        matches!(self.input_mode, TagInputMode::AddingTag | TagInputMode::RemovingTag)
    }
    
    pub fn input_mode(&self) -> &TagInputMode {
        &self.input_mode
    }
}

// Helper function to create centered rect with fixed size
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    // Ensure we don't exceed the available area
    let width = width.min(r.width.saturating_sub(2));
    let height = height.min(r.height.saturating_sub(2));
    
    // Calculate centered position
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    
    Rect {
        x,
        y,
        width,
        height,
    }
}