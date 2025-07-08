use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub struct UpdateDialog {
    pub update_notes: String,
    pub should_close: bool,
}

impl UpdateDialog {
    pub fn new(update_notes: String) -> Self {
        Self {
            update_notes,
            should_close: false,
        }
    }
    
    pub fn close(&mut self) {
        self.should_close = true;
    }
    
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Calculate dialog size - make it wider and taller for update notes
        let dialog_width = 80.min(area.width - 4);
        let dialog_height = 20.min(area.height - 4);
        
        // Center the dialog
        let dialog_area = centered_rect(dialog_width, dialog_height, area);
        
        // Clear the area behind the dialog
        f.render_widget(Clear, dialog_area);
        
        // Create the main block
        let block = Block::default()
            .title(" 🎉 Update Notes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(ratatui::widgets::BorderType::Rounded);
        
        let inner = block.inner(dialog_area);
        f.render_widget(block, dialog_area);
        
        // Layout for content and help text
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),      // Update notes
                Constraint::Length(2),   // Help text
            ])
            .split(inner);
        
        // Render update notes with word wrap
        let notes_paragraph = Paragraph::new(Text::from(self.update_notes.as_str()))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .scroll((0, 0));
        
        f.render_widget(notes_paragraph, chunks[0]);
        
        // Render help text
        let help_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("Enter", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" to continue"),
            ]),
        ];
        
        let help_paragraph = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        
        f.render_widget(help_paragraph, chunks[1]);
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