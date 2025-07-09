use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};
use crate::application::models::PromptType;
use crate::presentation::tui::components::input_field::InputField;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogField {
    Filename,
    Type,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreateTemplate {
    FromClipboard,
    Default,
    Basic,
}

impl CreateTemplate {
    pub fn name(&self) -> &str {
        match self {
            CreateTemplate::FromClipboard => "From Clipboard",
            CreateTemplate::Default => "Default",
            CreateTemplate::Basic => "Basic Template",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            CreateTemplate::FromClipboard => "Create from your current clipboard content",
            CreateTemplate::Default => "Empty prompt with default frontmatter",
            CreateTemplate::Basic => "Start with a structured template",
        }
    }
}

pub struct CreateDialog {
    filename: String,
    template: CreateTemplate,
    prompt_type: PromptType,
    current_field: DialogField,
}

impl Default for CreateDialog {
    fn default() -> Self {
        Self {
            filename: String::new(),
            template: CreateTemplate::FromClipboard,
            prompt_type: PromptType::default(),
            current_field: DialogField::Filename,
        }
    }
}

impl CreateDialog {
    pub fn new() -> Self {
        Self::default()
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
    
    pub fn get_prompt_type(&self) -> PromptType {
        self.prompt_type
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
            DialogField::Filename => DialogField::Type,
            DialogField::Type => DialogField::Template,
            DialogField::Template => DialogField::Filename,
        };
    }
    
    pub fn previous_field(&mut self) {
        self.current_field = match self.current_field {
            DialogField::Filename => DialogField::Template,
            DialogField::Type => DialogField::Filename,
            DialogField::Template => DialogField::Type,
        };
    }
    
    pub fn next_template(&mut self) {
        if self.current_field == DialogField::Template {
            self.template = match self.template {
                CreateTemplate::FromClipboard => CreateTemplate::Default,
                CreateTemplate::Default => CreateTemplate::Basic,
                CreateTemplate::Basic => CreateTemplate::FromClipboard,
            };
        }
    }
    
    pub fn previous_template(&mut self) {
        if self.current_field == DialogField::Template {
            self.template = match self.template {
                CreateTemplate::FromClipboard => CreateTemplate::Basic,
                CreateTemplate::Basic => CreateTemplate::Default,
                CreateTemplate::Default => CreateTemplate::FromClipboard,
            };
        }
    }
    
    pub fn next_type(&mut self) {
        if self.current_field == DialogField::Type {
            self.prompt_type = match self.prompt_type {
                PromptType::Whole => PromptType::Instruction,
                PromptType::Instruction => PromptType::Context,
                PromptType::Context => PromptType::InputIndicator,
                PromptType::InputIndicator => PromptType::OutputIndicator,
                PromptType::OutputIndicator => PromptType::Etc,
                PromptType::Etc => PromptType::Whole,
            };
        }
    }
    
    pub fn previous_type(&mut self) {
        if self.current_field == DialogField::Type {
            self.prompt_type = match self.prompt_type {
                PromptType::Whole => PromptType::Etc,
                PromptType::Instruction => PromptType::Whole,
                PromptType::Context => PromptType::Instruction,
                PromptType::InputIndicator => PromptType::Context,
                PromptType::OutputIndicator => PromptType::InputIndicator,
                PromptType::Etc => PromptType::OutputIndicator,
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
            .border_type(ratatui::widgets::BorderType::Rounded)
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
                Constraint::Length(5), // Type selection
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
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(filename_style);
        
        // Handle empty filename case
        if self.filename.is_empty() && self.current_field == DialogField::Filename {
            Paragraph::new("_")
                .style(Style::default().fg(Color::DarkGray))
                .block(filename_block)
                .render(chunks[0], buf);
        } else {
            // Use InputField for non-empty filename
            let filename_field = InputField::new(&self.filename)
                .show_cursor(self.current_field == DialogField::Filename)
                .block(filename_block);
            filename_field.render(chunks[0], buf);
        }
        
        // Template selection
        let template_style = if self.current_field == DialogField::Template {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let template_block = Block::default()
            .title("Template")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
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
            .render(chunks[4], buf);
        
        // Type selection
        let type_style = if self.current_field == DialogField::Type {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let type_block = Block::default()
            .title("Type")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(type_style);
        
        let type_name = match self.prompt_type {
            PromptType::Whole => "Whole",
            PromptType::Instruction => "Instruction",
            PromptType::Context => "Context",
            PromptType::InputIndicator => "Input Indicator",
            PromptType::OutputIndicator => "Output Indicator",
            PromptType::Etc => "Etc",
        };
        
        let type_description = match self.prompt_type {
            PromptType::Whole => "Complete prompt (default)",
            PromptType::Instruction => "Task or instruction for the model",
            PromptType::Context => "External information or context",
            PromptType::InputIndicator => "Input or question marker",
            PromptType::OutputIndicator => "Output format indicator",
            PromptType::Etc => "Other component type",
        };
        
        let type_text = vec![
            Line::from(vec![
                Span::raw("  "),
                if self.current_field == DialogField::Type {
                    Span::styled("◄ ", Style::default().fg(Color::Cyan))
                } else {
                    Span::raw("  ")
                },
                Span::styled(type_name, Style::default().add_modifier(Modifier::BOLD)),
                if self.current_field == DialogField::Type {
                    Span::styled(" ►", Style::default().fg(Color::Cyan))
                } else {
                    Span::raw("  ")
                },
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(type_description, Style::default().fg(Color::DarkGray)),
            ]),
        ];
        
        Paragraph::new(type_text)
            .block(type_block)
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
                        Span::styled("h/l or ←/→", Style::default().fg(Color::Cyan)),
                        Span::raw(" to change template • "),
                        Span::styled("Tab", Style::default().fg(Color::Cyan)),
                        Span::raw(" to switch fields • "),
                        Span::styled("Enter", Style::default().fg(Color::Green)),
                        Span::raw(" to create"),
                    ])
                ]
            }
            DialogField::Type => {
                vec![
                    Line::from(vec![
                        Span::styled("h/l or ←/→", Style::default().fg(Color::Cyan)),
                        Span::raw(" to change type • "),
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
            .render(chunks[6], buf);
    }
}