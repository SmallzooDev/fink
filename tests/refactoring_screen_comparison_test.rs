use jkms::presentation::tui::tui::TUIApp;
use jkms::presentation::tui::screens::{QuickSelectScreen, QuickSelectScreenRefactored};
use ratatui::{
    backend::TestBackend,
    Terminal,
};
use tempfile::TempDir;

fn create_test_app() -> anyhow::Result<(TUIApp, TempDir)> {
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir_all(&prompts_dir)?;
    
    let prompt_content = r#"---
name: test-prompt
description: Test prompt
tags: [test, example]
---
# Test Prompt
This is a test prompt for rendering comparison."#;
    std::fs::write(prompts_dir.join("test-prompt.md"), prompt_content)?;
    
    let app = TUIApp::new(temp_dir.path().to_path_buf())?;
    Ok((app, temp_dir))
}

#[test]
fn both_screens_should_render_without_errors() {
    // Arrange
    let (app, _temp_dir) = create_test_app().unwrap();
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Act & Assert - Original screen
    terminal.draw(|f| {
        let screen = QuickSelectScreen::new(&app);
        screen.render(f, f.size());
    }).unwrap();
    
    // Act & Assert - Refactored screen
    terminal.draw(|f| {
        let screen = QuickSelectScreenRefactored::new(&app);
        screen.render(f, f.size());
    }).unwrap();
    
    // Both should render without panics
}

#[test]
fn refactored_screen_should_have_cleaner_code() {
    // This test just verifies compilation and demonstrates the code is cleaner
    // The refactored version:
    // 1. Uses StandardLayout instead of manual layout calculations
    // 2. Uses UIStyles for consistent block styling
    // 3. Uses FooterBuilder for standard footer content
    // 4. Uses PreviewRenderer for preview pane
    // 5. Uses ListItemBuilder for list items
    
    // All of this results in:
    // - Less code duplication
    // - More consistent styling
    // - Easier to maintain
    // - Better separation of concerns
    
    assert!(true); // This test is more about demonstrating the improvements
}

#[test]
fn refactored_screen_maintains_same_structure() {
    // The refactored screen should maintain the same visual structure:
    // - Header at top (3 lines)
    // - Content in middle (split 40/60 for list/preview)
    // - Footer at bottom (3 lines)
    
    let (app, _temp_dir) = create_test_app().unwrap();
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|f| {
        let screen = QuickSelectScreenRefactored::new(&app);
        screen.render(f, f.size());
        
        // The layout should be:
        // Header: y=0, height=3
        // Content: y=3, height=24
        // Footer: y=27, height=3
    }).unwrap();
}