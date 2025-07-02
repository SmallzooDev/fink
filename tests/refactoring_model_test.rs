use jkms::application::models::PromptMetadata;
use jkms::presentation::tui::components::PromptList;

#[test]
fn prompt_list_should_work_with_prompt_metadata() {
    // Arrange
    let metadata = vec![
        PromptMetadata {
            name: "test1".to_string(),
            file_path: "test1.md".to_string(),
            tags: vec!["tag1".to_string()],
        },
        PromptMetadata {
            name: "test2".to_string(),
            file_path: "test2.md".to_string(),
            tags: vec!["tag2".to_string()],
        },
    ];
    
    // Act
    let prompt_list = PromptList::new(metadata);
    
    // Assert
    assert_eq!(prompt_list.len(), 2);
    assert_eq!(prompt_list.selected(), 0);
    
    // Should be able to get selected prompt
    let selected = prompt_list.get_selected();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().name, "test1");
}

#[test]
fn prompt_list_should_navigate_correctly() {
    // Arrange
    let metadata = vec![
        PromptMetadata {
            name: "test1".to_string(),
            file_path: "test1.md".to_string(),
            tags: vec![],
        },
        PromptMetadata {
            name: "test2".to_string(),
            file_path: "test2.md".to_string(),
            tags: vec![],
        },
        PromptMetadata {
            name: "test3".to_string(),
            file_path: "test3.md".to_string(),
            tags: vec![],
        },
    ];
    
    // Act
    let mut prompt_list = PromptList::new(metadata);
    
    // Navigate down
    prompt_list.next();
    assert_eq!(prompt_list.selected(), 1);
    assert_eq!(prompt_list.get_selected().unwrap().name, "test2");
    
    // Navigate down again
    prompt_list.next();
    assert_eq!(prompt_list.selected(), 2);
    assert_eq!(prompt_list.get_selected().unwrap().name, "test3");
    
    // Navigate up
    prompt_list.previous();
    assert_eq!(prompt_list.selected(), 1);
    assert_eq!(prompt_list.get_selected().unwrap().name, "test2");
}