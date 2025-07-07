use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct InitDialog;

impl InitDialog {
    pub fn render(&self, f: &mut Frame<'_>, area: Rect) {
        // Calculate centered dialog size
        let dialog_width = 60.min(area.width - 4);
        let dialog_height = 13.min(area.height - 4);
        
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;
        
        let dialog_area = Rect::new(x, y, dialog_width, dialog_height);
        
        // Clear the background
        f.render_widget(Clear, dialog_area);
        
        // Create the dialog block
        let block = Block::default()
            .title(" Welcome to Fink! ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        
        // Create the content
        let text = vec![
            Line::from(""),
            Line::from("No prompts found. Would you like to initialize"),
            Line::from("with some example prompts to get started?"),
            Line::from(""),
            Line::from("This will create 14 helpful prompts in ~/.fink/prompts/:"),
            Line::from("  • Code review & debugging"),
            Line::from("  • Testing & documentation"),
            Line::from("  • Performance & security"),
            Line::from("  • API design & more..."),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to initialize or "),
                Span::styled("N", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to skip"),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        
        f.render_widget(paragraph, dialog_area);
    }
}