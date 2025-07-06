use crate::utils::error::{FinkError, StorageError};
use std::path::Path;
use std::fs;

pub struct DefaultPrompt {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
}

pub const DEFAULT_PROMPTS: &[DefaultPrompt] = &[
    DefaultPrompt {
        name: "code-review",
        description: "Code review assistant for various programming languages",
        tags: &["code", "review", "development"],
        content: r#"You are an experienced software developer conducting a code review. Please analyze the provided code for:

1. **Code Quality**: Readability, maintainability, and adherence to best practices
2. **Potential Bugs**: Logic errors, edge cases, or common pitfalls
3. **Performance**: Inefficiencies or optimization opportunities
4. **Security**: Potential vulnerabilities or security concerns
5. **Design Patterns**: Appropriate use of design patterns and architecture

Provide constructive feedback with specific examples and suggestions for improvement."#,
    },
    DefaultPrompt {
        name: "asking",
        description: "General question-asking assistant",
        tags: &["general", "questions", "assistant"],
        content: r#"You are a helpful AI assistant. Please answer the following question clearly and concisely:

- Provide accurate and relevant information
- Use examples when helpful
- Break down complex topics into understandable parts
- Acknowledge any limitations or uncertainties
- Suggest follow-up resources if appropriate"#,
    },
    DefaultPrompt {
        name: "backend-development",
        description: "Backend development assistant for APIs and server-side logic",
        tags: &["backend", "api", "development"],
        content: r#"You are a backend development expert specializing in API design and server-side architecture. Help with:

## Areas of Expertise:
- RESTful API design and best practices
- Database design and optimization
- Authentication and authorization
- Microservices architecture
- Performance optimization
- Security best practices
- Error handling and logging

## When providing solutions:
- Consider scalability and maintainability
- Follow SOLID principles
- Suggest appropriate design patterns
- Include error handling
- Consider security implications"#,
    },
    DefaultPrompt {
        name: "frontend-development",
        description: "Frontend development assistant for UI/UX and client-side logic",
        tags: &["frontend", "ui", "development"],
        content: r#"You are a frontend development expert specializing in modern web development. Help with:

## Areas of Expertise:
- Modern JavaScript/TypeScript
- React, Vue, or Angular frameworks
- CSS/SCSS and responsive design
- State management
- Performance optimization
- Accessibility (a11y)
- User experience best practices

## When providing solutions:
- Prioritize user experience
- Ensure accessibility compliance
- Consider cross-browser compatibility
- Optimize for performance
- Follow component-based architecture
- Include proper error handling"#,
    },
];

pub fn initialize_default_prompts(prompts_dir: &Path) -> Result<(), FinkError> {
    // Check if initialization flag exists
    let init_flag = prompts_dir.join(".initialized");
    if init_flag.exists() {
        return Ok(());
    }

    // Create prompts directory if it doesn't exist
    fs::create_dir_all(prompts_dir).map_err(|e| {
        FinkError::Storage(StorageError::Io(e))
    })?;

    // Create default prompts
    for prompt in DEFAULT_PROMPTS {
        let file_path = prompts_dir.join(format!("{}.md", prompt.name));
        
        // Skip if prompt already exists
        if file_path.exists() {
            continue;
        }

        let frontmatter = format!(
            r#"---
name: "{}"
description: "{}"
tags: [{}]
type: "whole"
created_at: "{}"
modified_at: "{}"
---

{}"#,
            prompt.name,
            prompt.description,
            prompt.tags.iter()
                .map(|tag| format!("\"{}\"", tag))
                .collect::<Vec<_>>()
                .join(", "),
            chrono::Utc::now().to_rfc3339(),
            chrono::Utc::now().to_rfc3339(),
            prompt.content
        );

        fs::write(&file_path, frontmatter).map_err(|e| {
            FinkError::Storage(StorageError::Io(e))
        })?;
    }

    // Create initialization flag
    fs::write(&init_flag, "").map_err(|e| {
        FinkError::Storage(StorageError::Io(e))
    })?;

    Ok(())
}