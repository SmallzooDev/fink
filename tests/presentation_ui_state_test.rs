use jkms::presentation::tui::state::{UIState, UIEvent, Direction};
use jkms::presentation::tui::tui::AppMode;
use jkms::presentation::tui::components::confirmation_dialog::ConfirmationAction;

#[test]
fn ui_state_should_handle_navigation() {
    // Arrange
    let mut state = UIState::new();
    state.set_total_items(5);
    
    // Act & Assert
    assert_eq!(state.selected_index(), 0);
    
    state.handle_event(UIEvent::Navigate(Direction::Next));
    assert_eq!(state.selected_index(), 1);
    
    state.handle_event(UIEvent::Navigate(Direction::Previous));
    assert_eq!(state.selected_index(), 0);
    
    // Test wrap around
    state.handle_event(UIEvent::Navigate(Direction::Previous));
    assert_eq!(state.selected_index(), 4); // Should wrap to end
}

#[test]
fn ui_state_should_toggle_mode() {
    // Arrange
    let mut state = UIState::new();
    assert_eq!(state.mode(), &AppMode::QuickSelect);
    
    // Act
    state.handle_event(UIEvent::ToggleMode);
    
    // Assert
    assert_eq!(state.mode(), &AppMode::Management);
    
    // Toggle back
    state.handle_event(UIEvent::ToggleMode);
    assert_eq!(state.mode(), &AppMode::QuickSelect);
}

#[test]
fn ui_state_should_manage_quit_state() {
    // Arrange
    let mut state = UIState::new();
    assert!(!state.should_quit());
    
    // Act
    state.handle_event(UIEvent::Quit);
    
    // Assert
    assert!(state.should_quit());
}

#[test]
fn ui_state_should_manage_confirmation_dialog() {
    // Arrange
    let mut state = UIState::new();
    assert!(!state.is_showing_confirmation());
    
    // Act - show confirmation
    state.show_confirmation(
        "Delete this?".to_string(),
        ConfirmationAction::Delete("test".to_string())
    );
    
    // Assert
    assert!(state.is_showing_confirmation());
    assert_eq!(state.get_confirmation_message(), Some("Delete this?".to_string()));
    
    // Act - cancel confirmation
    state.handle_event(UIEvent::CancelAction);
    
    // Assert
    assert!(!state.is_showing_confirmation());
}

#[test]
fn ui_state_should_preserve_selection_on_mode_change() {
    // Arrange
    let mut state = UIState::new();
    state.set_total_items(5);
    state.handle_event(UIEvent::Navigate(Direction::Next));
    state.handle_event(UIEvent::Navigate(Direction::Next));
    assert_eq!(state.selected_index(), 2);
    
    // Act - toggle mode
    state.handle_event(UIEvent::ToggleMode);
    
    // Assert - selection should be preserved
    assert_eq!(state.selected_index(), 2);
    assert_eq!(state.mode(), &AppMode::Management);
}

#[test]
fn ui_state_should_emit_commands() {
    // Arrange
    let mut state = UIState::new();
    
    // Act & Assert - no commands initially
    assert!(state.take_command().is_none());
    
    // Set up a delete confirmation
    state.show_confirmation(
        "Delete?".to_string(),
        ConfirmationAction::Delete("prompt1".to_string())
    );
    
    // Confirm the action
    state.handle_event(UIEvent::ConfirmAction);
    
    // Should emit a delete command
    let command = state.take_command();
    assert!(command.is_some());
    
    match command.unwrap() {
        jkms::presentation::tui::state::AppCommand::DeletePrompt(name) => {
            assert_eq!(name, "prompt1");
        }
        _ => panic!("Expected DeletePrompt command"),
    }
    
    // Command should be consumed
    assert!(state.take_command().is_none());
}

#[test]
fn ui_state_should_handle_events_correctly_in_confirmation_mode() {
    // Arrange
    let mut state = UIState::new();
    state.set_total_items(5);
    state.show_confirmation(
        "Confirm?".to_string(),
        ConfirmationAction::Delete("test".to_string())
    );
    
    // Act - navigation should be blocked during confirmation
    let initial_selection = state.selected_index();
    state.handle_event(UIEvent::Navigate(Direction::Next));
    
    // Assert - selection should not change
    assert_eq!(state.selected_index(), initial_selection);
    assert!(state.is_showing_confirmation());
}