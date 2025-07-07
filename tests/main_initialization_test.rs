use std::fs;
use tempfile::TempDir;
use fink::utils::default_prompts::{initialize_default_prompts, DEFAULT_PROMPTS};

#[test]
fn test_main_should_initialize_prompts_in_fink_subdirectory() {
    // This test simulates what main.rs should do
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path(); // This simulates ~/.fink
    
    // CURRENT BUG: main.rs calls initialize_default_prompts(&base_path)
    // This would create prompts directly in ~/.fink/ instead of ~/.fink/fink/
    
    // First, let's verify the buggy behavior
    initialize_default_prompts(&base_path).unwrap();
    
    // With the bug, prompts would be created directly in base_path
    let wrong_location = base_path.join("code-review.md");
    assert!(wrong_location.exists(), "Bug confirmed: prompts are in wrong location");
    
    // Clean up for the correct test
    fs::remove_dir_all(&base_path).unwrap();
    fs::create_dir_all(&base_path).unwrap();
    
    // CORRECT BEHAVIOR: should initialize in base_path/prompts/
    let prompts_dir = base_path.join("prompts");
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Verify prompts are in the correct location
    for prompt in DEFAULT_PROMPTS {
        let correct_location = prompts_dir.join(format!("{}.md", prompt.name));
        assert!(
            correct_location.exists(), 
            "Prompt '{}' should be in fink/ subdirectory", 
            prompt.name
        );
        
        // And NOT in the base directory
        let wrong_location = base_path.join(format!("{}.md", prompt.name));
        assert!(
            !wrong_location.exists(), 
            "Prompt '{}' should NOT be directly in base path", 
            prompt.name
        );
    }
}