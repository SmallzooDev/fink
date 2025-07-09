use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, ListItem, Paragraph},
    text::{Line, Span},
    Frame,
};

/// Standard layout structure for screens
pub struct StandardLayout {
    pub header: Rect,
    pub content: Rect,
    pub footer: Rect,
}

impl StandardLayout {
    pub fn builder() -> StandardLayoutBuilder {
        StandardLayoutBuilder::default()
    }
}

#[derive(Default)]
pub struct StandardLayoutBuilder {
    header_height: u16,
    footer_height: u16,
}

impl StandardLayoutBuilder {
    pub fn header_height(mut self, height: u16) -> Self {
        self.header_height = height;
        self
    }
    
    pub fn footer_height(mut self, height: u16) -> Self {
        self.footer_height = height;
        self
    }
    
    pub fn build(self, area: Rect) -> StandardLayout {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.header_height),
                Constraint::Min(0),
                Constraint::Length(self.footer_height),
            ])
            .split(area);
            
        StandardLayout {
            header: chunks[0],
            content: chunks[1],
            footer: chunks[2],
        }
    }
}

/// Common UI styles
pub struct UIStyles;

impl UIStyles {
    pub fn header_block(title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan))
    }
    
    pub fn content_block() -> Block<'static> {
        Block::default()
            .borders(Borders::NONE)
    }
    
    pub fn footer_block() -> Block<'static> {
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::DarkGray))
    }
    
    pub fn selection_highlight() -> Style {
        Style::default()
            .bg(Color::DarkGray)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    }
    
    pub fn error_style() -> Style {
        Style::default().fg(Color::Red)
    }
    
    pub fn success_style() -> Style {
        Style::default().fg(Color::Green)
    }
    
    pub fn info_style() -> Style {
        Style::default().fg(Color::Cyan)
    }
}

/// Dialog layout calculator
pub struct DialogLayout;

impl DialogLayout {
    pub fn centered(parent: Rect, size: (u16, u16)) -> Rect {
        let (width, height) = size;
        let x = parent.x + parent.width.saturating_sub(width) / 2;
        let y = parent.y + parent.height.saturating_sub(height) / 2;
        
        Rect {
            x,
            y,
            width: width.min(parent.width),
            height: height.min(parent.height),
        }
    }
}

/// Split pane helper
pub struct SplitPane;

impl SplitPane {
    pub fn horizontal() -> SplitPaneBuilder {
        SplitPaneBuilder { 
            direction: Direction::Horizontal,
            first_ratio: 50,
        }
    }
    
    pub fn vertical() -> SplitPaneBuilder {
        SplitPaneBuilder { 
            direction: Direction::Vertical,
            first_ratio: 50,
        }
    }
}

pub struct SplitPaneBuilder {
    direction: Direction,
    first_ratio: u16,
}

impl SplitPaneBuilder {
    pub fn ratio(mut self, first: u16, _second: u16) -> Self {
        self.first_ratio = first;
        self
    }
    
    pub fn split(self, area: Rect) -> (Rect, Rect) {
        let chunks = Layout::default()
            .direction(self.direction)
            .constraints([
                Constraint::Percentage(self.first_ratio),
                Constraint::Percentage(100 - self.first_ratio),
            ])
            .split(area);
            
        (chunks[0], chunks[1])
    }
}

/// Common footer content builder
pub struct FooterBuilder;

impl FooterBuilder {
    pub fn quick_select_footer() -> Vec<Line<'static>> {
        vec![
            Line::from(vec![
                Span::raw("↑↓: Navigate  "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": Copy  "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": Manage  "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": Exit"),
            ]),
        ]
    }
    
    pub fn management_footer() -> Vec<Line<'static>> {
        vec![
            Line::from(vec![
                Span::styled("[E]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("dit  "),
                Span::styled("[D]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("elete  "),
                Span::styled("[N]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("ew  "),
                Span::styled("[Tab]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" Quick  "),
                Span::styled("[Q]", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("uit"),
            ]),
        ]
    }
    
    pub fn confirmation_footer() -> Vec<Line<'static>> {
        vec![
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("[Y]es", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to confirm or "),
                Span::styled("[N]o", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel"),
            ]),
        ]
    }
}

/// Generic list item builder
pub struct ListItemBuilder;

impl ListItemBuilder {
    pub fn build_prompt_item(name: &str, tags: &[String], is_selected: bool) -> ListItem<'static> {
        let style = if is_selected {
            UIStyles::selection_highlight()
        } else {
            Style::default()
        };
        
        let tags_str = if tags.is_empty() {
            String::new()
        } else {
            format!(" #{}", tags.join(" #"))
        };
        
        let content = format!("{}{}", name, tags_str);
        ListItem::new(content).style(style)
    }
}

/// Preview pane renderer
pub struct PreviewRenderer;

impl PreviewRenderer {
    pub fn render(f: &mut Frame, area: Rect, content: Option<&str>, title: &str) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::DarkGray));
        
        let content = content.unwrap_or("No preview available");
        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        f.render_widget(paragraph, area);
    }
}