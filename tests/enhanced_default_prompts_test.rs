use fink::utils::default_prompts::{initialize_default_prompts, DEFAULT_PROMPTS};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_should_have_at_least_10_default_prompts() {
    // We should have a rich set of default prompts
    assert!(
        DEFAULT_PROMPTS.len() >= 10, 
        "Should have at least 10 default prompts, but only have {}", 
        DEFAULT_PROMPTS.len()
    );
}

#[test]
fn test_default_prompts_should_cover_diverse_use_cases() {
    let prompt_names: Vec<&str> = DEFAULT_PROMPTS.iter().map(|p| p.name).collect();
    
    // Check for essential prompt categories
    assert!(prompt_names.iter().any(|&n| n.contains("code") || n.contains("review")), 
        "Should have code review prompts");
    assert!(prompt_names.iter().any(|&n| n.contains("debug") || n.contains("fix")), 
        "Should have debugging prompts");
    assert!(prompt_names.iter().any(|&n| n.contains("explain")), 
        "Should have explanation prompts");
    assert!(prompt_names.iter().any(|&n| n.contains("test")), 
        "Should have testing prompts");
    assert!(prompt_names.iter().any(|&n| n.contains("doc") || n.contains("comment")), 
        "Should have documentation prompts");
    assert!(prompt_names.iter().any(|&n| n.contains("refactor") || n.contains("improve")), 
        "Should have refactoring prompts");
}

#[test]
fn test_all_default_prompts_have_proper_metadata() {
    for prompt in DEFAULT_PROMPTS {
        assert!(!prompt.name.is_empty(), "Prompt name should not be empty");
        assert!(!prompt.description.is_empty(), "Prompt description should not be empty");
        assert!(!prompt.tags.is_empty(), "Prompt should have at least one tag");
        assert!(!prompt.content.is_empty(), "Prompt content should not be empty");
        assert!(prompt.content.len() > 50, 
            "Prompt '{}' content should be substantial (>50 chars)", prompt.name);
    }
}

#[test]
fn test_initializing_enhanced_prompts_creates_all_files() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path();
    
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Check all prompts were created
    let created_files: Vec<_> = fs::read_dir(&prompts_dir)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
        })
        .filter(|name| name.ends_with(".md"))
        .collect();
    
    assert_eq!(
        created_files.len(), 
        DEFAULT_PROMPTS.len(), 
        "Should create exactly {} prompt files", 
        DEFAULT_PROMPTS.len()
    );
}