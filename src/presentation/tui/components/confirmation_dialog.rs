use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmationAction {
    Delete(String),
    Overwrite(String),
}

pub struct ConfirmationDialog {
    message: String,
    action: ConfirmationAction,
}

impl ConfirmationDialog {
    pub fn new(message: String, action: ConfirmationAction) -> Self {
        Self { message, action }
    }
    
    pub fn get_dimensions(&self) -> (u16, u16) {
        (60, 7) // width, height
    }
    
    pub fn get_action(&self) -> &ConfirmationAction {
        &self.action
    }
    
    pub fn get_message(&self) -> &str {
        &self.message
    }
    
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let (width, height) = self.get_dimensions();
        
        // Calculate centered position
        let x = area.width.saturating_sub(width) / 2;
        let y = area.height.saturating_sub(height) / 2;
        
        let dialog_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: width.min(area.width),
            height: height.min(area.height),
        };
        
        // Clear the background
        f.render_widget(Clear, dialog_area);
        
        // Create dialog content
        let text = vec![
            Line::from(""),
            Line::from(Span::raw(&self.message)),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[Y]es", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to confirm or "),
                Span::styled("[N]o", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel"),
            ]),
        ];
        
        let dialog = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow))
                .title("Confirmation")
                .title_alignment(Alignment::Center))
            .alignment(Alignment::Center);
        
        f.render_widget(dialog, dialog_area);
    }
}