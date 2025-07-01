use crate::core::PromptManager;
use crate::external::ClipboardManager;
use crate::storage::{FileSystem, Prompt};
use crate::ui::components::PromptList;
use anyhow::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    QuickSelect,
    Management,
}

pub struct App {
    mode: AppMode,
    should_quit: bool,
    prompt_list: PromptList,
    prompt_manager: PromptManager,
    clipboard: ClipboardManager,
}

impl App {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let storage = FileSystem::new(base_path.clone());
        let prompts = storage.list_prompts()?;
        let prompt_list = PromptList::new(prompts.clone());
        let prompt_manager = PromptManager::new(base_path);

        Ok(Self {
            mode: AppMode::QuickSelect,
            should_quit: false,
            prompt_list,
            prompt_manager,
            clipboard: ClipboardManager::new(),
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
            self.prompt_manager
                .get_prompt_content(&prompt.file_path)
                .ok()
        })
    }

    pub fn get_prompts(&self) -> &Vec<Prompt> {
        self.prompt_list.prompts()
    }

    pub fn get_list_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.prompt_list.selected()));
        state
    }

    pub fn copy_selected_to_clipboard(&mut self) -> Result<()> {
        if let Some(content) = self.get_selected_content() {
            self.clipboard.copy(&content)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No prompt selected"))
        }
    }
}
