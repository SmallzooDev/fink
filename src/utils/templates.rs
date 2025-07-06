use crate::utils::error::{Result, JkmsError, PromptError};
use crate::application::models::PromptType;

pub struct TemplateGenerator;

impl TemplateGenerator {
    /// Generates prompt content based on the template
    pub fn generate(name: &str, template: Option<&str>) -> Result<String> {
        Self::generate_with_type(name, template, PromptType::default())
    }
    
    /// Generates prompt content based on the template with specific type
    pub fn generate_with_type(name: &str, template: Option<&str>, prompt_type: PromptType) -> Result<String> {
        match template {
            Some("basic") => Ok(Self::generate_basic_template_with_type(name, prompt_type)),
            Some("clipboard") => Ok(Self::generate_clipboard_template_with_type(name, prompt_type)),
            Some(template_name) => {
                Err(JkmsError::Prompt(PromptError::InvalidFormat(
                    format!("Unknown template: {}", template_name)
                )))
            }
            None => Ok(Self::generate_default_template_with_type(name, prompt_type)),
        }
    }
    
    /// Generates prompt content with additional clipboard content
    pub fn generate_with_content(name: &str, template: Option<&str>, content: Option<&str>) -> Result<String> {
        Self::generate_with_content_and_type(name, template, content, PromptType::default())
    }
    
    /// Generates prompt content with additional clipboard content and specific type
    pub fn generate_with_content_and_type(name: &str, template: Option<&str>, content: Option<&str>, prompt_type: PromptType) -> Result<String> {
        match template {
            Some("clipboard") => {
                if let Some(clipboard_content) = content {
                    Ok(Self::generate_clipboard_template_with_content_and_type(name, clipboard_content, prompt_type))
                } else {
                    Ok(Self::generate_clipboard_template_with_type(name, prompt_type))
                }
            }
            _ => Self::generate_with_type(name, template, prompt_type)
        }
    }
    
    fn prompt_type_to_string(prompt_type: PromptType) -> &'static str {
        match prompt_type {
            PromptType::Instruction => "instruction",
            PromptType::Context => "context",
            PromptType::InputIndicator => "input_indicator",
            PromptType::OutputIndicator => "output_indicator",
            PromptType::Etc => "etc",
            PromptType::Whole => "whole",
        }
    }
    
    fn generate_basic_template_with_type(name: &str, prompt_type: PromptType) -> String {
        format!(
            r#"---
name: "{}"
tags: []
type: "{}"
---
# {}

# Instruction
(a specific task or instruction you want the model to perform)
Please input your prompt's instruction in here!

# Context
(external information or additional context that can steer the model to better responses)
Please input your prompt's context in here!

# Input Data
(the input or question that we are interested to find a response for)
Please input your prompt's input data in here!

# Output Indicator
(the type or format of the output)
Please input your prompt's output indicator here!
"#,
            name, Self::prompt_type_to_string(prompt_type), name
        )
    }
    
    fn generate_default_template_with_type(name: &str, prompt_type: PromptType) -> String {
        format!(
            r#"---
name: "{}"
tags: []
type: "{}"
---
# {}

"#,
            name, Self::prompt_type_to_string(prompt_type), name
        )
    }
    
    fn generate_clipboard_template_with_type(name: &str, prompt_type: PromptType) -> String {
        format!(
            r#"---
name: "{}"
tags: ["from-clipboard"]
type: "{}"
---
# {}

<!-- Content from clipboard will be inserted below -->

"#,
            name, Self::prompt_type_to_string(prompt_type), name
        )
    }
    
    fn generate_clipboard_template_with_content_and_type(name: &str, content: &str, prompt_type: PromptType) -> String {
        format!(
            r#"---
name: "{}"
tags: ["from-clipboard"]
type: "{}"
---
# {}

{}
"#,
            name, Self::prompt_type_to_string(prompt_type), name, content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_default_template() {
        let result = TemplateGenerator::generate("test-prompt", None).unwrap();
        assert!(result.contains(r#"name: "test-prompt""#));
        assert!(result.contains("tags: []"));
        assert!(result.contains(r#"type: "whole""#));
        assert!(result.contains("# test-prompt"));
    }
    
    #[test]
    fn test_generate_basic_template() {
        let result = TemplateGenerator::generate("test-prompt", Some("basic")).unwrap();
        assert!(result.contains(r#"type: "whole""#));
        assert!(result.contains("# Instruction"));
        assert!(result.contains("# Context"));
        assert!(result.contains("# Input Data"));
        assert!(result.contains("# Output Indicator"));
    }
    
    #[test]
    fn test_generate_with_custom_type() {
        let result = TemplateGenerator::generate_with_type("test-prompt", None, PromptType::Instruction).unwrap();
        assert!(result.contains(r#"type: "instruction""#));
        assert!(!result.contains(r#"type: "whole""#));
    }
    
    #[test]
    fn test_unknown_template() {
        let result = TemplateGenerator::generate("test", Some("unknown"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown template"));
    }
}