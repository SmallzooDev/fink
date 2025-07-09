use crate::presentation::tui::tui::{TUIApp, AppMode};
use crate::presentation::tui::components::search::HighlightedText;
use crate::presentation::tui::components::input_field::InputField;
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
                    Constraint::Length(6),  // Footer (now 6 lines for vertical layout)
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Min(0),     // Main content
                    Constraint::Length(6),  // Footer (now 6 lines for vertical layout)
                ])
                .split(area)
        };

        // Header
        let mode_text = match self.app.mode() {
            AppMode::QuickSelect => "Quick Select",
            AppMode::Management => "Management Mode",
            AppMode::Build => "Build Mode",
            AppMode::Config => "Configuration",
        };
        
        // Add active filters to header if any
        let active_filters = self.app.get_active_tag_filters();
        let header_text = if active_filters.is_empty() {
            mode_text.to_string()
        } else {
            let filter_tags: Vec<String> = active_filters.iter().cloned().collect();
            format!("{} | Filtering by: {}", mode_text, filter_tags.join(", "))
        };
        
        let header = Paragraph::new(header_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded));
        f.render_widget(header, chunks[0]);

        // Search bar (if active)
        let main_content_index = if self.app.is_search_active() {
            self.render_search_bar(f, chunks[1]);
            2
        } else {
            1
        };

        // Main content area - render differently based on mode
        if self.app.is_build_mode() {
            // In build mode, we can't render the interactive panel here due to borrowing rules
            // The panel will be rendered separately in the main render loop
            let placeholder = Paragraph::new("Build mode active - Interactive panel rendering...")
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded));
            f.render_widget(placeholder, chunks[main_content_index]);
        } else {
            // Normal mode - split for list and preview
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(60),
                ])
                .split(chunks[main_content_index]);
            
            self.render_prompt_list(f, main_chunks[0]);
            self.render_preview_pane(f, main_chunks[1]);
        }

        // Footer
        let footer_index = if self.app.is_search_active() { 3 } else { 2 };
        
        // Split footer into two parts: mode selector and commands (vertically)
        let footer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Mode selector
                Constraint::Length(3),  // Commands
            ])
            .split(chunks[footer_index]);
        
        // Mode selector box
        let mode_text = match self.app.mode() {
            AppMode::QuickSelect => "[Quick] m:Manage b:Build c:Config",
            AppMode::Management => "[Manage] m:Quick b:Build c:Config",
            AppMode::Build => "[Build] m:Quick",
            AppMode::Config => "[Config] Esc:Exit",
        };
        let mode_selector = Paragraph::new(mode_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Mode "));
        f.render_widget(mode_selector, footer_chunks[0]);
        
        // Commands box
        let commands_text = if self.app.is_search_active() {
            "Type to search  Enter: Select  Esc: Cancel search"
        } else {
            match self.app.mode() {
                AppMode::QuickSelect => "↑↓: Navigate  Enter: Copy  s: Star  /: Search  f: Filter  F: Clear Filters  Esc: Exit",
                AppMode::Management => "↑↓: Navigate  e: Edit  d: Delete  n: New  s: Star  t: Tags  f: Filter  F: Clear  /: Search  Esc: Exit",
                AppMode::Build => "↑↓: Navigate  Space: Select  Enter: Combine  Esc: Back",
                AppMode::Config => "Tab: Switch fields  Ctrl+S: Save  Esc: Exit",
            }
        };
        let commands = Paragraph::new(commands_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Commands "));
        f.render_widget(commands, footer_chunks[1]);

        // Render initialization dialog if showing
        if self.app.is_showing_init_dialog() {
            let init_dialog = crate::presentation::tui::components::InitDialog;
            init_dialog.render(f, area);
        }
        
        // Render type prompts dialog if showing
        if self.app.is_showing_type_prompts_dialog() {
            let type_prompts_dialog = crate::presentation::tui::components::TypePromptsDialog;
            type_prompts_dialog.render(f, area);
        }
        
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
        
        // Render create dialog if showing
        if let Some(create_dialog) = self.app.get_create_dialog() {
            // Calculate centered area for dialog
            let dialog_width = 60;
            let dialog_height = 20;
            let x = (area.width.saturating_sub(dialog_width)) / 2;
            let y = (area.height.saturating_sub(dialog_height)) / 2;
            
            let dialog_area = Rect {
                x: area.x + x,
                y: area.y + y,
                width: dialog_width.min(area.width),
                height: dialog_height.min(area.height),
            };
            
            f.render_widget(create_dialog, dialog_area);
        }
        
        // Render editing dialog if editing externally
        if self.app.is_editing_external() {
            let editing_dialog = crate::presentation::tui::components::EditingDialog;
            editing_dialog.render(f, area);
        }
        
        // Render error message if present
        if let Some(error_msg) = self.app.get_error_message() {
            let error_width = 60.min(area.width - 4);
            let error_height = 6;
            let x = (area.width.saturating_sub(error_width)) / 2;
            let y = (area.height.saturating_sub(error_height)) / 2;
            
            let error_area = Rect {
                x: area.x + x,
                y: area.y + y,
                width: error_width,
                height: error_height,
            };
            
            // Clear the area
            f.render_widget(ratatui::widgets::Clear, error_area);
            
            // Render error box
            let error_text = vec![
                Line::from(""),
                Line::from(Span::styled(error_msg, Style::default().fg(Color::White))),
                Line::from(""),
                Line::from(Span::styled(
                    "Press any key to continue", 
                    Style::default().fg(Color::DarkGray)
                )),
            ];
            
            let error_widget = Paragraph::new(error_text)
                .block(Block::default()
                    .title(" Error ")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Red)))
                .alignment(ratatui::layout::Alignment::Center);
                
            f.render_widget(error_widget, error_area);
        }
        
        // Render success message if present
        if let Some(success_msg) = self.app.get_success_message() {
            let success_width = 60.min(area.width - 4);
            let success_height = 6;
            let x = (area.width.saturating_sub(success_width)) / 2;
            let y = (area.height.saturating_sub(success_height)) / 2;
            
            let success_area = Rect {
                x: area.x + x,
                y: area.y + y,
                width: success_width,
                height: success_height,
            };
            
            // Clear the area
            f.render_widget(ratatui::widgets::Clear, success_area);
            
            // Render success box
            let success_text = vec![
                Line::from(""),
                Line::from(Span::styled(success_msg, Style::default().fg(Color::White))),
                Line::from(""),
                Line::from(Span::styled(
                    "Press any key to continue", 
                    Style::default().fg(Color::DarkGray)
                )),
            ];
            
            let success_widget = Paragraph::new(success_text)
                .block(Block::default()
                    .title(" Success ")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green)))
                .alignment(ratatui::layout::Alignment::Center);
                
            f.render_widget(success_widget, success_area);
        }
    }

    fn render_prompt_list(&self, f: &mut Frame, area: Rect) {
        let prompts = self.app.get_filtered_prompts();
        
        let search_query = if self.app.is_search_active() && !self.app.get_search_query().is_empty() {
            Some(self.app.get_search_query())
        } else {
            None
        };
        
        let highlighter = HighlightedText::new();
        
        let items: Vec<ListItem> = prompts
            .iter()
            .map(|p| {
                // Check if prompt is starred
                let is_starred = p.tags.iter().any(|t| t == "starred");
                let star_prefix = if is_starred { "⭐ " } else { "   " };
                
                if let Some(query) = search_query {
                    let highlighted = highlighter.highlight(&p.name, query);
                    let mut spans: Vec<Span> = vec![
                        Span::styled(star_prefix, Style::default().fg(Color::Yellow))
                    ];
                    spans.extend(highlighted.segments
                        .into_iter()
                        .map(|seg| {
                            if seg.is_match {
                                Span::styled(seg.text, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                            } else {
                                Span::raw(seg.text)
                            }
                        }));
                    ListItem::new(Line::from(spans))
                } else {
                    ListItem::new(Line::from(vec![
                        Span::styled(star_prefix, Style::default().fg(Color::Yellow)),
                        Span::raw(&p.name)
                    ]))
                }
            })
            .collect();

        let title = if !self.app.get_active_tag_filters().is_empty() {
            let filters: Vec<String> = self.app.get_active_tag_filters().iter().cloned().collect();
            format!("Prompts (tags: {}) - {} results", filters.join(", "), prompts.len())
        } else if self.app.is_search_active() && !self.app.get_search_query().is_empty() {
            format!("Prompts (filtered: {})", prompts.len())
        } else {
            "Prompts".to_string()
        };

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(title))
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
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Preview")
                .title_alignment(ratatui::layout::Alignment::Center))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((0, 0)); // Allow scrolling in the future

        f.render_widget(preview, area);
    }

    fn render_search_bar(&self, f: &mut Frame, area: Rect) {
        let search_query = self.app.get_search_query();
        
        let search_field = InputField::new(search_query)
            .prefix("Search: ")
            .show_cursor(true)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Search (Esc to cancel)")
                .border_style(Style::default().fg(Color::Yellow)));
        
        f.render_widget(search_field, area);
    }

}
