use jkms::presentation::ui::screens::QuickSelectScreen;
use jkms::presentation::ui::app::App;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use tempfile::tempdir;

#[test]
fn should_render_quick_select_screen() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test1.md"), "# Test 1").unwrap();
    std::fs::write(prompts_dir.join("test2.md"), "# Test 2").unwrap();

    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create App instance
    let app = App::new(temp_dir.path().to_path_buf()).unwrap();

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

    assert!(content.contains("jkms Manager"));
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
