use fink::presentation::tui::tui::TUIApp;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_tag_filtering_activation() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompt files with tags
    let files = vec![
        ("code-review.md", r#"---
name: "code-review"
tags: ["code", "review", "python"]
---
# Code Review Assistant"#),
        ("bug-analysis.md", r#"---
name: "bug-analysis"
tags: ["bug", "debug", "code"]
---
# Bug Analysis Tool"#),
        ("documentation.md", r#"---
name: "documentation"
tags: ["docs", "writing", "markdown"]
---
# Documentation Helper"#),
    ];
    
    for (filename, content) in files {
        fs::write(jkms_path.join(filename), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Initial state should not have tag filtering active
    assert_eq!(app.is_tag_filter_active(), false);
    assert_eq!(app.get_active_tag_filter(), None);
    
    // Activate tag filtering mode
    app.activate_tag_filter();
    
    // Should be in tag filter mode
    assert_eq!(app.is_tag_filter_active(), true);
}

#[test]
fn test_tag_filtering_by_single_tag() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompt files with tags
    let files = vec![
        ("code-review.md", r#"---
name: "code-review"
tags: ["code", "review", "python"]
---
# Code Review Assistant"#),
        ("bug-analysis.md", r#"---
name: "bug-analysis"
tags: ["bug", "debug", "code"]
---
# Bug Analysis Tool"#),
        ("documentation.md", r#"---
name: "documentation"
tags: ["docs", "writing", "markdown"]
---
# Documentation Helper"#),
        ("python-helper.md", r#"---
name: "python-helper"
tags: ["python", "programming"]
---
# Python Helper"#),
    ];
    
    for (filename, content) in files {
        fs::write(jkms_path.join(filename), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Initially should have all 4 prompts
    assert_eq!(app.get_filtered_prompts().len(), 4);
    
    // Filter by "code" tag
    app.set_tag_filter("code");
    
    // Should only show prompts with "code" tag
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 2);
    
    let names: Vec<String> = filtered.iter().map(|p| p.name.clone()).collect();
    assert!(names.contains(&"code-review".to_string()));
    assert!(names.contains(&"bug-analysis".to_string()));
    
    // Filter by "python" tag
    app.set_tag_filter("python");
    
    // Should only show prompts with "python" tag
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 2);
    
    let names: Vec<String> = filtered.iter().map(|p| p.name.clone()).collect();
    assert!(names.contains(&"code-review".to_string()));
    assert!(names.contains(&"python-helper".to_string()));
    
    // Clear tag filter should show all prompts again
    app.clear_tag_filter();
    assert_eq!(app.get_filtered_prompts().len(), 4);
}

#[test]
fn test_tag_list_extraction() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompt files with tags
    let files = vec![
        ("code-review.md", r#"---
name: "code-review"
tags: ["code", "review", "python"]
---
# Code Review Assistant"#),
        ("bug-analysis.md", r#"---
name: "bug-analysis"
tags: ["bug", "debug", "code"]
---
# Bug Analysis Tool"#),
        ("documentation.md", r#"---
name: "documentation"
tags: ["docs", "writing", "markdown"]
---
# Documentation Helper"#),
    ];
    
    for (filename, content) in files {
        fs::write(jkms_path.join(filename), content).unwrap();
    }
    
    // Create TUIApp
    let app = TUIApp::new(temp_path).unwrap();
    
    // Get all unique tags
    let all_tags = app.get_all_tags();
    
    // Should have these unique tags
    let expected_tags = vec!["bug", "code", "debug", "docs", "markdown", "python", "review", "writing"];
    assert_eq!(all_tags.len(), expected_tags.len());
    
    for tag in expected_tags {
        assert!(all_tags.contains(&tag.to_string()), "Missing tag: {}", tag);
    }
}

#[test]
fn test_tag_filter_combined_with_search() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create test prompt files with tags
    let files = vec![
        ("code-review.md", r#"---
name: "code-review"
tags: ["code", "review"]
---
# Code Review Assistant"#),
        ("code-generator.md", r#"---
name: "code-generator"
tags: ["code", "generator"]
---
# Code Generator"#),
        ("bug-analysis.md", r#"---
name: "bug-analysis"
tags: ["bug", "debug"]
---
# Bug Analysis Tool"#),
        ("review-helper.md", r#"---
name: "review-helper"
tags: ["review", "helper"]
---
# Review Helper"#),
    ];
    
    for (filename, content) in files {
        fs::write(jkms_path.join(filename), content).unwrap();
    }
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Initially should have all 4 prompts
    assert_eq!(app.get_filtered_prompts().len(), 4);
    
    // Filter by "code" tag
    app.set_tag_filter("code");
    assert_eq!(app.get_filtered_prompts().len(), 2);
    
    // Now also search for "review"
    app.activate_search();
    app.set_search_query("review");
    
    // Should only show prompts with "code" tag AND "review" in name
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "code-review");
    
    // Clear search but keep tag filter
    app.set_search_query("");
    assert_eq!(app.get_filtered_prompts().len(), 2);
    
    // Clear tag filter but set search
    app.clear_tag_filter();
    app.set_search_query("review");
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 2);
    let names: Vec<String> = filtered.iter().map(|p| p.name.clone()).collect();
    assert!(names.contains(&"code-review".to_string()));
    assert!(names.contains(&"review-helper".to_string()));
}