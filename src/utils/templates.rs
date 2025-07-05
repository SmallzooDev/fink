use crate::utils::error::{Result, JkmsError, PromptError};

pub struct TemplateGenerator;

impl TemplateGenerator {
    /// Generates prompt content based on the template
    pub fn generate(name: &str, template: Option<&str>) -> Result<String> {
        match template {
            Some("basic") => Ok(Self::generate_basic_template(name)),
            Some("clipboard") => Ok(Self::generate_clipboard_template(name)),
            Some(template_name) => {
                Err(JkmsError::Prompt(PromptError::InvalidFormat(
                    format!("Unknown template: {}", template_name)
                )))
            }
            None => Ok(Self::generate_default_template(name)),
        }
    }
    
    /// Generates prompt content with additional clipboard content
    pub fn generate_with_content(name: &str, template: Option<&str>, content: Option<&str>) -> Result<String> {
        match template {
            Some("clipboard") => {
                if let Some(clipboard_content) = content {
                    Ok(Self::generate_clipboard_template_with_content(name, clipboard_content))
                } else {
                    Ok(Self::generate_clipboard_template(name))
                }
            }
            _ => Self::generate(name, template)
        }
    }
    
    fn generate_basic_template(name: &str) -> String {
        format!(
            r#"---
name: "{}"
tags: []
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
            name, name
        )
    }
    
    fn generate_default_template(name: &str) -> String {
        format!(
            r#"---
name: "{}"
tags: []
---
# {}

"#,
            name, name
        )
    }
    
    fn generate_clipboard_template(name: &str) -> String {
        format!(
            r#"---
name: "{}"
tags: ["from-clipboard"]
---
# {}

<!-- Content from clipboard will be inserted below -->

"#,
            name, name
        )
    }
    
    fn generate_clipboard_template_with_content(name: &str, content: &str) -> String {
        format!(
            r#"---
name: "{}"
tags: ["from-clipboard"]
---
# {}

{}
"#,
            name, name, content
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
        assert!(result.contains("# test-prompt"));
    }
    
    #[test]
    fn test_generate_basic_template() {
        let result = TemplateGenerator::generate("test-prompt", Some("basic")).unwrap();
        assert!(result.contains("# Instruction"));
        assert!(result.contains("# Context"));
        assert!(result.contains("# Input Data"));
        assert!(result.contains("# Output Indicator"));
    }
    
    #[test]
    fn test_unknown_template() {
        let result = TemplateGenerator::generate("test", Some("unknown"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown template"));
    }
}