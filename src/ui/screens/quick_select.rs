use crate::ui::app::App;
use anyhow::Result;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::path::PathBuf;

pub struct QuickSelectScreen {
    app: App,
}

impl QuickSelectScreen {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let app = App::new(base_path)?;
        Ok(Self { app })
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
        let header = Paragraph::new("jkms Manager")
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
        let footer = Paragraph::new("↑↓: Navigate  Enter: Copy  Esc: Exit")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }
}
