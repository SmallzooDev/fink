use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use crate::presentation::tui::components::PromptList;
use anyhow::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    QuickSelect,
    Management,
}

pub struct TUIApp {
    mode: AppMode,
    should_quit: bool,
    prompt_list: PromptList,
    application: DefaultPromptApplication,
}

impl TUIApp {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let application = DefaultPromptApplication::new(base_path)?;
        let prompts_metadata = application.list_prompts(None)?;
        
        // Convert metadata to storage format for compatibility
        let prompts = prompts_metadata.iter().map(|m| crate::storage::Prompt {
            name: m.name.clone(),
            file_path: m.file_path.clone(),
        }).collect();
        
        let prompt_list = PromptList::new(prompts);

        Ok(Self {
            mode: AppMode::QuickSelect,
            should_quit: false,
            prompt_list,
            application,
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

    pub fn get_prompts(&self) -> &Vec<crate::storage::Prompt> {
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
}
