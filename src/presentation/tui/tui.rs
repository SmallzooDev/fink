use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use crate::presentation::tui::components::{PromptList, confirmation_dialog::{ConfirmationDialog as Dialog, ConfirmationAction}, TagManagementDialog, TagFilterDialog, CreateDialog, BuildPanel, InteractiveBuildPanel};
use crate::presentation::tui::screens::ConfigScreen;
use crate::utils::config::Config;
use crate::utils::state::AppState;
use crate::utils::constants::PROMPTS_DIR;
use anyhow::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::collections::HashSet;

const STARRED_TAG: &str = "starred";

#[derive(Debug, PartialEq)]
pub enum AppMode {
    QuickSelect,
    Management,
    Build,
    Config,
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
    active_tag_filters: HashSet<String>,
    tag_management_active: bool,
    pub tag_dialog: Option<TagManagementDialog>,
    tag_filter_dialog_active: bool,
    pub tag_filter_dialog: Option<TagFilterDialog>,
    create_dialog_active: bool,
    pub create_dialog: Option<CreateDialog>,
    build_panel: Option<BuildPanel>,
    interactive_build_panel: Option<InteractiveBuildPanel>,
    error_message: Option<String>,
    success_message: Option<String>,
    init_dialog_active: bool,
    type_prompts_dialog_active: bool,
    config_screen: Option<ConfigScreen>,
    config: Config,
    config_path: PathBuf,
    app_state: AppState,
}

