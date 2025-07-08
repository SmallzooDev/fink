use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct TypePromptsDialog;

impl TypePromptsDialog {
    pub fn render(&self, f: &mut Frame<'_>, area: Rect) {
        // Calculate centered dialog size
        let dialog_width = 65.min(area.width - 4);
        let dialog_height = 15.min(area.height - 4);
        
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;
        
        let dialog_area = Rect::new(x, y, dialog_width, dialog_height);
        
        // Clear the background
        f.render_widget(Clear, dialog_area);
        
        // Create the dialog block
        let block = Block::default()
            .title(" Additional Prompts Available ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        
        // Create the content
        let text = vec![
            Line::from(""),
            Line::from("Great! Basic prompts have been initialized."),
            Line::from(""),
            Line::from("Would you also like to add type-specific prompts?"),
            Line::from("These prompts are organized by their purpose:"),
            Line::from(""),
            Line::from("  • Instruction - Step-by-step guides & how-tos"),
            Line::from("  • Context - Project & code background info"),
            Line::from("  • Input/Output - Data format specifications"),
            Line::from("  • Etc - Brainstorming & checklists"),
            Line::from(""),
            Line::from("This will add 12 more specialized prompts."),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to add type-specific prompts or "),
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