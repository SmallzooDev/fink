use fink::presentation::tui::screens::QuickSelectScreen;
use fink::presentation::tui::tui::{TUIApp, AppMode};
use fink::utils::config::Config;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use tempfile::TempDir;

#[test]
fn test_init_dialog_renders_on_first_launch() {
    // Create empty directory
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    // Create app - should show init dialog
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    assert!(app.is_showing_init_dialog());
    
    // Create a test terminal
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Render the screen
    terminal.draw(|f| {
        let screen = QuickSelectScreen::new(&app);
        screen.render(f, f.size());
    }).unwrap();
    
    // Get the buffer content as string
    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            content.push_str(cell.symbol());
        }
        content.push('\n');
    }
    
    // Debug: print the content
    println!("Terminal content:\n{}", content);
    
    // Check that the init dialog text appears
    assert!(content.contains("Welcome to Fink"), "Should show welcome message\nContent: {}", content);
    assert!(content.contains("Would you like to initialize"), "Should show initialization question");
    assert!(content.contains("Y"), "Should show Y key");
    assert!(content.contains("N"), "Should show N key");
}