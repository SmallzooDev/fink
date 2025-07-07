use fink::presentation::tui::tui::{TUIApp, AppMode};
use fink::utils::config::Config;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_starred_prompts_appear_first() {
    // Create a temporary directory with test prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create prompts - some starred, some not
    let prompt1 = r#"---
name: "zebra-prompt"
tags: ["test"]
type: "whole"
---
# Zebra Prompt"#;
    
    let prompt2 = r#"---
name: "alpha-prompt"
tags: ["test", "starred"]
type: "whole"
---
# Alpha Prompt"#;
    
    let prompt3 = r#"---
name: "beta-prompt"
tags: ["test"]
type: "whole"
---
# Beta Prompt"#;
    
    let prompt4 = r#"---
name: "gamma-prompt"
tags: ["test", "starred"]
type: "whole"
---
# Gamma Prompt"#;
    
    fs::write(prompts_dir.join("zebra.md"), prompt1).unwrap();
    fs::write(prompts_dir.join("alpha.md"), prompt2).unwrap();
    fs::write(prompts_dir.join("beta.md"), prompt3).unwrap();
    fs::write(prompts_dir.join("gamma.md"), prompt4).unwrap();
    
    // Create app
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Get all prompts
    let prompts = app.get_filtered_prompts();
    
    // Verify order: starred prompts first (alphabetically), then non-starred (alphabetically)
    assert_eq!(prompts.len(), 4);
    assert_eq!(prompts[0].name, "alpha-prompt");
    assert!(prompts[0].tags.contains(&"starred".to_string()));
    assert_eq!(prompts[1].name, "gamma-prompt");
    assert!(prompts[1].tags.contains(&"starred".to_string()));
    assert_eq!(prompts[2].name, "beta-prompt");
    assert!(!prompts[2].tags.contains(&"starred".to_string()));
    assert_eq!(prompts[3].name, "zebra-prompt");
    assert!(!prompts[3].tags.contains(&"starred".to_string()));
}

#[test]
fn test_starred_sorting_with_filters() {
    // Create a temporary directory with test prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create prompts with different tags
    let prompt1 = r#"---
name: "rust-unstared"
tags: ["rust"]
type: "whole"
---
# Rust Unstarred"#;
    
    let prompt2 = r#"---
name: "rust-starred"
tags: ["rust", "starred"]
type: "whole"
---
# Rust Starred"#;
    
    let prompt3 = r#"---
name: "python-starred"
tags: ["python", "starred"]
type: "whole"
---
# Python Starred"#;
    
    let prompt4 = r#"---
name: "python-unstared"
tags: ["python"]
type: "whole"
---
# Python Unstarred"#;
    
    fs::write(prompts_dir.join("rust-unstarred.md"), prompt1).unwrap();
    fs::write(prompts_dir.join("rust-starred.md"), prompt2).unwrap();
    fs::write(prompts_dir.join("python-starred.md"), prompt3).unwrap();
    fs::write(prompts_dir.join("python-unstarred.md"), prompt4).unwrap();
    
    // Create app
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Apply filter for "rust" tag
    let mut rust_filter = std::collections::HashSet::new();
    rust_filter.insert("rust".to_string());
    app.set_tag_filters(rust_filter);
    
    // Get filtered prompts
    let filtered = app.get_filtered_prompts();
    
    // Should show rust prompts only, with starred first
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].name, "rust-starred");
    assert!(filtered[0].tags.contains(&"starred".to_string()));
    assert_eq!(filtered[1].name, "rust-unstared");
    assert!(!filtered[1].tags.contains(&"starred".to_string()));
}

#[test]
fn test_starred_sorting_with_search() {
    // Create a temporary directory with test prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create prompts
    let prompt1 = r#"---
name: "review-helper"
tags: ["review"]
type: "whole"
---
# Review Helper"#;
    
    let prompt2 = r#"---
name: "code-review"
tags: ["review", "starred"]
type: "whole"
---
# Code Review"#;
    
    let prompt3 = r#"---
name: "review-assistant"
tags: ["review", "starred"]
type: "whole"
---
# Review Assistant"#;
    
    fs::write(prompts_dir.join("review-helper.md"), prompt1).unwrap();
    fs::write(prompts_dir.join("code-review.md"), prompt2).unwrap();
    fs::write(prompts_dir.join("review-assistant.md"), prompt3).unwrap();
    
    // Create app
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Search for "review"
    app.set_search_query("review");
    
    // Get filtered prompts
    let filtered = app.get_filtered_prompts();
    
    // All should match, with starred first
    assert_eq!(filtered.len(), 3);
    assert_eq!(filtered[0].name, "code-review");
    assert!(filtered[0].tags.contains(&"starred".to_string()));
    assert_eq!(filtered[1].name, "review-assistant");
    assert!(filtered[1].tags.contains(&"starred".to_string()));
    assert_eq!(filtered[2].name, "review-helper");
    assert!(!filtered[2].tags.contains(&"starred".to_string()));
}