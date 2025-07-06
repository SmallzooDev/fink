use jkms::application::models::{PromptMetadata, PromptType};
use jkms::presentation::tui::components::PromptList;

#[test]
fn should_create_prompt_list_component() {
    // Arrange
    let prompts = vec![
        PromptMetadata {
            name: "Code Review".to_string(),
            file_path: "code-review.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
        PromptMetadata {
            name: "Bug Analysis".to_string(),
            file_path: "bug-analysis.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
    ];

    // Act
    let prompt_list = PromptList::new(prompts);

    // Assert
    assert_eq!(prompt_list.len(), 2);
    assert_eq!(prompt_list.selected(), 0);
}

#[test]
fn should_navigate_prompt_list() {
    // Arrange
    let prompts = vec![
        PromptMetadata {
            name: "Code Review".to_string(),
            file_path: "code-review.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
        PromptMetadata {
            name: "Bug Analysis".to_string(),
            file_path: "bug-analysis.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
        PromptMetadata {
            name: "Documentation".to_string(),
            file_path: "documentation.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
    ];
    let mut prompt_list = PromptList::new(prompts);

    // Act
    prompt_list.next();

    // Assert
    assert_eq!(prompt_list.selected(), 1);

    // Act
    prompt_list.next();
    prompt_list.next(); // Should wrap around

    // Assert
    assert_eq!(prompt_list.selected(), 0);
}

#[test]
fn should_handle_previous_navigation() {
    // Arrange
    let prompts = vec![
        PromptMetadata {
            name: "Code Review".to_string(),
            file_path: "code-review.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
        PromptMetadata {
            name: "Bug Analysis".to_string(),
            file_path: "bug-analysis.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
    ];
    let mut prompt_list = PromptList::new(prompts);

    // Act
    prompt_list.previous(); // Should wrap to end

    // Assert
    assert_eq!(prompt_list.selected(), 1);
}

#[test]
fn should_get_selected_prompt() {
    // Arrange
    let prompts = vec![
        PromptMetadata {
            name: "Code Review".to_string(),
            file_path: "code-review.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
        PromptMetadata {
            name: "Bug Analysis".to_string(),
            file_path: "bug-analysis.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        },
    ];
    let mut prompt_list = PromptList::new(prompts);

    // Act
    prompt_list.next();
    let selected = prompt_list.get_selected();

    // Assert
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().name, "Bug Analysis");
}