impl TUIApp {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        Self::new_with_mode(base_path, AppMode::QuickSelect)
    }
    
    pub fn new_with_config(config: &Config) -> Result<Self> {
        // For tests, use a temp path
        let config_path = std::env::temp_dir().join("fink_test_config.toml");
        Self::new_with_mode_and_config_path(config, AppMode::QuickSelect, config_path)
    }

    pub fn new_with_mode(_base_path: PathBuf, mode: AppMode) -> Result<Self> {
        // Load the actual config from the default location
        let config_path = Config::default_config_path();
        let config = Config::load_or_create(&config_path)?;
        Self::new_with_mode_and_config_path(&config, mode, config_path)
    }
    
    pub fn new_with_mode_and_config(config: &Config, mode: AppMode) -> Result<Self> {
        // For compatibility, use default path
        let config_path = Config::default_config_path();
        Self::new_with_mode_and_config_path(config, mode, config_path)
    }
    
    pub fn new_with_mode_and_config_path(config: &Config, mode: AppMode, config_path: PathBuf) -> Result<Self> {
        let application = DefaultPromptApplication::with_config(config)?;
        let prompts_metadata = application.list_prompts(None)?;
        let mut prompt_list = PromptList::new(prompts_metadata.clone());
        
        // Load app state and restore cursor position
        let app_state = AppState::load().unwrap_or_default();
        if let Some(last_selected) = app_state.last_selected_prompt() {
            prompt_list.find_and_select(last_selected);
        }
        
        // Check if this is first launch (no .initialized flag and no prompts)
        let prompts_dir = config.storage_path().join(PROMPTS_DIR);
        let init_flag = prompts_dir.join(".initialized");
        let is_first_launch = !init_flag.exists() && prompts_metadata.is_empty();

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
            active_tag_filters: HashSet::new(),
            tag_management_active: false,
            tag_dialog: None,
            tag_filter_dialog_active: false,
            tag_filter_dialog: None,
            create_dialog_active: false,
            create_dialog: None,
            build_panel: None,
            interactive_build_panel: None,
            error_message: None,
            success_message: None,
            init_dialog_active: is_first_launch,
            type_prompts_dialog_active: false,
            config_screen: None,
            config: config.clone(),
            config_path,
            app_state,
        })
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        // Save current state before quitting
        self.save_state();
        self.should_quit = true;
    }
    
    pub fn save_state(&mut self) {
        // Update state with current selection
        if let Some(selected) = self.prompt_list.get_selected() {
            self.app_state.set_last_selected_prompt(Some(selected.name.clone()));
        } else {
            self.app_state.set_last_selected_prompt(None);
        }
        
        // Save state to file
        if let Err(e) = self.app_state.save() {
            // Log error but don't prevent quitting
            eprintln!("Failed to save app state: {}", e);
        }
    }

    pub fn next(&mut self) {
        // Always navigate through the sorted/filtered list since UI always shows it sorted
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
        
        // Save state after navigation
        self.save_state();
    }

    pub fn previous(&mut self) {
        // Always navigate through the sorted/filtered list since UI always shows it sorted
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
        
        // Save state after navigation
        self.save_state();
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
    
    pub fn get_selected_prompt_name(&self) -> Option<String> {
        self.prompt_list.get_selected().map(|p| p.name.clone())
    }

    pub fn get_prompts(&self) -> &Vec<crate::application::models::PromptMetadata> {
        self.prompt_list.prompts()
    }

    pub fn get_list_state(&self) -> ListState {
        let mut state = ListState::default();
        
        // Always use filtered prompts since they are always sorted (starred first)
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
        
        state
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn enter_build_mode(&mut self) {
        self.mode = AppMode::Build;
        let build_prompts = self.get_build_prompts();
        self.interactive_build_panel = Some(InteractiveBuildPanel::new(build_prompts));
    }

    pub fn exit_build_mode(&mut self) {
        self.mode = AppMode::QuickSelect;
        self.build_panel = None;
        self.interactive_build_panel = None;
    }
    
    pub fn enter_config_mode(&mut self) {
        self.mode = AppMode::Config;
        self.config_screen = Some(ConfigScreen::new_with_path(self.config.clone(), self.config_path.clone()));
    }
    
    pub fn exit_config_mode(&mut self) {
        self.mode = AppMode::QuickSelect;
        // If there were changes, reload the config
        if let Some(screen) = &self.config_screen {
            self.config = screen.get_config().clone();
        }
        self.config_screen = None;
    }
    
    pub fn is_config_mode(&self) -> bool {
        matches!(self.mode, AppMode::Config)
    }
    
    pub fn get_config_screen(&self) -> Option<&ConfigScreen> {
        self.config_screen.as_ref()
    }
    
    pub fn get_config_screen_mut(&mut self) -> Option<&mut ConfigScreen> {
        self.config_screen.as_mut()
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_build_prompts(&self) -> Vec<crate::application::models::PromptMetadata> {
        use crate::application::models::PromptType;
        
        self.prompt_list.prompts()
            .iter()
            .filter(|p| !matches!(p.prompt_type, PromptType::Whole))
            .cloned()
            .collect()
    }

    pub fn copy_selected_to_clipboard(&mut self) -> Result<()> {
        if let Some(content) = self.get_selected_content() {
            // Build content with prefix/postfix and proper newlines
            let mut final_content = String::new();
            
            // Add prefix with newline if prefix exists
            let prefix = self.config.clipboard_prefix();
            if !prefix.is_empty() {
                final_content.push_str(prefix);
                final_content.push('\n');
            }
            
            // Add main content
            final_content.push_str(&content);
            
            // Add postfix with newline before it if postfix exists
            let postfix = self.config.clipboard_postfix();
            if !postfix.is_empty() {
                final_content.push('\n');
                final_content.push_str(postfix);
            }
            
            self.application.copy_to_clipboard(&final_content)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::QuickSelect => AppMode::Management,
            AppMode::Management => AppMode::QuickSelect,
            AppMode::Build => AppMode::QuickSelect,
            AppMode::Config => AppMode::QuickSelect,
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

    pub fn reload_prompts(&mut self) -> Result<()> {
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
        let base_prompts = self.prompt_list.prompts();
        
        // Only clone if we need to filter, otherwise work with references
        let filtered: Vec<&crate::application::models::PromptMetadata> = base_prompts
            .iter()
            .filter(|p| {
                // Apply tag filter if active
                if !self.active_tag_filters.is_empty() {
                    if !self.active_tag_filters.iter().any(|tag| p.tags.contains(tag)) {
                        return false;
                    }
                }
                
                // Apply search filter if active
                if !self.search_query.is_empty() {
                    let query_lower = self.search_query.to_lowercase();
                    if !p.name.to_lowercase().contains(&query_lower) {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        // Now clone and sort
        let mut prompts: Vec<_> = filtered.into_iter().cloned().collect();
        
        // Sort prompts: starred first, then alphabetically within each group
        prompts.sort_by_cached_key(|p| {
            let is_starred = !p.tags.iter().any(|t| t == STARRED_TAG);
            (is_starred, p.name.to_lowercase())
        });
        
        prompts
    }
    
    // Tag filtering methods
    pub fn is_tag_filter_active(&self) -> bool {
        self.tag_filter_active
    }
    
    pub fn get_active_tag_filters(&self) -> &HashSet<String> {
        &self.active_tag_filters
    }
    
    pub fn activate_tag_filter(&mut self) {
        self.tag_filter_active = true;
    }
    
    pub fn set_tag_filters(&mut self, tags: HashSet<String>) {
        self.active_tag_filters = tags;
        self.tag_filter_active = !self.active_tag_filters.is_empty();
    }
    
    pub fn add_tag_filter(&mut self, tag: &str) {
        self.active_tag_filters.insert(tag.to_string());
        self.tag_filter_active = true;
    }
    
    pub fn remove_tag_filter(&mut self, tag: &str) {
        self.active_tag_filters.remove(tag);
        self.tag_filter_active = !self.active_tag_filters.is_empty();
    }
    
    pub fn clear_tag_filters(&mut self) {
        self.active_tag_filters.clear();
        self.tag_filter_active = false;
    }
    
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::<&str>::new();
        
        for prompt in self.prompt_list.prompts() {
            for tag in &prompt.tags {
                tags.insert(tag.as_str());
            }
        }
        
        let mut sorted_tags: Vec<String> = tags.into_iter().map(|s| s.to_string()).collect();
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
    
    pub fn toggle_star_on_selected(&mut self) -> Result<()> {
        if let Some(prompt) = self.prompt_list.get_selected() {
            let mut tags = prompt.tags.clone();
            
            if tags.iter().any(|t| t == STARRED_TAG) {
                // Remove star
                tags.retain(|t| t != STARRED_TAG);
                self.application.update_prompt_tags(&prompt.name, tags)?;
                self.set_success("Removed star from prompt".to_string());
            } else {
                // Add star
                tags.push(STARRED_TAG.to_string());
                self.application.update_prompt_tags(&prompt.name, tags)?;
                self.set_success("Added star to prompt".to_string());
            }
            
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
        self.tag_filter_dialog = Some(TagFilterDialog::new(all_tags, self.active_tag_filters.clone()));
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
    
    // Success message methods
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        // Clear any error message when setting success
        self.error_message = None;
    }
    
    pub fn clear_success(&mut self) {
        self.success_message = None;
    }
    
    pub fn has_success(&self) -> bool {
        self.success_message.is_some()
    }
    
    pub fn get_success_message(&self) -> Option<&str> {
        self.success_message.as_deref()
    }
    
    // Build panel methods
    pub fn get_build_panel(&self) -> Option<&BuildPanel> {
        self.build_panel.as_ref()
    }
    
    pub fn get_build_panel_mut(&mut self) -> Option<&mut BuildPanel> {
        self.build_panel.as_mut()
    }
    
    pub fn is_build_mode(&self) -> bool {
        matches!(self.mode, AppMode::Build)
    }
    
    pub fn is_showing_init_dialog(&self) -> bool {
        self.init_dialog_active
    }
    
    pub fn accept_init_dialog(&mut self) -> Result<()> {
        use crate::utils::default_prompts::initialize_default_prompts;
        
        // Initialize default prompts
        let prompts_dir = self.get_base_path().join(PROMPTS_DIR);
        initialize_default_prompts(&prompts_dir)?;
        
        // Reload prompts to show them
        self.reload_prompts()?;
        
        // Close dialog and show type prompts dialog
        self.init_dialog_active = false;
        self.type_prompts_dialog_active = true;
        Ok(())
    }
    
    pub fn decline_init_dialog(&mut self) -> Result<()> {
        // Create .initialized flag so we don't ask again
        let prompts_dir = self.get_base_path().join(PROMPTS_DIR);
        std::fs::create_dir_all(&prompts_dir)?;
        std::fs::write(prompts_dir.join(".initialized"), "")?;
        
        // Close dialog
        self.init_dialog_active = false;
        Ok(())
    }
    
    // Type prompts dialog methods
    pub fn is_showing_type_prompts_dialog(&self) -> bool {
        self.type_prompts_dialog_active
    }
    
    pub fn accept_type_prompts_dialog(&mut self) -> Result<()> {
        use crate::utils::default_prompts::initialize_type_specific_prompts;
        
        // Initialize type-specific prompts
        let prompts_dir = self.get_base_path().join(PROMPTS_DIR);
        initialize_type_specific_prompts(&prompts_dir)?;
        
        // Reload prompts to show them
        self.reload_prompts()?;
        
        // Close dialog
        self.type_prompts_dialog_active = false;
        self.set_success("Type-specific prompts have been initialized!".to_string());
        Ok(())
    }
    
    pub fn decline_type_prompts_dialog(&mut self) {
        self.type_prompts_dialog_active = false;
    }
    
    // Interactive build panel methods
    pub fn get_interactive_build_panel(&self) -> Option<&InteractiveBuildPanel> {
        self.interactive_build_panel.as_ref()
    }
    
    pub fn get_interactive_build_panel_mut(&mut self) -> Option<&mut InteractiveBuildPanel> {
        self.interactive_build_panel.as_mut()
    }
    
    pub fn combine_and_copy_selected_prompts(&mut self) -> Result<()> {
        if let Some(interactive_panel) = &self.interactive_build_panel {
            let selected_prompts = interactive_panel.get_selected_prompt_names();
            
            if selected_prompts.is_empty() {
                return Err(anyhow::anyhow!("No prompts selected"));
            }
            
            let mut combined_content = String::new();
            
            // Add prompts in order
            for (_, prompt_name) in &selected_prompts {
                if !combined_content.is_empty() {
                    combined_content.push_str("\n\n");
                }
                
                // Get the actual content of the prompt
                if let Ok((_, content)) = self.application.get_prompt(prompt_name) {
                    combined_content.push_str(&content);
                }
            }
            
            // Add comment if provided
            let comment = interactive_panel.get_comment();
            if !comment.is_empty() {
                combined_content.push_str("\n\n");
                combined_content.push_str("# Additional Notes:\n");
                combined_content.push_str(comment);
            }
            
            // Copy to clipboard
            self.application.copy_to_clipboard(&combined_content)?;
            
            // Set success message with count
            let prompt_count = selected_prompts.len();
            self.set_success(format!("Successfully combined {} prompts and copied to clipboard!", prompt_count));
            
            // Exit build mode after successful copy
            self.exit_build_mode();
            
            Ok(())
        } else if let Some(build_panel) = &self.build_panel {
            // Fallback to old build panel logic
            let selected_prompts = build_panel.get_selected_prompts();
            
            if selected_prompts.is_empty() {
                return Err(anyhow::anyhow!("No prompts selected"));
            }
            
            // Combine prompts by type order
            use crate::application::models::PromptType;
            let type_order = [
                PromptType::Instruction,
                PromptType::Context,
                PromptType::InputIndicator,
                PromptType::OutputIndicator,
                PromptType::Etc,
            ];
            
            let mut combined_content = String::new();
            
            for prompt_type in &type_order {
                // Get prompts of this type that were selected
                let prompts_of_type: Vec<_> = selected_prompts
                    .iter()
                    .filter(|p| p.prompt_type == *prompt_type)
                    .collect();
                
                // Add prompts of this type to the combined content
                for prompt in prompts_of_type {
                    if !combined_content.is_empty() {
                        combined_content.push_str("\n\n");
                    }
                    
                    // Get the actual content of the prompt
                    if let Ok((_, content)) = self.application.get_prompt(&prompt.name) {
                        combined_content.push_str(&content);
                    }
                }
            }
            
            // Copy to clipboard
            self.application.copy_to_clipboard(&combined_content)?;
            
            // Set success message with count
            let prompt_count = selected_prompts.len();
            self.set_success(format!("Successfully combined {} prompts and copied to clipboard!", prompt_count));
            
            // Exit build mode after successful copy
            self.exit_build_mode();
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Build panel not initialized"))
        }
    }
}
