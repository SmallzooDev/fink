use crate::application::models::{PromptMetadata, PromptType};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildStep {
    SelectInstruction,
    SelectContext,
    SelectInputIndicator,
    SelectOutputIndicator,
    SelectEtc,
    AddComment,
    Complete,
}

impl BuildStep {
    fn next(&self) -> Self {
        match self {
            BuildStep::SelectInstruction => BuildStep::SelectContext,
            BuildStep::SelectContext => BuildStep::SelectInputIndicator,
            BuildStep::SelectInputIndicator => BuildStep::SelectOutputIndicator,
            BuildStep::SelectOutputIndicator => BuildStep::SelectEtc,
            BuildStep::SelectEtc => BuildStep::AddComment,
            BuildStep::AddComment => BuildStep::Complete,
            BuildStep::Complete => BuildStep::Complete,
        }
    }
    
    fn get_prompt_type(&self) -> Option<PromptType> {
        match self {
            BuildStep::SelectInstruction => Some(PromptType::Instruction),
            BuildStep::SelectContext => Some(PromptType::Context),
            BuildStep::SelectInputIndicator => Some(PromptType::InputIndicator),
            BuildStep::SelectOutputIndicator => Some(PromptType::OutputIndicator),
            BuildStep::SelectEtc => Some(PromptType::Etc),
            _ => None,
        }
    }
    
    fn get_title(&self) -> &str {
        match self {
            BuildStep::SelectInstruction => "Select Instruction Prompt",
            BuildStep::SelectContext => "Select Context Prompt",
            BuildStep::SelectInputIndicator => "Select Input Indicator",
            BuildStep::SelectOutputIndicator => "Select Output Indicator",
            BuildStep::SelectEtc => "Select Additional Prompt",
            BuildStep::AddComment => "Add Optional Comment",
            BuildStep::Complete => "Build Complete",
        }
    }
    
    fn get_description(&self) -> &str {
        match self {
            BuildStep::SelectInstruction => "Choose an instruction prompt that defines the AI's role and behavior",
            BuildStep::SelectContext => "Choose a context prompt that provides background information",
            BuildStep::SelectInputIndicator => "Choose an input indicator to mark where user input goes",
            BuildStep::SelectOutputIndicator => "Choose an output indicator to specify expected output format",
            BuildStep::SelectEtc => "Choose any additional prompts or utilities",
            BuildStep::AddComment => "Add an optional comment to be included at the end (press Enter to skip)",
            BuildStep::Complete => "Your prompts have been combined and copied to clipboard!",
        }
    }
}

pub struct InteractiveBuildPanel {
    all_prompts: Vec<PromptMetadata>,
    pub current_step: BuildStep,
    selected_prompts: HashMap<PromptType, Option<String>>, // None means skip this type
    list_state: ListState,
    comment: String,
    comment_cursor: usize,
}

impl InteractiveBuildPanel {
    pub fn new(prompts: Vec<PromptMetadata>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Select "None" by default
        
        Self {
            all_prompts: prompts,
            current_step: BuildStep::SelectInstruction,
            selected_prompts: HashMap::new(),
            list_state,
            comment: String::new(),
            comment_cursor: 0,
        }
    }
    
    pub fn get_current_prompts(&self) -> Vec<(String, &PromptMetadata)> {
        let mut prompts = vec![("None".to_string(), None)];
        
        if let Some(prompt_type) = self.current_step.get_prompt_type() {
            for prompt in &self.all_prompts {
                if prompt.prompt_type == prompt_type {
                    prompts.push((prompt.name.clone(), Some(prompt)));
                }
            }
        }
        
        prompts.into_iter()
            .filter_map(|(name, opt)| opt.map(|p| (name, p)))
            .collect()
    }
    
    pub fn get_current_options(&self) -> Vec<String> {
        let mut options = vec!["None (skip this type)".to_string()];
        
        if let Some(prompt_type) = self.current_step.get_prompt_type() {
            for prompt in &self.all_prompts {
                if prompt.prompt_type == prompt_type {
                    options.push(format!("{} - {}", prompt.name, prompt.tags.join(", ")));
                }
            }
        }
        
        options
    }
    
    pub fn next(&mut self) {
        let options = self.get_current_options();
        if options.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let next = (selected + 1) % options.len();
        self.list_state.select(Some(next));
    }
    
    pub fn previous(&mut self) {
        let options = self.get_current_options();
        if options.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let previous = if selected == 0 {
            options.len() - 1
        } else {
            selected - 1
        };
        self.list_state.select(Some(previous));
    }
    
    pub fn select_current(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(prompt_type) = self.current_step.get_prompt_type() {
                if selected_idx == 0 {
                    // User selected "None"
                    self.selected_prompts.insert(prompt_type, None);
                } else {
                    // Get the actual prompt
                    let prompts: Vec<_> = self.all_prompts
                        .iter()
                        .filter(|p| p.prompt_type == prompt_type)
                        .collect();
                    
                    if let Some(prompt) = prompts.get(selected_idx - 1) {
                        self.selected_prompts.insert(prompt_type, Some(prompt.name.clone()));
                    }
                }
            }
        }
        
