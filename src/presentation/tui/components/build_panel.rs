use crate::application::models::{PromptMetadata, PromptType};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::collections::HashSet;

pub struct BuildPanel {
    prompts: Vec<PromptMetadata>,
    selected_prompts: HashSet<String>,
    list_state: ListState,
}

impl BuildPanel {
    pub fn new(prompts: Vec<PromptMetadata>) -> Self {
        let mut list_state = ListState::default();
        if !prompts.is_empty() {
            list_state.select(Some(0));
        }
        
        Self {
            prompts,
            selected_prompts: HashSet::new(),
            list_state,
        }
    }
    
    pub fn update_prompts(&mut self, prompts: Vec<PromptMetadata>) {
        self.prompts = prompts;
        
        // Keep selection in bounds
        if let Some(selected) = self.list_state.selected() {
            if selected >= self.prompts.len() && !self.prompts.is_empty() {
                self.list_state.select(Some(self.prompts.len() - 1));
            } else if self.prompts.is_empty() {
                self.list_state.select(None);
            }
        } else if !self.prompts.is_empty() {
            self.list_state.select(Some(0));
        }
        
        // Remove selections that no longer exist
        self.selected_prompts.retain(|name| {
            self.prompts.iter().any(|p| &p.name == name)
        });
    }
    
    pub fn next(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let next = (selected + 1) % self.prompts.len();
        self.list_state.select(Some(next));
    }
    
    pub fn previous(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let previous = if selected == 0 {
            self.prompts.len() - 1
        } else {
            selected - 1
        };
        self.list_state.select(Some(previous));
    }
    
    pub fn toggle_selection(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if let Some(prompt) = self.prompts.get(selected) {
                if self.selected_prompts.contains(&prompt.name) {
                    self.selected_prompts.remove(&prompt.name);
                } else {
                    self.selected_prompts.insert(prompt.name.clone());
                }
            }
        }
    }
    
    pub fn get_selected_prompts(&self) -> Vec<&PromptMetadata> {
        self.prompts
            .iter()
            .filter(|p| self.selected_prompts.contains(&p.name))
            .collect()
    }
    
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Split area for prompts list and selection info
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);
        
        // Render prompts list
        let items: Vec<ListItem> = self.prompts
            .iter()
            .map(|prompt| {
                let type_indicator = match prompt.prompt_type {
                    PromptType::Instruction => "📝",
                    PromptType::Context => "📚",
                    PromptType::InputIndicator => "⬇️",
                    PromptType::OutputIndicator => "⬆️",
                    PromptType::Etc => "🔧",
                    PromptType::Whole => "📦",
                };
                
                let selected_indicator = if self.selected_prompts.contains(&prompt.name) {
                    "[x]"
                } else {
                    "[ ]"
                };
                
                let tags = prompt.tags.join(", ");
                let content = format!("{} {} {} - {} [{}]", 
                    selected_indicator,
                    type_indicator,
                    prompt.name,
                    prompt.prompt_type.to_string(),
                    tags
                );
                
                ListItem::new(content)
            })
            .collect();
        
        let title = format!("Build Panel - {} prompts available", self.prompts.len());
        let list = List::new(items)
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray));
        
        frame.render_stateful_widget(list, chunks[0], &mut self.list_state);
        
        // Render selection info
        let selected_count = self.selected_prompts.len();
        let info_text = format!("Selected: {} prompt(s)", selected_count);
        let info = Paragraph::new(info_text)
            .block(Block::default()
                .borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        
        frame.render_widget(info, chunks[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_build_panel_creation() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("fink").join("test1.md");
        
        let prompts = vec![
            PromptMetadata {
                name: "test1".to_string(),
                file_path: test_file.to_string_lossy().to_string(),
                tags: vec![],
                prompt_type: PromptType::Instruction,
            },
        ];
        
        let panel = BuildPanel::new(prompts);
        assert_eq!(panel.prompts.len(), 1);
        assert_eq!(panel.selected_prompts.len(), 0);
    }
    
    #[test]
    fn test_toggle_selection() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("fink").join("test1.md");
        
        let prompts = vec![
            PromptMetadata {
                name: "test1".to_string(),
                file_path: test_file.to_string_lossy().to_string(),
                tags: vec![],
                prompt_type: PromptType::Instruction,
            },
        ];
        
        let mut panel = BuildPanel::new(prompts);
        
        // Toggle selection on
        panel.toggle_selection();
        assert_eq!(panel.selected_prompts.len(), 1);
        assert!(panel.selected_prompts.contains("test1"));
        
        // Toggle selection off
        panel.toggle_selection();
        assert_eq!(panel.selected_prompts.len(), 0);
    }
}