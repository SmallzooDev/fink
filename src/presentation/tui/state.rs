use crate::presentation::tui::tui::AppMode;
use crate::presentation::tui::components::confirmation_dialog::ConfirmationAction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Next,
    Previous,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIEvent {
    Navigate(Direction),
    ToggleMode,
    Quit,
    ConfirmAction,
    CancelAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    DeletePrompt(String),
    EditPrompt(String),
    CreatePrompt,
    CopyToClipboard(String),
}

pub struct UIState {
    selected_index: usize,
    total_items: usize,
    mode: AppMode,
    should_quit: bool,
    confirmation_message: Option<String>,
    confirmation_action: Option<ConfirmationAction>,
    pending_command: Option<AppCommand>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            total_items: 0,
            mode: AppMode::QuickSelect,
            should_quit: false,
            confirmation_message: None,
            confirmation_action: None,
            pending_command: None,
        }
    }

    pub fn set_total_items(&mut self, total: usize) {
        self.total_items = total;
        // Ensure selected index is within bounds
        if self.selected_index >= total && total > 0 {
            self.selected_index = total - 1;
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn is_showing_confirmation(&self) -> bool {
        self.confirmation_message.is_some()
    }

    pub fn get_confirmation_message(&self) -> Option<String> {
        self.confirmation_message.clone()
    }

    pub fn show_confirmation(&mut self, message: String, action: ConfirmationAction) {
        self.confirmation_message = Some(message);
        self.confirmation_action = Some(action);
    }

    pub fn take_command(&mut self) -> Option<AppCommand> {
        self.pending_command.take()
    }

    pub fn handle_event(&mut self, event: UIEvent) {
        // If showing confirmation, only handle confirmation-related events
        if self.is_showing_confirmation() {
            match event {
                UIEvent::ConfirmAction => {
                    if let Some(action) = self.confirmation_action.take() {
                        match action {
                            ConfirmationAction::Delete(name) => {
                                self.pending_command = Some(AppCommand::DeletePrompt(name));
                            }
                            ConfirmationAction::Overwrite(_path) => {
                                // Handle overwrite in the future
                            }
                        }
                    }
                    self.confirmation_message = None;
                }
                UIEvent::CancelAction => {
                    self.confirmation_message = None;
                    self.confirmation_action = None;
                }
                _ => {
                    // Ignore other events during confirmation
                }
            }
            return;
        }

        // Handle normal events
        match event {
            UIEvent::Navigate(direction) => {
                if self.total_items == 0 {
                    return;
                }
                
                match direction {
                    Direction::Next => {
                        self.selected_index = (self.selected_index + 1) % self.total_items;
                    }
                    Direction::Previous => {
                        if self.selected_index == 0 {
                            self.selected_index = self.total_items - 1;
                        } else {
                            self.selected_index -= 1;
                        }
                    }
                }
            }
            UIEvent::ToggleMode => {
                self.mode = match self.mode {
                    AppMode::QuickSelect => AppMode::Management,
                    AppMode::Management => AppMode::QuickSelect,
                    AppMode::Build => AppMode::QuickSelect,
                };
            }
            UIEvent::Quit => {
                self.should_quit = true;
            }
            UIEvent::CancelAction => {
                // Already handled above for confirmation mode
            }
            UIEvent::ConfirmAction => {
                // Already handled above for confirmation mode
            }
        }
    }
}