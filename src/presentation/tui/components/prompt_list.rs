use crate::application::models::PromptMetadata;

pub struct PromptList {
    prompts: Vec<PromptMetadata>,
    selected: usize,
}

impl PromptList {
    pub fn new(prompts: Vec<PromptMetadata>) -> Self {
        Self {
            prompts,
            selected: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.prompts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.prompts.is_empty()
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn next(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.prompts.len();
    }

    pub fn previous(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.prompts.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn get_selected(&self) -> Option<&PromptMetadata> {
        self.prompts.get(self.selected)
    }

    pub fn prompts(&self) -> &Vec<PromptMetadata> {
        &self.prompts
    }
    
    pub fn set_selected(&mut self, index: usize) {
        if index < self.prompts.len() {
            self.selected = index;
        }
    }
    
    pub fn find_and_select(&mut self, name: &str) -> bool {
        if let Some(index) = self.prompts.iter().position(|p| p.name == name) {
            self.selected = index;
            true
        } else {
            false
        }
    }
    
    pub fn update_prompts(&mut self, new_prompts: Vec<PromptMetadata>) {
        // Save current selection name
        let current_name = self.get_selected().map(|p| p.name.clone());
        
        // Update prompts
        self.prompts = new_prompts;
        
        // Try to restore selection
        if let Some(name) = current_name {
            if !self.find_and_select(&name) {
                // If the prompt was not found, reset to 0
                self.selected = 0;
            }
        } else {
            self.selected = 0;
        }
        
        // Ensure selected is within bounds
        if self.selected >= self.prompts.len() && !self.prompts.is_empty() {
            self.selected = self.prompts.len() - 1;
        }
    }
}
