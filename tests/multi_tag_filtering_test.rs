use fink::presentation::tui::app::{TUIApp, AppMode};
use fink::utils::config::Config;
use tempfile::TempDir;
use std::fs;
use std::collections::HashSet;

#[test]
fn test_multi_tag_filtering() {
    // Create a temporary directory with test prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create prompts with different tags
    let prompt1 = r#"---
name: "Rust Code Review"
tags: ["rust", "code", "review"]
type: "whole"
---
# Rust Code Review"#;
    
    let prompt2 = r#"---
name: "Python Debug"
tags: ["python", "debug"]
type: "whole"
---
# Python Debug"#;
    
    let prompt3 = r#"---
name: "Rust Debug"
tags: ["rust", "debug"]
type: "whole"
---
# Rust Debug"#;
    
    let prompt4 = r#"---
name: "JavaScript Code"
tags: ["javascript", "code"]
type: "whole"
---
# JavaScript Code"#;
    
    fs::write(prompts_dir.join("rust-review.md"), prompt1).unwrap();
    fs::write(prompts_dir.join("python-debug.md"), prompt2).unwrap();
    fs::write(prompts_dir.join("rust-debug.md"), prompt3).unwrap();
    fs::write(prompts_dir.join("js-code.md"), prompt4).unwrap();
    
    // Create app
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Test 1: No filters - should show all prompts
    let all_prompts = app.get_filtered_prompts();
    assert_eq!(all_prompts.len(), 4, "Should show all 4 prompts without filters");
    
    // Test 2: Single tag filter
    let mut single_filter = HashSet::new();
    single_filter.insert("rust".to_string());
    app.set_tag_filters(single_filter);
    
    let rust_prompts = app.get_filtered_prompts();
    assert_eq!(rust_prompts.len(), 2, "Should show 2 prompts with 'rust' tag");
    assert!(rust_prompts.iter().any(|p| p.name == "Rust Code Review"));
    assert!(rust_prompts.iter().any(|p| p.name == "Rust Debug"));
    
    // Test 3: Multiple tag filters (OR operation)
    let mut multi_filter = HashSet::new();
    multi_filter.insert("rust".to_string());
    multi_filter.insert("python".to_string());
    app.set_tag_filters(multi_filter);
    
    let multi_prompts = app.get_filtered_prompts();
    assert_eq!(multi_prompts.len(), 3, "Should show 3 prompts with 'rust' OR 'python' tags");
    assert!(multi_prompts.iter().any(|p| p.name == "Rust Code Review"));
    assert!(multi_prompts.iter().any(|p| p.name == "Rust Debug"));
    assert!(multi_prompts.iter().any(|p| p.name == "Python Debug"));
    
    // Test 4: Clear filters
    app.clear_tag_filters();
    let cleared_prompts = app.get_filtered_prompts();
    assert_eq!(cleared_prompts.len(), 4, "Should show all prompts after clearing filters");
    
    // Test 5: Filter with search combined
    let mut code_filter = HashSet::new();
    code_filter.insert("code".to_string());
    app.set_tag_filters(code_filter);
    app.set_search_query("Rust");
    
    let combined_filter = app.get_filtered_prompts();
    assert_eq!(combined_filter.len(), 1, "Should show only 'Rust Code Review' with combined filters");
    assert_eq!(combined_filter[0].name, "Rust Code Review");
}

#[test]
fn test_tag_filter_dialog_search() {
    use fink::presentation::tui::components::TagFilterDialog;
    
    let tags = vec![
        "rust".to_string(),
        "python".to_string(),
        "javascript".to_string(),
        "code".to_string(),
        "debug".to_string(),
        "test".to_string(),
    ];
    
    let mut dialog = TagFilterDialog::new(tags.clone(), HashSet::new());
    
    // Test search functionality
    dialog.add_char('r');
    dialog.add_char('u');
    let filtered = dialog.get_filtered_tags();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], "rust");
    
    // Clear search
    dialog.clear_search();
    let all_tags = dialog.get_filtered_tags();
    assert_eq!(all_tags.len(), 6);
    
    // Test partial search
    dialog.add_char('c');
    dialog.add_char('o');
    let code_tags = dialog.get_filtered_tags();
    assert_eq!(code_tags.len(), 1); // Only "code" contains "co"
    assert!(code_tags.contains(&"code".to_string()));
}

#[test]
fn test_tag_filter_dialog_selection() {
    use fink::presentation::tui::components::TagFilterDialog;
    
    let tags = vec![
        "rust".to_string(),
        "python".to_string(),
        "code".to_string(),
    ];
    
    let mut dialog = TagFilterDialog::new(tags.clone(), HashSet::new());
    
    // Toggle mode to selection
    dialog.toggle_mode();
    assert!(!dialog.is_searching);
    
    // Select first tag
    dialog.toggle_selected_tag();
    let selected = dialog.get_selected_tags();
    assert_eq!(selected.len(), 1);
    assert!(selected.contains("rust"));
    
    // Move down and select another
    dialog.move_down();
    dialog.toggle_selected_tag();
    let selected = dialog.get_selected_tags();
    assert_eq!(selected.len(), 2);
    assert!(selected.contains("rust"));
    assert!(selected.contains("python"));
    
    // Deselect first tag
    dialog.move_up();
    dialog.toggle_selected_tag();
    let selected = dialog.get_selected_tags();
    assert_eq!(selected.len(), 1);
    assert!(selected.contains("python"));
    
    // Clear all
    dialog.clear_selection();
    let selected = dialog.get_selected_tags();
    assert!(selected.is_empty());
}

#[test]
fn test_tag_filter_active_state() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Initially no filter is active
    assert!(!app.is_tag_filter_active());
    assert!(app.get_active_tag_filters().is_empty());
    
    // Add a filter
    app.add_tag_filter("rust");
    assert!(app.is_tag_filter_active());
    assert_eq!(app.get_active_tag_filters().len(), 1);
    
    // Add another filter
    app.add_tag_filter("code");
    assert!(app.is_tag_filter_active());
    assert_eq!(app.get_active_tag_filters().len(), 2);
    
    // Remove one filter
    app.remove_tag_filter("rust");
    assert!(app.is_tag_filter_active());
    assert_eq!(app.get_active_tag_filters().len(), 1);
    
    // Remove last filter - should deactivate
    app.remove_tag_filter("code");
    assert!(!app.is_tag_filter_active());
    assert!(app.get_active_tag_filters().is_empty());
}