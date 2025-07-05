use crate::presentation::tui::tui::{TUIApp, AppMode};
use crate::presentation::tui::components::search::HighlightedText;
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
        let chunks = if self.app.is_search_active() {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Length(3),  // Search bar
                    Constraint::Min(0),     // Main content
                    Constraint::Length(3),  // Footer
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Min(0),     // Main content
                    Constraint::Length(3),  // Footer
                ])
                .split(area)
        };

        // Header
        let mode_text = match self.app.mode() {
            AppMode::QuickSelect => "jkms Manager - Quick Select",
            AppMode::Management => "jkms Manager - Management Mode",
        };
        let header = Paragraph::new(mode_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Search bar (if active)
        let main_content_index = if self.app.is_search_active() {
            self.render_search_bar(f, chunks[1]);
            2
        } else {
            1
        };

        // Main content area - always split for list and preview
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(chunks[main_content_index]);
        
        self.render_prompt_list(f, main_chunks[0]);
        self.render_preview_pane(f, main_chunks[1]);

        // Footer
        let footer_index = if self.app.is_search_active() { 3 } else { 2 };
        let footer_text = if self.app.is_search_active() {
            "Type to search  Enter: Select  Esc: Cancel search"
        } else {
            match self.app.mode() {
                AppMode::QuickSelect => "↑↓: Navigate  Enter: Copy  /: Search  f: Filter  m: Manage  Esc: Exit",
                AppMode::Management => "↑↓: Navigate  e: Edit  d: Delete  n: New  t: Tags  f: Filter  /: Search  m: Quick  Esc: Exit",
            }
        };
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[footer_index]);

        // Render confirmation dialog if showing
        if let Some(dialog) = self.app.get_confirmation_dialog() {
            dialog.render(f, area);
        }
        
        // Render tag management dialog if showing
        if let Some(tag_dialog) = self.app.get_tag_dialog() {
            tag_dialog.render(f, area);
        }
        
        // Render tag filter dialog if showing
        if let Some(filter_dialog) = self.app.get_tag_filter_dialog() {
            filter_dialog.render(f, area);
        }
    }

    fn render_prompt_list(&self, f: &mut Frame, area: Rect) {
        let prompts = if self.app.is_search_active() || self.app.is_tag_filter_active() {
            self.app.get_filtered_prompts()
        } else {
            self.app.get_prompts().clone()
        };
        
        let search_query = if self.app.is_search_active() && !self.app.get_search_query().is_empty() {
            Some(self.app.get_search_query())
        } else {
            None
        };
        
        let highlighter = HighlightedText::new();
        
        let items: Vec<ListItem> = prompts
            .iter()
            .map(|p| {
                if let Some(query) = search_query {
                    let highlighted = highlighter.highlight(&p.name, query);
                    let spans: Vec<Span> = highlighted.segments
                        .into_iter()
                        .map(|seg| {
                            if seg.is_match {
                                Span::styled(seg.text, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                            } else {
                                Span::raw(seg.text)
                            }
                        })
                        .collect();
                    ListItem::new(Line::from(spans))
                } else {
                    ListItem::new(Line::from(vec![Span::raw(&p.name)]))
                }
            })
            .collect();

        let title = if let Some(tag_filter) = self.app.get_active_tag_filter() {
            format!("Prompts (tag: {}) - {} results", tag_filter, prompts.len())
        } else if self.app.is_search_active() && !self.app.get_search_query().is_empty() {
            format!("Prompts (filtered: {})", prompts.len())
        } else {
            "Prompts".to_string()
        };

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
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

    fn render_search_bar(&self, f: &mut Frame, area: Rect) {
        let search_query = self.app.get_search_query();
        let search_text = format!("Search: {}_", search_query);
        
        let search_bar = Paragraph::new(search_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Search (Esc to cancel)")
                .border_style(Style::default().fg(Color::Yellow)));
        
        f.render_widget(search_bar, area);
    }

}
