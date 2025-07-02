use crate::presentation::tui::tui::{TUIApp, AppMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub struct QuickSelectScreen<'a> {
    app: &'a TUIApp,
}

impl<'a> QuickSelectScreen<'a> {
    pub fn new(app: &'a TUIApp) -> Self {
        Self { app }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let mode_text = match self.app.mode() {
            AppMode::QuickSelect => "jkms Manager - Quick Select",
            AppMode::Management => "jkms Manager - Management Mode",
        };
        let header = Paragraph::new(mode_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Prompt list
        let prompts = self.app.get_prompts();
        let items: Vec<ListItem> = prompts
            .iter()
            .map(|p| ListItem::new(Line::from(vec![Span::raw(&p.name)])))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        let mut list_state = self.app.get_list_state();
        f.render_stateful_widget(list, chunks[1], &mut list_state);

        // Footer
        let footer_text = match self.app.mode() {
            AppMode::QuickSelect => "↑↓: Navigate  Enter: Copy  m: Manage  Esc: Exit",
            AppMode::Management => "↑↓: Navigate  e: Edit  d: Delete  n: New  m: Quick  Esc: Exit",
        };
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }
}
