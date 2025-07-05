use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use crate::presentation::tui::components::{PromptList, confirmation_dialog::{ConfirmationDialog as Dialog, ConfirmationAction}};
use anyhow::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    QuickSelect,
    Management,
}

#[derive(Debug, PartialEq)]
pub enum PendingAction {
    Edit,
}

pub struct TUIApp {
    mode: AppMode,
    should_quit: bool,
    prompt_list: PromptList,
    application: DefaultPromptApplication,
    pending_action: Option<PendingAction>,
    confirmation_dialog: Option<Dialog>,
    search_active: bool,
    search_query: String,
}

impl TUIApp {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        Self::new_with_mode(base_path, AppMode::QuickSelect)
    }

    pub fn new_with_mode(base_path: PathBuf, mode: AppMode) -> Result<Self> {
        let application = DefaultPromptApplication::new(base_path)?;
        let prompts_metadata = application.list_prompts(None)?;
        let prompt_list = PromptList::new(prompts_metadata);

        Ok(Self {
            mode,
            should_quit: false,
            prompt_list,
            application,
            pending_action: None,
            confirmation_dialog: None,
            search_active: false,
            search_query: String::new(),
        })
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next(&mut self) {
        self.prompt_list.next();
    }

    pub fn previous(&mut self) {
        self.prompt_list.previous();
    }

    pub fn selected_index(&self) -> usize {
        self.prompt_list.selected()
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
        state.select(Some(self.prompt_list.selected()));
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
        self.mode = match self.mode {
            AppMode::QuickSelect => AppMode::Management,
            AppMode::Management => AppMode::QuickSelect,
        };
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
        self.prompt_list = PromptList::new(prompts_metadata);
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
        self.confirmation_dialog.is_some()
    }

    pub fn get_confirmation_message(&self) -> Option<String> {
        self.confirmation_dialog.as_ref().map(|d| d.get_message().to_string())
    }
    
    pub fn get_confirmation_dialog(&self) -> Option<&Dialog> {
        self.confirmation_dialog.as_ref()
    }

    pub fn show_delete_confirmation(&mut self) {
        if let Some(prompt) = self.prompt_list.get_selected() {
            self.confirmation_dialog = Some(Dialog::new(
                format!("Are you sure you want to delete '{}'?", prompt.name),
                ConfirmationAction::Delete(prompt.name.clone()),
            ));
        }
    }

    pub fn cancel_confirmation(&mut self) {
        self.confirmation_dialog = None;
    }

    pub fn confirm_action(&mut self) -> Result<()> {
        if let Some(dialog) = self.confirmation_dialog.take() {
            match dialog.get_action() {
                ConfirmationAction::Delete(name) => {
                    self.application.delete_prompt(name, true)?;
                    self.reload_prompts()?;
                }
                _ => {} // Handle other action types in the future
            }
        }
        Ok(())
    }

    pub fn is_search_active(&self) -> bool {
        self.search_active
    }

    pub fn activate_search(&mut self) {
        self.search_active = true;
        self.search_query.clear();
    }

    pub fn deactivate_search(&mut self) {
        self.search_active = false;
        self.search_query.clear();
    }

    pub fn set_search_query(&mut self, query: &str) {
        self.search_query = query.to_string();
    }

    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    pub fn get_filtered_prompts(&self) -> Vec<crate::application::models::PromptMetadata> {
        if self.search_query.is_empty() {
            self.prompt_list.prompts().clone()
        } else {
            // Use the application layer's search functionality
            self.application
                .search_prompts(&self.search_query, crate::application::models::SearchType::Name)
                .unwrap_or_else(|_| Vec::new())
        }
    }
}