        // Move to next step
        self.current_step = self.current_step.next();
        self.list_state.select(Some(0)); // Reset selection for next step
    }
    
    pub fn add_comment_char(&mut self, c: char) {
        // Convert to chars for proper Unicode handling
        let mut chars: Vec<char> = self.comment.chars().collect();
        if self.comment_cursor <= chars.len() {
            chars.insert(self.comment_cursor, c);
            self.comment = chars.into_iter().collect();
            self.comment_cursor += 1;
        }
    }
    
    pub fn delete_comment_char(&mut self) {
        if self.comment_cursor > 0 {
            // Convert to chars for proper Unicode handling
            let mut chars: Vec<char> = self.comment.chars().collect();
            if self.comment_cursor <= chars.len() {
                chars.remove(self.comment_cursor - 1);
                self.comment = chars.into_iter().collect();
                self.comment_cursor -= 1;
            }
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.comment_cursor > 0 {
            self.comment_cursor -= 1;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        let char_count = self.comment.chars().count();
        if self.comment_cursor < char_count {
            self.comment_cursor += 1;
        }
    }
    
    pub fn finish_comment(&mut self) {
        self.current_step = BuildStep::Complete;
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_step == BuildStep::Complete
    }
    
    pub fn get_selected_prompt_names(&self) -> Vec<(PromptType, String)> {
        let mut result = Vec::new();
        let type_order = [
            PromptType::Instruction,
            PromptType::Context,
            PromptType::InputIndicator,
            PromptType::OutputIndicator,
            PromptType::Etc,
        ];
        
        for prompt_type in &type_order {
            if let Some(Some(name)) = self.selected_prompts.get(prompt_type) {
                result.push((*prompt_type, name.clone()));
            }
        }
        
        result
    }
    
    pub fn get_comment(&self) -> &str {
        &self.comment
    }
    
    pub fn get_selected_prompt_for_preview(&self) -> Option<&PromptMetadata> {
        if let Some(selected_idx) = self.list_state.selected() {
            if selected_idx == 0 {
                return None; // "None" is selected
            }
            
            if let Some(prompt_type) = self.current_step.get_prompt_type() {
                self.all_prompts
                    .iter()
                    .filter(|p| p.prompt_type == prompt_type)
                    .nth(selected_idx - 1)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),   // Header with title and description
                Constraint::Min(10),     // Main content
                Constraint::Length(3),   // Status bar
            ])
            .split(area);
        
        // Header
        self.render_header(frame, chunks[0]);
        
        // Main content - different based on step
        match self.current_step {
            BuildStep::AddComment => self.render_comment_input(frame, chunks[1]),
            BuildStep::Complete => self.render_complete(frame, chunks[1]),
            _ => self.render_prompt_selection(frame, chunks[1]),
        }
        
        // Status bar
        self.render_status_bar(frame, chunks[2]);
    }
    
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_text = vec![
            Line::from(Span::styled(
                self.current_step.get_title(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(self.current_step.get_description()),
        ];
        
        let header = Paragraph::new(header_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded));
        
        frame.render_widget(header, area);
    }
    
    fn render_prompt_selection(&mut self, frame: &mut Frame, area: Rect) {
        // Split area for list and preview
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(area);
        
        // Render options list
        let options = self.get_current_options();
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(idx, option)| {
                if idx == 0 {
                    ListItem::new(Span::styled(option, Style::default().fg(Color::DarkGray)))
                } else {
                    ListItem::new(option.as_str())
                }
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Available Options "))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));
        
        frame.render_stateful_widget(list, chunks[0], &mut self.list_state);
        
        // Render preview (without content for now - will be provided from app)
        self.render_preview(frame, chunks[1], None);
    }
    
    pub fn render_preview(&self, frame: &mut Frame, area: Rect, prompt_content: Option<String>) {
        let content = if let Some(prompt) = self.get_selected_prompt_for_preview() {
            if let Some(content) = prompt_content {
                content
            } else {
                format!(
                    "Name: {}\nType: {}\nTags: {}\n\n[Loading content...]",
                    prompt.name,
                    prompt.prompt_type,
                    prompt.tags.join(", ")
                )
            }
        } else {
            "No prompt selected - choosing 'None' will skip this prompt type.".to_string()
        };
        
        let preview = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Preview "))
            .wrap(Wrap { trim: true });
        
        frame.render_widget(preview, area);
    }
    
    fn render_comment_input(&self, frame: &mut Frame, area: Rect) {
        let input_text = format!("{}_", self.comment);
        
        let input = Paragraph::new(input_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Optional Comment (press Enter to skip or finish) "))
            .style(Style::default().fg(Color::Yellow));
        
        frame.render_widget(input, area);
    }
    
    fn render_complete(&self, frame: &mut Frame, area: Rect) {
        let selected_count = self.selected_prompts
            .values()
            .filter(|v| v.is_some())
            .count();
        
        let complete_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "✓ Build Complete!",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Selected {} prompts", selected_count)),
            if !self.comment.is_empty() {
                Line::from(format!("Added comment: {}", self.comment))
            } else {
                Line::from("No comment added")
            },
            Line::from(""),
            Line::from("Your combined prompt has been copied to the clipboard."),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter or Esc to exit build mode",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        
        let complete = Paragraph::new(complete_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded))
            .alignment(Alignment::Center);
        
        frame.render_widget(complete, area);
    }
    
    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let keys = match self.current_step {
            BuildStep::AddComment => "Type comment | Enter: Finish | Esc: Skip comment",
            BuildStep::Complete => "Enter/Esc: Exit build mode",
            _ => "↑↓: Navigate | Enter: Select | Esc: Cancel",
        };
        
        let status = Paragraph::new(keys)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded))
            .style(Style::default().fg(Color::DarkGray));
        
        frame.render_widget(status, area);
    }
}