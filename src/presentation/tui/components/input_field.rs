use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph, Widget},
};
use crate::utils::text_scroll::get_visible_text_end_aligned;

/// A reusable input field widget with automatic text scrolling
pub struct InputField<'a> {
    /// The full text content
    text: &'a str,
    /// Optional prefix (like "Search: ")
    prefix: Option<&'a str>,
    /// Whether to show a cursor at the end
    show_cursor: bool,
    /// Style for the text
    style: Style,
    /// Optional block for borders and title
    block: Option<Block<'a>>,
}

impl<'a> InputField<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            prefix: None,
            show_cursor: true,
            style: Style::default(),
            block: None,
        }
    }
    
    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }
    
    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }
    
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for InputField<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = match &self.block {
            Some(b) => {
                let inner = b.inner(area);
                b.clone().render(area, buf);
                inner
            }
            None => area,
        };
        
        // Calculate available width
        let available_width = inner_area.width as usize;
        let prefix_len = self.prefix.map(|p| p.len()).unwrap_or(0);
        let cursor_space = if self.show_cursor { 1 } else { 0 };
        let text_width = available_width.saturating_sub(prefix_len + cursor_space);
        
        // Get visible portion of text
        let visible_text = if text_width > 0 {
            get_visible_text_end_aligned(self.text, text_width)
        } else {
            String::new()
        };
        
        // Build the display text
        let display_text = match self.prefix {
            Some(prefix) => {
                if self.show_cursor {
                    format!("{}{}_", prefix, visible_text)
                } else {
                    format!("{}{}", prefix, visible_text)
                }
            }
            None => {
                if self.show_cursor {
                    format!("{}_", visible_text)
                } else {
                    visible_text
                }
            }
        };
        
        Paragraph::new(display_text)
            .style(self.style)
            .render(inner_area, buf);
    }
}

/// Helper function to create a scrollable input field with common settings
pub fn scrollable_input<'a>(
    text: &'a str,
    prefix: Option<&'a str>,
    show_cursor: bool,
    style: Style,
    block: Option<Block<'a>>,
) -> InputField<'a> {
    let mut field = InputField::new(text)
        .show_cursor(show_cursor)
        .style(style);
    
    if let Some(p) = prefix {
        field = field.prefix(p);
    }
    
    if let Some(b) = block {
        field = field.block(b);
    }
    
    field
}