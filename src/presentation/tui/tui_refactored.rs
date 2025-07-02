use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use crate::presentation::tui::components::{PromptList, confirmation_dialog::{ConfirmationDialog as Dialog, ConfirmationAction}};
use crate::presentation::tui::state::{UIState, UIEvent, Direction, AppCommand};
use crate::presentation::tui::tui::AppMode;
use anyhow::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum PendingAction {
    Edit,
}

pub struct TUIApp {
    ui_state: UIState,
    prompt_list: PromptList,
    application: DefaultPromptApplication,
    pending_action: Option<PendingAction>,
}

impl TUIApp {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        Self::new_with_mode(base_path, AppMode::QuickSelect)
    }

    pub fn new_with_mode(base_path: PathBuf, mode: AppMode) -> Result<Self> {
        let application = DefaultPromptApplication::new(base_path)?;
        let prompts_metadata = application.list_prompts(None)?;
        let prompt_list = PromptList::new(prompts_metadata);
        
        let mut ui_state = UIState::new();
        ui_state.set_total_items(prompt_list.len());
        
        // Set initial mode if not QuickSelect
        if mode != AppMode::QuickSelect {
            ui_state.handle_event(UIEvent::ToggleMode);
        }

        Ok(Self {
            ui_state,
            prompt_list,
            application,
            pending_action: None,
        })
    }

    pub fn mode(&self) -> &AppMode {
        self.ui_state.mode()
    }

    pub fn should_quit(&self) -> bool {
        self.ui_state.should_quit()
    }

    pub fn quit(&mut self) {
        self.ui_state.handle_event(UIEvent::Quit);
    }

    pub fn next(&mut self) {
        self.ui_state.handle_event(UIEvent::Navigate(Direction::Next));
        self.sync_selection_to_prompt_list();
    }

    pub fn previous(&mut self) {
        self.ui_state.handle_event(UIEvent::Navigate(Direction::Previous));
        self.sync_selection_to_prompt_list();
    }

    pub fn selected_index(&self) -> usize {
        self.ui_state.selected_index()
    }

    pub fn get_selected_content(&self) -> Option<String> {
        self.prompt_list.get_selected().and_then(|prompt| {
            self.application
                .get_prompt(&prompt.name)
                .map(|(_, content)| content)
                .ok()
        })
    }

    pub fn get_prompts(&self) -> &Vec<crate::application::models::PromptMetadata> {
        self.prompt_list.prompts()
    }

    pub fn get_list_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.ui_state.selected_index()));
        state
    }

    pub fn copy_selected_to_clipboard(&mut self) -> Result<()> {
        if let Some(content) = self.get_selected_content() {
            self.application.copy_to_clipboard(&content)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }

    pub fn toggle_mode(&mut self) {
        self.ui_state.handle_event(UIEvent::ToggleMode);
    }

    pub fn edit_selected(&mut self) -> Result<()> {
        if let Some(prompt) = self.prompt_list.get_selected() {
            // Delegate to the application layer
            self.application.edit_prompt(&prompt.name)?;
            
            // Reload prompts after editing
            self.reload_prompts()?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }

    pub fn delete_selected(&mut self) -> Result<()> {
        // This method is now deprecated in favor of show_delete_confirmation
        // But we'll keep it for backward compatibility
        self.show_delete_confirmation();
        Ok(())
    }

    pub fn create_new_prompt(&mut self) -> Result<()> {
        // For now, create a simple prompt
        // Later we can add a prompt creation dialog
        let name = format!("new-prompt-{}", chrono::Utc::now().timestamp());
        self.application.create_prompt(&name, None)?;
        
        // Reload prompts after creation
        self.reload_prompts()?;
        
        Ok(())
    }

    fn reload_prompts(&mut self) -> Result<()> {
        let prompts_metadata = self.application.list_prompts(None)?;
        let current_selection = self.ui_state.selected_index();
        self.prompt_list = PromptList::new(prompts_metadata);
        self.ui_state.set_total_items(self.prompt_list.len());
        
        // Restore selection if possible
        if current_selection < self.prompt_list.len() {
            for _ in 0..current_selection {
                self.prompt_list.next();
            }
        }
        
        Ok(())
    }

    pub fn get_base_path(&self) -> &std::path::Path {
        self.application.get_base_path()
    }

    pub fn set_pending_action(&mut self, action: Option<PendingAction>) {
        self.pending_action = action;
    }

    pub fn take_pending_action(&mut self) -> Option<PendingAction> {
        self.pending_action.take()
    }

    pub fn is_showing_confirmation(&self) -> bool {
        self.ui_state.is_showing_confirmation()
    }

    pub fn get_confirmation_message(&self) -> Option<String> {
        self.ui_state.get_confirmation_message()
    }
    
    pub fn get_confirmation_dialog(&self) -> Option<Dialog> {
        if let Some(message) = self.get_confirmation_message() {
            if let Some(prompt) = self.prompt_list.get_selected() {
                // Reconstruct dialog from UI state
                // This is a bit hacky but maintains backward compatibility
                Some(Dialog::new(
                    message,
                    ConfirmationAction::Delete(prompt.name.clone()),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn show_delete_confirmation(&mut self) {
        if let Some(prompt) = self.prompt_list.get_selected() {
            self.ui_state.show_confirmation(
                format!("Are you sure you want to delete '{}'?", prompt.name),
                ConfirmationAction::Delete(prompt.name.clone()),
            );
        }
    }

    pub fn cancel_confirmation(&mut self) {
        self.ui_state.handle_event(UIEvent::CancelAction);
    }

    pub fn confirm_action(&mut self) -> Result<()> {
        self.ui_state.handle_event(UIEvent::ConfirmAction);
        self.process_pending_commands()
    }

    // New methods for UIState integration
    pub fn handle_ui_event(&mut self, event: UIEvent) {
        self.ui_state.handle_event(event);
        
        // Sync selection after navigation events
        match event {
            UIEvent::Navigate(_) => self.sync_selection_to_prompt_list(),
            _ => {}
        }
    }

    pub fn process_pending_commands(&mut self) -> Result<()> {
        if let Some(command) = self.ui_state.take_command() {
            match command {
                AppCommand::DeletePrompt(name) => {
                    self.application.delete_prompt(&name, true)?;
                    self.reload_prompts()?;
                }
                AppCommand::EditPrompt(name) => {
                    self.application.edit_prompt(&name)?;
                    self.reload_prompts()?;
                }
                AppCommand::CreatePrompt => {
                    self.create_new_prompt()?;
                }
                AppCommand::CopyToClipboard(content) => {
                    self.application.copy_to_clipboard(&content)?;
                }
            }
        }
        Ok(())
    }

    fn sync_selection_to_prompt_list(&mut self) {
        let target_index = self.ui_state.selected_index();
        
        // Reset prompt list selection to 0
        while self.prompt_list.selected() > 0 {
            self.prompt_list.previous();
        }
        
        // Move to target index
        for _ in 0..target_index {
            self.prompt_list.next();
        }
    }
}