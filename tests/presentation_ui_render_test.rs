use fink::presentation::tui::screens::QuickSelectScreen;
use fink::presentation::tui::app::{TUIApp, AppMode};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use tempfile::tempdir;

#[test]
fn should_render_quick_select_screen() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test1.md"), "# Test 1").unwrap();
    std::fs::write(prompts_dir.join("test2.md"), "# Test 2").unwrap();

    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance
    let app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    terminal
        .draw(|f| {
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
        })
        .unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);

    assert!(content.contains("fink Manager"));
    assert!(content.contains("test1"));
    assert!(content.contains("test2"));
}

fn buffer_to_string(buffer: &Buffer) -> String {
    let mut result = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            result.push_str(cell.symbol());
        }
        result.push('\n');
    }
    result
}

#[test]
fn should_render_preview_pane_in_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Test Prompt"
tags: ["test", "demo"]
---
# Test Prompt
This is a test prompt content.
It has multiple lines.
And shows in preview."#;

    std::fs::write(prompts_dir.join("test-prompt.md"), content).unwrap();

    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance in Management mode
    let app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();

    // Act
    terminal
        .draw(|f| {
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
        })
        .unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);

    // Should show management mode in header
    assert!(content.contains("Management Mode"));
    
    // Should show the prompt name
    assert!(content.contains("Test Prompt"));
    
    // Should show preview content
    assert!(content.contains("This is a test prompt content"));
    assert!(content.contains("It has multiple lines"));
}

#[test]
fn should_show_preview_metadata_in_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content1 = r#"---
name: "First Prompt"
tags: ["tag1", "tag2"]
---
# First Prompt
First content."#;

    let content2 = r#"---
name: "Second Prompt"
tags: ["tag3", "tag4"]
---
# Second Prompt
Second content."#;

    std::fs::write(prompts_dir.join("first.md"), content1).unwrap();
    std::fs::write(prompts_dir.join("second.md"), content2).unwrap();

    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance in Management mode
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    
    // Navigate to second prompt
    app.next();

    // Act
    terminal
        .draw(|f| {
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
        })
        .unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);

    // Should show second prompt in preview
    assert!(content.contains("Second Prompt"));
    assert!(content.contains("Second content"));
}

#[test]
fn should_show_preview_pane_in_quick_select_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Quick Select Prompt"
tags: ["quick", "test"]
---
# Quick Select Prompt
This is content shown in quick select mode.
It should appear in the preview pane."#;

    std::fs::write(prompts_dir.join("quick-prompt.md"), content).unwrap();

    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance in QuickSelect mode (default)
    let app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    terminal
        .draw(|f| {
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
        })
        .unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);

    // Should show quick select mode in header
    assert!(content.contains("Quick Select"));
    
    // Should show the prompt name in list
    assert!(content.contains("Quick Select Prompt"));
    
    // Should show preview content
    assert!(content.contains("This is content shown in quick select mode"));
    assert!(content.contains("It should appear in the preview pane"));
}

#[test]
fn should_render_confirmation_dialog() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Test Prompt"
tags: ["test"]
---
# Test Prompt
Content to delete."#;

    std::fs::write(prompts_dir.join("test.md"), content).unwrap();

    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance in Management mode with confirmation dialog
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    app.show_delete_confirmation();

    // Act
    terminal
        .draw(|f| {
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
        })
        .unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);

    // Should show confirmation dialog
    assert!(content.contains("Are you sure you want to delete 'Test Prompt'?"));
    assert!(content.contains("[Y]es") || content.contains("(Y)es"));
    assert!(content.contains("[N]o") || content.contains("(N)o"));
}
