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

        // Main content area - always split for list and preview
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(chunks[1]);
        
        self.render_prompt_list(f, main_chunks[0]);
        self.render_preview_pane(f, main_chunks[1]);

        // Footer
        let footer_text = match self.app.mode() {
            AppMode::QuickSelect => "↑↓: Navigate  Enter: Copy  m: Manage  Esc: Exit",
            AppMode::Management => "↑↓: Navigate  e: Edit  d: Delete  n: New  m: Quick  Esc: Exit",
        };
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);

        // Render confirmation dialog if showing
        if let Some(dialog) = self.app.get_confirmation_dialog() {
            dialog.render(f, area);
        }
    }

    fn render_prompt_list(&self, f: &mut Frame, area: Rect) {
        let prompts = self.app.get_prompts();
        let items: Vec<ListItem> = prompts
            .iter()
            .map(|p| ListItem::new(Line::from(vec![Span::raw(&p.name)])))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Prompts"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

        let mut list_state = self.app.get_list_state();
        f.render_stateful_widget(list, area, &mut list_state);
    }

    fn render_preview_pane(&self, f: &mut Frame, area: Rect) {
        let content = if let Some(content) = self.app.get_selected_content() {
            content
        } else {
            "No prompt selected".to_string()
        };

        // Add some padding for better readability
        let _inner_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let preview = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Preview")
                .title_alignment(ratatui::layout::Alignment::Center))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((0, 0)); // Allow scrolling in the future

        f.render_widget(preview, area);
    }

}
