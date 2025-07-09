/// Calculate the visible portion of text that should be displayed in a fixed-width area
pub struct ScrollableText {
    cursor_position: usize,
    view_offset: usize,
}

impl ScrollableText {
    pub fn new() -> Self {
        Self {
            cursor_position: 0,
            view_offset: 0,
        }
    }
    
    /// Get the visible portion of text that fits within the given width
    /// Returns (visible_text, cursor_position_in_view)
    pub fn get_visible_text(&self, text: &str, available_width: usize) -> (String, usize) {
        if text.len() <= available_width {
            // Text fits completely
            return (text.to_string(), self.cursor_position);
        }
        
        // Calculate view window to keep cursor visible
        let mut start = self.view_offset;
        let mut end = start + available_width;
        
        // Adjust window if cursor is outside
        if self.cursor_position < start {
            start = self.cursor_position;
            end = start + available_width;
        } else if self.cursor_position >= end {
            end = self.cursor_position + 1;
            start = end.saturating_sub(available_width);
        }
        
        // Ensure we don't go past the text length
        if end > text.len() {
            end = text.len();
            start = end.saturating_sub(available_width);
        }
        
        let visible = text.chars()
            .skip(start)
            .take(available_width)
            .collect();
        let cursor_in_view = self.cursor_position - start;
        
        (visible, cursor_in_view)
    }
    
    /// Update cursor position and view offset
    pub fn set_cursor(&mut self, position: usize) {
        self.cursor_position = position;
    }
    
    /// Move cursor to the end of text
    pub fn move_to_end(&mut self, text_len: usize) {
        self.cursor_position = text_len;
    }
}

/// Helper function to create scrollable text display with cursor
pub fn format_scrollable_input(text: &str, available_width: usize, show_cursor: bool) -> String {
    let scrollable = ScrollableText::new();
    let (visible_text, _) = scrollable.get_visible_text(text, available_width.saturating_sub(1)); // -1 for cursor
    
    if show_cursor {
        format!("{}_", visible_text)
    } else {
        visible_text
    }
}

/// Calculate the visible portion of text for display, showing the end of the text
pub fn get_visible_text_end_aligned(text: &str, available_width: usize) -> String {
    if text.len() <= available_width {
        text.to_string()
    } else {
        // Show the end of the text
        text.chars()
            .skip(text.len().saturating_sub(available_width))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_fits_completely() {
        let text = "short text";
        let visible = get_visible_text_end_aligned(text, 20);
        assert_eq!(visible, "short text");
    }
    
    #[test]
    fn test_text_needs_scrolling() {
        let text = "this is a very long text that needs scrolling";
        let visible = get_visible_text_end_aligned(text, 10);
        assert_eq!(visible, " scrolling");
        assert_eq!(visible.len(), 10);
    }
    
    #[test]
    fn test_scrollable_text() {
        let mut scrollable = ScrollableText::new();
        let text = "hello world";
        
        let (visible, cursor_pos) = scrollable.get_visible_text(text, 5);
        assert_eq!(visible, "hello");
        assert_eq!(cursor_pos, 0);
        
        scrollable.set_cursor(8);
        let (visible, cursor_pos) = scrollable.get_visible_text(text, 5);
        assert_eq!(visible, "o wor"); // Shows chars 4-8 to keep cursor at position 8 visible
        assert_eq!(cursor_pos, 4); // Cursor is at the end of visible text (position 8 - 4 = 4)
    }
}