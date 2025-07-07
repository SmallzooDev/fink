use fink::utils::default_prompts::{initialize_default_prompts, DEFAULT_PROMPTS};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_default_prompts_initialized_in_correct_subdirectory() {
    // Create a temporary directory to simulate ~/.fink
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // The prompts should be initialized in the 'fink' subdirectory
    let prompts_dir = base_path.join("prompts");
    
    // Initialize default prompts - this should create them in base_path/fink/
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Verify the fink subdirectory was created
    assert!(prompts_dir.exists(), "The 'fink' subdirectory should exist");
    
    // Verify all default prompts were created in the correct location
    for prompt in DEFAULT_PROMPTS {
        let prompt_file = prompts_dir.join(format!("{}.md", prompt.name));
        assert!(
            prompt_file.exists(), 
            "Prompt file '{}' should exist in the fink subdirectory", 
            prompt.name
        );
        
        // Verify the content contains the expected frontmatter
        let content = fs::read_to_string(&prompt_file).unwrap();
        assert!(content.contains(&format!("name: \"{}\"", prompt.name)));
        assert!(content.contains(&format!("description: \"{}\"", prompt.description)));
    }
    
    // Verify the initialization flag was created
    let init_flag = prompts_dir.join(".initialized");
    assert!(init_flag.exists(), "Initialization flag should exist");
}

#[test]
fn test_default_prompts_not_overwritten_on_subsequent_runs() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    
    // First initialization
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Modify one of the prompts
    let test_prompt_path = prompts_dir.join("code-review.md");
    let original_content = fs::read_to_string(&test_prompt_path).unwrap();
    let modified_content = format!("{}\n\n# User modifications", original_content);
    fs::write(&test_prompt_path, &modified_content).unwrap();
    
    // Second initialization should not overwrite
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Verify the modification is still there
    let current_content = fs::read_to_string(&test_prompt_path).unwrap();
    assert!(current_content.contains("# User modifications"));
}