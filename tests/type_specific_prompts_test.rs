use fink::utils::default_prompts::{get_type_specific_prompts, TypeSpecificPrompt};
use fink::application::models::PromptType;
use std::collections::HashMap;

#[test]
fn test_type_specific_prompts_exist_for_all_types() {
    let type_prompts = get_type_specific_prompts();
    
    // Check that we have prompts for each type
    assert!(type_prompts.contains_key(&PromptType::Instruction));
    assert!(type_prompts.contains_key(&PromptType::Context));
    assert!(type_prompts.contains_key(&PromptType::InputIndicator));
    assert!(type_prompts.contains_key(&PromptType::OutputIndicator));
    assert!(type_prompts.contains_key(&PromptType::Etc));
    
    // Each type should have at least one prompt
    for (prompt_type, prompts) in type_prompts.iter() {
        assert!(!prompts.is_empty(), "Type {:?} should have at least one prompt", prompt_type);
    }
}

#[test]
fn test_instruction_type_prompts() {
    let type_prompts = get_type_specific_prompts();
    let instruction_prompts = type_prompts.get(&PromptType::Instruction).unwrap();
    
    // Should have specific instruction prompts
    assert!(instruction_prompts.iter().any(|p| p.name.contains("step")));
    assert!(instruction_prompts.iter().any(|p| p.name.contains("guide")));
}

#[test]
fn test_context_type_prompts() {
    let type_prompts = get_type_specific_prompts();
    let context_prompts = type_prompts.get(&PromptType::Context).unwrap();
    
    // Should have context-providing prompts
    assert!(context_prompts.iter().any(|p| p.name.contains("context")));
    assert!(context_prompts.iter().any(|p| p.tags.contains(&"background")));
}

#[test]
fn test_input_indicator_prompts() {
    let type_prompts = get_type_specific_prompts();
    let input_prompts = type_prompts.get(&PromptType::InputIndicator).unwrap();
    
    // Should have prompts for marking inputs
    assert!(input_prompts.iter().any(|p| p.name.contains("input")));
}

#[test]
fn test_output_indicator_prompts() {
    let type_prompts = get_type_specific_prompts();
    let output_prompts = type_prompts.get(&PromptType::OutputIndicator).unwrap();
    
    // Should have prompts for marking outputs
    assert!(output_prompts.iter().any(|p| p.name.contains("output")));
}

#[test]
fn test_all_type_specific_prompts_have_correct_type() {
    let type_prompts = get_type_specific_prompts();
    
    for (expected_type, prompts) in type_prompts.iter() {
        for prompt in prompts {
            assert_eq!(prompt.prompt_type, *expected_type, 
                "Prompt '{}' should have type {:?}", prompt.name, expected_type);
        }
    }
}