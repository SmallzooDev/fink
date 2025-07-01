use crate::storage::Prompt;

pub struct PromptList {
    prompts: Vec<Prompt>,
    selected: usize,
}

impl PromptList {
    pub fn new(prompts: Vec<Prompt>) -> Self {
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

    pub fn get_selected(&self) -> Option<&Prompt> {
        self.prompts.get(self.selected)
    }

    pub fn prompts(&self) -> &Vec<Prompt> {
        &self.prompts
    }
}
