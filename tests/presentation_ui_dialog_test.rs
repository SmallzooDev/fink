use jkms::presentation::tui::components::confirmation_dialog::{ConfirmationDialog, ConfirmationAction};
use ratatui::Terminal;
use ratatui::backend::TestBackend;

#[test]
fn confirmation_dialog_should_render_message() {
    // Arrange
    let dialog = ConfirmationDialog::new(
        "Are you sure you want to delete this?".to_string(),
        ConfirmationAction::Delete("test-prompt".to_string()),
    );
    
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Act
    terminal
        .draw(|f| {
            let area = f.size();
            dialog.render(f, area);
        })
        .unwrap();
    
    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);
    
    assert!(content.contains("Are you sure you want to delete this?"));
    assert!(content.contains("Confirmation"));
    assert!(content.contains("[Y]es") || content.contains("(Y)es"));
    assert!(content.contains("[N]o") || content.contains("(N)o"));
}

#[test]
fn confirmation_dialog_should_center_in_area() {
    // Arrange
    let dialog = ConfirmationDialog::new(
        "Delete?".to_string(),
        ConfirmationAction::Delete("test".to_string()),
    );
    
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Act
    terminal
        .draw(|f| {
            let area = f.size();
            dialog.render(f, area);
        })
        .unwrap();
    
    // Assert
    let buffer = terminal.backend().buffer();
    let content = buffer_to_string(buffer);
    
    // Dialog should be roughly centered (check that "Delete?" appears somewhere in the middle)
    let lines: Vec<&str> = content.lines().collect();
    let mut found_line = None;
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("Delete?") {
            found_line = Some(i);
            break;
        }
    }
    
    // Should be found somewhere in the middle third of the screen
    let line_num = found_line.expect("Dialog message not found");
    assert!(line_num > 5 && line_num < 25, "Dialog not vertically centered");
}

#[test]
fn confirmation_dialog_should_have_consistent_dimensions() {
    // Arrange
    let dialog = ConfirmationDialog::new(
        "Test message".to_string(),
        ConfirmationAction::Delete("test".to_string()),
    );
    
    // Act
    let dimensions = dialog.get_dimensions();
    
    // Assert
    assert_eq!(dimensions.0, 60); // width
    assert_eq!(dimensions.1, 7);  // height
}

#[test]
fn confirmation_dialog_should_handle_different_action_types() {
    // Arrange
    let delete_dialog = ConfirmationDialog::new(
        "Delete?".to_string(),
        ConfirmationAction::Delete("file1".to_string()),
    );
    
    let overwrite_dialog = ConfirmationDialog::new(
        "Overwrite?".to_string(),
        ConfirmationAction::Overwrite("file2".to_string()),
    );
    
    // Act & Assert
    assert_eq!(delete_dialog.get_action(), &ConfirmationAction::Delete("file1".to_string()));
    assert_eq!(overwrite_dialog.get_action(), &ConfirmationAction::Overwrite("file2".to_string()));
}

fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
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