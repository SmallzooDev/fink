use fink::presentation::tui::tui::TUIApp;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_scroll_with_tag_filter() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompts with different tags
    let prompts = vec![
        ("prompt1", r#"---
name: "prompt1"
tags: ["test", "a"]
---
# Prompt 1"#),
        ("prompt2", r#"---
name: "prompt2"
tags: ["test", "b"]
---
# Prompt 2"#),
        ("prompt3", r#"---
name: "prompt3"
tags: ["other", "c"]
---
# Prompt 3"#),
        ("prompt4", r#"---
name: "prompt4"
tags: ["test", "d"]
---
# Prompt 4"#),
    ];
    
    for (name, content) in prompts {
        fs::write(jkms_path.join(format!("{}.md", name)), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Apply tag filter for "test"
    app.set_tag_filter("test");
    
    // Get filtered prompts
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 3); // Should have prompt1, prompt2, prompt4
    
    // Remember initial selection
    let initial_selected = app.get_prompts().get(app.selected_index()).unwrap().name.clone();
    
    // Navigate forward through all filtered items
    let mut visited = vec![initial_selected.clone()];
    for _ in 0..2 {  // We have 3 filtered items, so navigate 2 more times
        app.next();
        let selected = app.get_prompts().get(app.selected_index()).unwrap();
        visited.push(selected.name.clone());
    }
    
    // Should have visited 3 different prompts
    assert_eq!(visited.len(), 3);
    
    // None of them should be prompt3 (which has "other" tag)
    assert!(!visited.contains(&"prompt3".to_string()));
    
    // Should wrap around to initial selection
    app.next();
    let selected = app.get_prompts().get(app.selected_index()).unwrap();
    assert_eq!(selected.name, initial_selected);
    
    // Navigate backward should go to the last filtered item
    app.previous();
    let selected = app.get_prompts().get(app.selected_index()).unwrap();
    assert!(!selected.tags.contains(&"other".to_string())); // Should not be prompt3
}

#[test]
fn test_selection_preserved_after_tag_update() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompts
    let prompts = vec![
        ("prompt1", r#"---
name: "prompt1"
tags: ["a"]
---
# Prompt 1"#),
        ("prompt2", r#"---
name: "prompt2"
tags: ["b"]
---
# Prompt 2"#),
        ("prompt3", r#"---
name: "prompt3"
tags: ["c"]
---
# Prompt 3"#),
    ];
    
    for (name, content) in prompts {
        fs::write(jkms_path.join(format!("{}.md", name)), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Navigate to prompt2
    app.next();
    let selected = app.get_prompts().get(app.selected_index()).unwrap();
    assert_eq!(selected.name, "prompt2");
    
    // Add a tag
    app.add_tag_to_selected("newtag").unwrap();
    
    // Selection should still be on prompt2
    let selected = app.get_prompts().get(app.selected_index()).unwrap();
    assert_eq!(selected.name, "prompt2");
    
    // Verify the tag was added
    assert!(selected.tags.contains(&"newtag".to_string()));
}

#[test]
fn test_list_state_with_filter() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompts
    for i in 0..5 {
        let content = format!(r#"---
name: "prompt{}"
tags: ["{}"]
---
# Prompt {}"#, i, if i % 2 == 0 { "even" } else { "odd" }, i);
        fs::write(jkms_path.join(format!("prompt{}.md", i)), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Apply filter for "even" tag
    app.set_tag_filter("even");
    
    // Navigate to second filtered item (prompt2)
    app.next();
    
    // Get list state
    let state = app.get_list_state();
    
    // The selected index in the ListState should be 1 (second item in filtered list)
    // not the actual index in the full list
    assert_eq!(state.selected(), Some(1));
}