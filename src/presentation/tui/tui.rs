use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use crate::presentation::tui::components::{PromptList, confirmation_dialog::{ConfirmationDialog as Dialog, ConfirmationAction}, TagManagementDialog, TagFilterDialog, CreateDialog};
use crate::utils::config::Config;
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
    tag_filter_active: bool,
    active_tag_filter: Option<String>,
    tag_management_active: bool,
    pub tag_dialog: Option<TagManagementDialog>,
    tag_filter_dialog_active: bool,
    pub tag_filter_dialog: Option<TagFilterDialog>,
    create_dialog_active: bool,
    pub create_dialog: Option<CreateDialog>,
    error_message: Option<String>,
}

impl TUIApp {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        Self::new_with_mode(base_path, AppMode::QuickSelect)
    }
    
    pub fn new_with_config(config: &Config) -> Result<Self> {
        Self::new_with_mode_and_config(config, AppMode::QuickSelect)
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
            tag_filter_active: false,
            active_tag_filter: None,
            tag_management_active: false,
            tag_dialog: None,
            tag_filter_dialog_active: false,
            tag_filter_dialog: None,
            create_dialog_active: false,
            create_dialog: None,
            error_message: None,
        })
    }
    
    pub fn new_with_mode_and_config(config: &Config, mode: AppMode) -> Result<Self> {
        let application = DefaultPromptApplication::with_config(config)?;
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
            tag_filter_active: false,
            active_tag_filter: None,
            tag_management_active: false,
            tag_dialog: None,
            tag_filter_dialog_active: false,
            tag_filter_dialog: None,
            create_dialog_active: false,
            create_dialog: None,
            error_message: None,
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
        if self.is_search_active() || self.is_tag_filter_active() {
            // Navigate only through filtered prompts
            let filtered_prompts = self.get_filtered_prompts();
            if filtered_prompts.is_empty() {
                return;
            }
            
            // Find current prompt in filtered list
            if let Some(current) = self.prompt_list.get_selected() {
                if let Some(current_index) = filtered_prompts.iter().position(|p| p.name == current.name) {
                    // Get next index in filtered list
                    let next_index = (current_index + 1) % filtered_prompts.len();
                    let next_prompt = &filtered_prompts[next_index];
                    
                    // Find and select in main list
                    self.prompt_list.find_and_select(&next_prompt.name);
                } else {
                    // Current selection not in filtered list, select first filtered item
                    if let Some(first) = filtered_prompts.first() {
                        self.prompt_list.find_and_select(&first.name);
                    }
                }
            } else {
                // No selection, select first filtered item
                if let Some(first) = filtered_prompts.first() {
                    self.prompt_list.find_and_select(&first.name);
                }
            }
        } else {
            // No filter active, use normal navigation
            self.prompt_list.next();
        }
    }

    pub fn previous(&mut self) {
        if self.is_search_active() || self.is_tag_filter_active() {
            // Navigate only through filtered prompts
            let filtered_prompts = self.get_filtered_prompts();
            if filtered_prompts.is_empty() {
                return;
            }
            
            // Find current prompt in filtered list
            if let Some(current) = self.prompt_list.get_selected() {
                if let Some(current_index) = filtered_prompts.iter().position(|p| p.name == current.name) {
                    // Get previous index in filtered list
                    let prev_index = if current_index == 0 {
                        filtered_prompts.len() - 1
                    } else {
                        current_index - 1
                    };
                    let prev_prompt = &filtered_prompts[prev_index];
                    
                    // Find and select in main list
                    self.prompt_list.find_and_select(&prev_prompt.name);
                } else {
                    // Current selection not in filtered list, select last filtered item
                    if let Some(last) = filtered_prompts.last() {
                        self.prompt_list.find_and_select(&last.name);
                    }
                }
            } else {
                // No selection, select last filtered item
                if let Some(last) = filtered_prompts.last() {
                    self.prompt_list.find_and_select(&last.name);
                }
            }
        } else {
            // No filter active, use normal navigation
            self.prompt_list.previous();
        }
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
        
        if self.is_search_active() || self.is_tag_filter_active() {
            // When filtered, find the index in the filtered list
            let filtered_prompts = self.get_filtered_prompts();
            if let Some(selected) = self.prompt_list.get_selected() {
                if let Some(index) = filtered_prompts.iter().position(|p| p.name == selected.name) {
                    state.select(Some(index));
                } else {
                    // Selected item not in filtered list
                    state.select(None);
                }
            } else {
                state.select(None);
            }
        } else {
            // No filter, use the actual index
            state.select(Some(self.prompt_list.selected()));
        }
        
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
        // Open the create dialog instead of creating immediately
        self.open_create_dialog();
        Ok(())
    }

    fn reload_prompts(&mut self) -> Result<()> {
        let prompts_metadata = self.application.list_prompts(None)?;
        self.prompt_list.update_prompts(prompts_metadata);
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
        let mut prompts = self.prompt_list.prompts().clone();
        
        // Apply tag filter first if active
        if let Some(tag) = &self.active_tag_filter {
            prompts = self.application
                .search_prompts(tag, crate::application::models::SearchType::Tags)
                .unwrap_or_else(|_| Vec::new());
        }
        
        // Then apply search filter if active
        if !self.search_query.is_empty() {
            if self.active_tag_filter.is_some() {
                // If tag filter is active, further filter by name search
                let query_lower = self.search_query.to_lowercase();
                prompts = prompts.into_iter()
                    .filter(|p| p.name.to_lowercase().contains(&query_lower))
                    .collect();
            } else {
                // Otherwise use application layer's search
                prompts = self.application
                    .search_prompts(&self.search_query, crate::application::models::SearchType::Name)
                    .unwrap_or_else(|_| Vec::new());
            }
        }
        
        prompts
    }
    
    // Tag filtering methods
    pub fn is_tag_filter_active(&self) -> bool {
        self.tag_filter_active
    }
    
    pub fn get_active_tag_filter(&self) -> Option<&String> {
        self.active_tag_filter.as_ref()
    }
    
    pub fn activate_tag_filter(&mut self) {
        self.tag_filter_active = true;
    }
    
    pub fn set_tag_filter(&mut self, tag: &str) {
        self.active_tag_filter = Some(tag.to_string());
        self.tag_filter_active = true;
    }
    
    pub fn clear_tag_filter(&mut self) {
        self.active_tag_filter = None;
        self.tag_filter_active = false;
    }
    
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::new();
        
        for prompt in self.prompt_list.prompts() {
            for tag in &prompt.tags {
                tags.insert(tag.clone());
            }
        }
        
        let mut sorted_tags: Vec<String> = tags.into_iter().collect();
        sorted_tags.sort();
        sorted_tags
    }
    
    // Tag management methods
    pub fn add_tag_to_selected(&mut self, tag: &str) -> Result<()> {
        if let Some(prompt) = self.prompt_list.get_selected() {
            let mut tags = prompt.tags.clone();
            
            // Check if tag already exists
            if tags.contains(&tag.to_string()) {
                return Err(anyhow::anyhow!("Tag '{}' already exists", tag));
            }
            
            // Add the new tag
            tags.push(tag.to_string());
            
            // Update the prompt with new tags
            self.application.update_prompt_tags(&prompt.name, tags)?;
            
            // Reload prompts to reflect changes
            self.reload_prompts()?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }
    
    pub fn remove_tag_from_selected(&mut self, tag: &str) -> Result<()> {
        if let Some(prompt) = self.prompt_list.get_selected() {
            let mut tags = prompt.tags.clone();
            
            // Check if tag exists
            if !tags.contains(&tag.to_string()) {
                return Err(anyhow::anyhow!("Tag '{}' not found", tag));
            }
            
            // Remove the tag
            tags.retain(|t| t != tag);
            
            // Update the prompt with new tags
            self.application.update_prompt_tags(&prompt.name, tags)?;
            
            // Reload prompts to reflect changes
            self.reload_prompts()?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }
    
    pub fn open_tag_management(&mut self) {
        let tags = self.get_selected_prompt_tags();
        self.tag_dialog = Some(TagManagementDialog::new(tags));
        self.tag_management_active = true;
    }
    
    pub fn close_tag_management(&mut self) {
        self.tag_dialog = None;
        self.tag_management_active = false;
    }
    
    pub fn get_tag_dialog(&self) -> Option<&TagManagementDialog> {
        self.tag_dialog.as_ref()
    }
    
    pub fn get_tag_dialog_mut(&mut self) -> Option<&mut TagManagementDialog> {
        self.tag_dialog.as_mut()
    }
    
    pub fn is_tag_management_active(&self) -> bool {
        self.tag_management_active
    }
    
    pub fn get_selected_prompt_tags(&self) -> Vec<String> {
        if let Some(prompt) = self.prompt_list.get_selected() {
            prompt.tags.clone()
        } else {
            Vec::new()
        }
    }
    
    // Tag filter dialog methods
    pub fn open_tag_filter(&mut self) {
        let all_tags = self.get_all_tags();
        self.tag_filter_dialog = Some(TagFilterDialog::new(all_tags, self.active_tag_filter.clone()));
        self.tag_filter_dialog_active = true;
    }
    
    pub fn close_tag_filter(&mut self) {
        self.tag_filter_dialog = None;
        self.tag_filter_dialog_active = false;
    }
    
    pub fn is_tag_filter_dialog_active(&self) -> bool {
        self.tag_filter_dialog_active
    }
    
    pub fn get_tag_filter_dialog(&self) -> Option<&TagFilterDialog> {
        self.tag_filter_dialog.as_ref()
    }
    
    pub fn get_tag_filter_dialog_mut(&mut self) -> Option<&mut TagFilterDialog> {
        self.tag_filter_dialog.as_mut()
    }
    
    // Create dialog methods
    pub fn open_create_dialog(&mut self) {
        self.create_dialog = Some(CreateDialog::new());
        self.create_dialog_active = true;
    }
    
    pub fn close_create_dialog(&mut self) {
        self.create_dialog = None;
        self.create_dialog_active = false;
    }
    
    pub fn is_create_dialog_active(&self) -> bool {
        self.create_dialog_active
    }
    
    pub fn get_create_dialog(&self) -> Option<&CreateDialog> {
        self.create_dialog.as_ref()
    }
    
    pub fn get_create_dialog_mut(&mut self) -> Option<&mut CreateDialog> {
        self.create_dialog.as_mut()
    }
    
    pub fn confirm_create(&mut self) -> Result<()> {
        if let Some(dialog) = &self.create_dialog {
            if dialog.is_valid() {
                let filename = dialog.get_normalized_filename();
                let prompt_type = dialog.get_prompt_type();
                
                match dialog.get_template() {
                    crate::presentation::tui::components::CreateTemplate::FromClipboard => {
                        // Get clipboard content and create prompt with it
                        let clipboard_content = self.application.get_clipboard_content().ok();
                        self.application.create_prompt_with_content_and_type(&filename, Some("clipboard"), clipboard_content, prompt_type)?;
                    },
                    crate::presentation::tui::components::CreateTemplate::Basic => {
                        self.application.create_prompt_with_type(&filename, Some("basic"), prompt_type)?;
                    },
                    crate::presentation::tui::components::CreateTemplate::Default => {
                        self.application.create_prompt_with_type(&filename, None, prompt_type)?;
                    },
                };
                
                self.close_create_dialog();
                self.reload_prompts()?;
            }
        }
        Ok(())
    }
    
    // Error message methods
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }
    
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
    
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }
    
    pub fn get_error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}
