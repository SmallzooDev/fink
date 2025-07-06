use crate::presentation::tui::tui::TUIApp;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct BuildScreen<'a> {
    app: &'a mut TUIApp,
}

impl<'a> BuildScreen<'a> {
    pub fn new(app: &'a mut TUIApp) -> Self {
        Self { app }
    }
    
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Build panel
                Constraint::Length(3),  // Footer
            ])
            .split(area);
        
        // Header
        let header = Paragraph::new("fink Manager - Build Mode")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);
        
        // Build panel
        if let Some(build_panel) = self.app.get_build_panel_mut() {
            build_panel.render(f, chunks[1]);
        } else {
            // Fallback if build panel is not initialized
            let placeholder = Paragraph::new("No build panel available")
                .block(Block::default()
                    .title("Build Panel")
                    .borders(Borders::ALL));
            f.render_widget(placeholder, chunks[1]);
        }
        
        // Footer
        let footer_text = "↑↓: Navigate  Space: Select/Deselect  Enter: Combine & Copy  Esc: Exit";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_build_screen_creation() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        let jkms_path = temp_path.join("fink");
        fs::create_dir_all(&jkms_path).unwrap();
        
        let mut app = TUIApp::new(temp_path).unwrap();
        app.enter_build_mode();
        
        let _screen = BuildScreen::new(&mut app);
        assert!(app.is_build_mode());
    }
}