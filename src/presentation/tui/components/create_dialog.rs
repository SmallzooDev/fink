use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogField {
    Filename,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreateTemplate {
    Default,
    Basic,
    FromClipboard,
}

impl CreateTemplate {
    pub fn name(&self) -> &str {
        match self {
            CreateTemplate::Default => "Default",
            CreateTemplate::Basic => "Basic Template",
            CreateTemplate::FromClipboard => "From Clipboard",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            CreateTemplate::Default => "Empty prompt with default frontmatter",
            CreateTemplate::Basic => "Start with a structured template",
            CreateTemplate::FromClipboard => "Create from your current clipboard content",
        }
    }
}

pub struct CreateDialog {
    filename: String,
    template: CreateTemplate,
    current_field: DialogField,
}

impl CreateDialog {
    pub fn new() -> Self {
        Self {
            filename: String::new(),
            template: CreateTemplate::Default,
            current_field: DialogField::Filename,
        }
    }
    
    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    
    pub fn get_normalized_filename(&self) -> String {
        self.filename.trim().to_lowercase().replace(' ', "-")
    }
    
    pub fn get_template(&self) -> CreateTemplate {
        self.template
    }
    
    pub fn current_field(&self) -> DialogField {
        self.current_field
    }
    
    pub fn add_char(&mut self, c: char) {
        if self.current_field == DialogField::Filename {
            self.filename.push(c);
        }
    }
    
    pub fn delete_char(&mut self) {
        if self.current_field == DialogField::Filename {
            self.filename.pop();
        }
    }
    
    pub fn next_field(&mut self) {
        self.current_field = match self.current_field {
            DialogField::Filename => DialogField::Template,
            DialogField::Template => DialogField::Filename,
        };
    }
    
    pub fn previous_field(&mut self) {
        self.next_field(); // Since we only have 2 fields, next and previous are the same
    }
    
    pub fn next_template(&mut self) {
        if self.current_field == DialogField::Template {
            self.template = match self.template {
                CreateTemplate::Default => CreateTemplate::Basic,
                CreateTemplate::Basic => CreateTemplate::FromClipboard,
                CreateTemplate::FromClipboard => CreateTemplate::Default,
            };
        }
    }
    
    pub fn previous_template(&mut self) {
        if self.current_field == DialogField::Template {
            self.template = match self.template {
                CreateTemplate::Default => CreateTemplate::FromClipboard,
                CreateTemplate::FromClipboard => CreateTemplate::Basic,
                CreateTemplate::Basic => CreateTemplate::Default,
            };
        }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.filename.trim().is_empty()
    }
}

impl Widget for &CreateDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the area first
        Clear.render(area, buf);
        
        // Create the outer block
        let block = Block::default()
            .title(" Create New Prompt ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Layout for the dialog content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Filename field
                Constraint::Length(1), // Spacing
                Constraint::Length(5), // Template selection
                Constraint::Length(1), // Spacing
                Constraint::Length(2), // Help text
            ])
            .split(inner);
        
        // Filename field
        let filename_style = if self.current_field == DialogField::Filename {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let filename_block = Block::default()
            .title("Filename")
            .borders(Borders::ALL)
            .border_style(filename_style);
        
        let filename_text = if self.filename.is_empty() && self.current_field == DialogField::Filename {
            Paragraph::new("_")
                .style(Style::default().fg(Color::DarkGray))
        } else {
            let display_text = if self.current_field == DialogField::Filename {
                format!("{}_", self.filename)
            } else {
                self.filename.clone()
            };
            Paragraph::new(display_text)
        };
        
        filename_text
            .block(filename_block)
            .render(chunks[0], buf);
        
        // Template selection
        let template_style = if self.current_field == DialogField::Template {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let template_block = Block::default()
            .title("Template")
            .borders(Borders::ALL)
            .border_style(template_style);
        
        let template_text = vec![
            Line::from(vec![
                Span::raw("Selected: "),
                Span::styled(
                    self.template.name(),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                self.template.description(),
                Style::default().fg(Color::DarkGray)
            )),
        ];
        
        Paragraph::new(template_text)
            .block(template_block)
            .render(chunks[2], buf);
        
        // Help text
        let help_text = match self.current_field {
            DialogField::Filename => {
                vec![
                    Line::from(vec![
                        Span::raw("Enter filename • "),
                        Span::styled("Tab", Style::default().fg(Color::Cyan)),
                        Span::raw(" to switch fields • "),
                        Span::styled("Enter", Style::default().fg(Color::Green)),
                        Span::raw(" to create • "),
                        Span::styled("Esc", Style::default().fg(Color::Red)),
                        Span::raw(" to cancel"),
                    ])
                ]
            }
            DialogField::Template => {
                vec![
                    Line::from(vec![
                        Span::styled("←/→", Style::default().fg(Color::Cyan)),
                        Span::raw(" to change template • "),
                        Span::styled("Tab", Style::default().fg(Color::Cyan)),
                        Span::raw(" to switch fields • "),
                        Span::styled("Enter", Style::default().fg(Color::Green)),
                        Span::raw(" to create"),
                    ])
                ]
            }
        };
        
        Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .render(chunks[4], buf);
    }
}