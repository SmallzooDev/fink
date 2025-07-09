use fink::presentation::tui::app::{AppMode, TUIApp};
use fink::presentation::tui::components::search::HighlightedText;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_search_bar_activation_with_slash() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create test prompt files
    let prompt1_path = temp_path.join("test-prompt-1.md");
    fs::write(&prompt1_path, r#"---
name: "test-prompt-1"
tags: ["test", "example"]
---
# Test Prompt 1
This is a test prompt."#).unwrap();

    let prompt2_path = temp_path.join("test-prompt-2.md");
    fs::write(&prompt2_path, r#"---
name: "test-prompt-2"
tags: ["demo"]
---
# Test Prompt 2
Another test prompt."#).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Initial state should be QuickSelect mode
    assert_eq!(app.mode(), &AppMode::QuickSelect);
    assert_eq!(app.is_search_active(), false);
    
    // Activate search mode
    app.activate_search();
    
    // Should be in search mode
    assert_eq!(app.is_search_active(), true);
}

#[test]
fn test_search_filtering_by_name() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompt files
    let files = vec![
        ("code-review.md", r#"---
name: "code-review"
tags: ["code", "review"]
---
# Code Review Assistant"#),
        ("bug-analysis.md", r#"---
name: "bug-analysis"
tags: ["bug", "debug"]
---
# Bug Analysis Tool"#),
        ("documentation.md", r#"---
name: "documentation"
tags: ["docs", "writing"]
---
# Documentation Helper"#),
    ];
    
    for (filename, content) in files {
        fs::write(jkms_path.join(filename), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Initially should have all 3 prompts
    assert_eq!(app.get_filtered_prompts().len(), 3);
    
    // Activate search and filter by "bug"
    app.activate_search();
    app.set_search_query("bug");
    
    // Should only show bug-analysis
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "bug-analysis");
    
    // Clear search should show all prompts again
    app.set_search_query("");
    assert_eq!(app.get_filtered_prompts().len(), 3);
}

#[test]
fn test_search_result_highlighting() {
    // Test that search terms are highlighted in the results
    let highlighter = HighlightedText::new();
    
    // Test case 1: Single match
    let text = "This is a test prompt";
    let query = "test";
    let highlighted = highlighter.highlight(text, query);
    
    assert_eq!(highlighted.segments.len(), 3);
    assert_eq!(highlighted.segments[0].text, "This is a ");
    assert_eq!(highlighted.segments[0].is_match, false);
    assert_eq!(highlighted.segments[1].text, "test");
    assert_eq!(highlighted.segments[1].is_match, true);
    assert_eq!(highlighted.segments[2].text, " prompt");
    assert_eq!(highlighted.segments[2].is_match, false);
    
    // Test case 2: Multiple matches
    let text = "test this test case";
    let query = "test";
    let highlighted = highlighter.highlight(text, query);
    
    assert_eq!(highlighted.segments.len(), 4);
    assert_eq!(highlighted.segments[0].text, "test");
    assert_eq!(highlighted.segments[0].is_match, true);
    assert_eq!(highlighted.segments[1].text, " this ");
    assert_eq!(highlighted.segments[1].is_match, false);
    assert_eq!(highlighted.segments[2].text, "test");
    assert_eq!(highlighted.segments[2].is_match, true);
    assert_eq!(highlighted.segments[3].text, " case");
    assert_eq!(highlighted.segments[3].is_match, false);
    
    // Test case 3: Case insensitive matching
    let text = "Test THIS test";
    let query = "test";
    let highlighted = highlighter.highlight(text, query);
    
    assert_eq!(highlighted.segments[0].text, "Test");
    assert_eq!(highlighted.segments[0].is_match, true);
    assert_eq!(highlighted.segments[2].text, "test");
    assert_eq!(highlighted.segments[2].is_match, true);
    
    // Test case 4: No matches
    let text = "No matches here";
    let query = "xyz";
    let highlighted = highlighter.highlight(text, query);
    
    assert_eq!(highlighted.segments.len(), 1);
    assert_eq!(highlighted.segments[0].text, "No matches here");
    assert_eq!(highlighted.segments[0].is_match, false);
}