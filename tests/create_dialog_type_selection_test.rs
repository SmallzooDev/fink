#[cfg(test)]
mod tests {
    use jkms::presentation::tui::components::{CreateDialog, DialogField};
    use jkms::application::models::PromptType;

    #[test]
    fn should_have_type_field_in_dialog() {
        let mut dialog = CreateDialog::new();
        
        // Should have Type as one of the fields
        dialog.next_field();
        assert_eq!(dialog.current_field(), DialogField::Type);
    }

    #[test]
    fn should_default_to_whole_type() {
        let dialog = CreateDialog::new();
        assert_eq!(dialog.get_prompt_type(), PromptType::Whole);
    }

    #[test]
    fn should_cycle_through_prompt_types() {
        let mut dialog = CreateDialog::new();
        
        // Navigate to type field
        dialog.next_field();
        
        // Cycle through types
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::Instruction);
        
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::Context);
        
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::InputIndicator);
        
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::OutputIndicator);
        
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::Etc);
        
        dialog.next_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::Whole);
    }

    #[test]
    fn should_cycle_backwards_through_prompt_types() {
        let mut dialog = CreateDialog::new();
        
        // Navigate to type field
        dialog.next_field();
        
        // Cycle backwards
        dialog.previous_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::Etc);
        
        dialog.previous_type();
        assert_eq!(dialog.get_prompt_type(), PromptType::OutputIndicator);
    }

    #[test]
    fn should_navigate_between_all_fields() {
        let mut dialog = CreateDialog::new();
        
        // Start at Filename
        assert_eq!(dialog.current_field(), DialogField::Filename);
        
        // Move to Type
        dialog.next_field();
        assert_eq!(dialog.current_field(), DialogField::Type);
        
        // Move to Template
        dialog.next_field();
        assert_eq!(dialog.current_field(), DialogField::Template);
        
        // Wrap back to Filename
        dialog.next_field();
        assert_eq!(dialog.current_field(), DialogField::Filename);
        
        // Test backwards navigation
        dialog.previous_field();
        assert_eq!(dialog.current_field(), DialogField::Template);
        
        dialog.previous_field();
        assert_eq!(dialog.current_field(), DialogField::Type);
        
        dialog.previous_field();
        assert_eq!(dialog.current_field(), DialogField::Filename);
    }
}