use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};
use crate::presentation::tui::components::input_field::InputField;
use crate::utils::config::Config;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigField {
    Prefix,
    Postfix,
}

pub struct ConfigScreen {
    config: Config,
    config_path: std::path::PathBuf,
    current_field: ConfigField,
    prefix_input: String,
    postfix_input: String,
    has_changes: bool,
    saved_message: Option<String>,
}

impl ConfigScreen {
    pub fn new(config: Config) -> Self {
        Self::new_with_path(config, Config::default_config_path())
    }
    
    pub fn new_with_path(config: Config, config_path: std::path::PathBuf) -> Self {
        let prefix_input = config.clipboard_prefix().to_string();
        let postfix_input = config.clipboard_postfix().to_string();
        
        Self {
            config,
            config_path,
            current_field: ConfigField::Prefix,
            prefix_input,
            postfix_input,
            has_changes: false,
            saved_message: None,
        }
    }
    
    pub fn current_field(&self) -> ConfigField {
        self.current_field
    }
    
    pub fn next_field(&mut self) {
        self.current_field = match self.current_field {
            ConfigField::Prefix => ConfigField::Postfix,
            ConfigField::Postfix => ConfigField::Prefix,
        };
    }
    
    pub fn previous_field(&mut self) {
        self.next_field(); // Only two fields, so next is same as previous
    }
    
    pub fn add_char(&mut self, c: char) {
        match self.current_field {
            ConfigField::Prefix => {
                self.prefix_input.push(c);
                self.has_changes = true;
            }
            ConfigField::Postfix => {
                self.postfix_input.push(c);
                self.has_changes = true;
            }
        }
        self.saved_message = None;
    }
    
    pub fn delete_char(&mut self) {
        match self.current_field {
            ConfigField::Prefix => {
                self.prefix_input.pop();
                self.has_changes = true;
            }
            ConfigField::Postfix => {
                self.postfix_input.pop();
                self.has_changes = true;
            }
        }
        self.saved_message = None;
    }
    
    pub fn save_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config.set_clipboard_prefix(self.prefix_input.clone());
        self.config.set_clipboard_postfix(self.postfix_input.clone());
        
        self.config.save(&self.config_path)?;
        
        self.has_changes = false;
        self.saved_message = Some("Configuration saved successfully!".to_string());
        Ok(())
    }
    
    pub fn has_changes(&self) -> bool {
        self.has_changes
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Calculate modal size
        let modal_width = 70;
        let modal_height = 20;
        
        let modal_area = centered_rect(modal_width, modal_height, area);
        
        // Clear the area behind the modal
        f.render_widget(Clear, modal_area);
        
        // Main dialog block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Configuration ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Cyan));
        
        let inner = block.inner(modal_area);
        f.render_widget(block, modal_area);
        
        // Layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Prefix field
                Constraint::Length(3), // Postfix field
                Constraint::Length(3), // Status message
                Constraint::Min(1),    // Spacer
                Constraint::Length(3), // Help text
            ])
            .split(inner);
        
        // Title
        let title = Paragraph::new("Configure Clipboard Prefix and Postfix")
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        // Prefix input
        let prefix_style = if self.current_field == ConfigField::Prefix {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let prefix_block = Block::default()
            .borders(Borders::ALL)
            .title("Prefix (prepended to copied prompts)")
            .border_style(prefix_style);
        
        let prefix_field = InputField::new(&self.prefix_input)
            .show_cursor(self.current_field == ConfigField::Prefix)
            .block(prefix_block);
        f.render_widget(prefix_field, chunks[1]);
        
        // Postfix input
        let postfix_style = if self.current_field == ConfigField::Postfix {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let postfix_block = Block::default()
            .borders(Borders::ALL)
            .title("Postfix (appended to copied prompts)")
            .border_style(postfix_style);
        
        let postfix_field = InputField::new(&self.postfix_input)
            .show_cursor(self.current_field == ConfigField::Postfix)
            .block(postfix_block);
        f.render_widget(postfix_field, chunks[2]);
        
        // Status message
        if let Some(msg) = &self.saved_message {
            let status = Paragraph::new(msg.as_str())
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center);
            f.render_widget(status, chunks[3]);
        } else if self.has_changes {
            let status = Paragraph::new("(unsaved changes)")
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM))
                .alignment(Alignment::Center);
            f.render_widget(status, chunks[3]);
        }
        
        // Help text
        let help_lines = vec![
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(Color::Cyan)),
                Span::raw(" to switch fields • "),
                Span::styled("Ctrl+S", Style::default().fg(Color::Green)),
                Span::raw(" to save • "),
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw(" to exit"),
            ]),
            Line::from(
                Span::styled("Example: Prefix=\"### \" Postfix=\"\\n\\nPlease help me with this.\"", Style::default().fg(Color::DarkGray))
            ),
        ];
        
        let help = Paragraph::new(help_lines)
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[5]);
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let horizontal_margin = (area.width.saturating_sub(width)) / 2;
    let vertical_margin = (area.height.saturating_sub(height)) / 2;
    
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width),
            Constraint::Length(horizontal_margin),
        ])
        .split(area);
    
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Length(vertical_margin),
        ])
        .split(horizontal[1]);
    
    vertical[1]
}