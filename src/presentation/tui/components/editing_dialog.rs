use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub struct EditingDialog;

impl EditingDialog {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Calculate dialog size
        let dialog_width = 60;
        let dialog_height = 10;
        
        let dialog_area = centered_rect(dialog_width, dialog_height, area);
        
        // Clear the area behind the dialog
        f.render_widget(Clear, dialog_area);
        
        // Main dialog block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" External Editor ")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::Yellow));
        
        let inner = block.inner(dialog_area);
        f.render_widget(block, dialog_area);
        
        // Layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Message
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Instructions
            ])
            .split(inner);
        
        // Message
        let message = vec![
            Line::from(vec![
                Span::styled("Editing in external editor...", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("The prompt is open in "),
                Span::styled("VS Code", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            ]),
        ];
        
        let message_widget = Paragraph::new(message)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(message_widget, chunks[0]);
        
        // Instructions
        let instructions = Line::from(vec![
            Span::raw("Press "),
            Span::styled("e", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" again to refresh the prompt"),
        ]);
        
        let instructions_widget = Paragraph::new(instructions)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(instructions_widget, chunks[2]);
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