use crate::utils::error::{FinkError, StorageError};
use crate::application::models::PromptType;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

pub struct DefaultPrompt {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
}

pub struct TypeSpecificPrompt {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
    pub prompt_type: PromptType,
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
    DefaultPrompt {
        name: "explain-code",
        description: "Explains complex code in simple terms",
        tags: &["explain", "learning", "education"],
        content: r#"You are an expert at explaining code clearly and concisely. When analyzing code:

1. **Overview**: Start with a high-level explanation of what the code does
2. **Key Concepts**: Identify and explain any important concepts or patterns
3. **Step-by-Step**: Walk through the code logic in order
4. **Edge Cases**: Point out any special cases or error handling
5. **Examples**: Provide simple examples of how to use the code

Use simple language and avoid jargon where possible. If technical terms are necessary, explain them clearly."#,
    },
    DefaultPrompt {
        name: "debug-assistant",
        description: "Helps identify and fix bugs in code",
        tags: &["debug", "fix", "troubleshooting"],
        content: r#"You are a debugging expert. When presented with a bug or error:

## Approach:
1. **Analyze the Error**: Understand the error message and stack trace
2. **Identify Root Cause**: Look for the underlying issue, not just symptoms
3. **Suggest Solutions**: Provide multiple approaches to fix the problem
4. **Prevent Recurrence**: Suggest how to avoid similar issues in the future

## Consider:
- Common pitfalls in the language/framework
- Edge cases and boundary conditions
- Race conditions or timing issues
- Memory leaks or resource management
- Type mismatches or null/undefined errors"#,
    },
    DefaultPrompt {
        name: "test-writer",
        description: "Generates comprehensive test cases for code",
        tags: &["test", "testing", "quality"],
        content: r#"You are a testing expert specializing in writing comprehensive test suites. When writing tests:

## Test Categories:
1. **Unit Tests**: Test individual functions/methods in isolation
2. **Integration Tests**: Test how components work together
3. **Edge Cases**: Test boundary conditions and unusual inputs
4. **Error Cases**: Test error handling and failure scenarios

## Best Practices:
- Use descriptive test names that explain what is being tested
- Follow the Arrange-Act-Assert pattern
- Keep tests independent and idempotent
- Mock external dependencies appropriately
- Aim for high code coverage but focus on meaningful tests"#,
    },
    DefaultPrompt {
        name: "documentation-writer",
        description: "Creates clear and comprehensive documentation",
        tags: &["docs", "documentation", "writing"],
        content: r#"You are a technical documentation expert. When writing documentation:

## Documentation Types:
1. **API Documentation**: Clear descriptions of functions, parameters, and return values
2. **README Files**: Project overview, setup instructions, and usage examples
3. **Code Comments**: Inline explanations for complex logic
4. **Architecture Docs**: High-level system design and decisions

## Guidelines:
- Write for your audience (developers, users, or both)
- Include practical examples
- Keep it concise but complete
- Update documentation as code changes
- Use consistent formatting and structure"#,
    },
    DefaultPrompt {
        name: "refactor-assistant",
        description: "Suggests code improvements and refactoring strategies",
        tags: &["refactor", "improve", "clean-code"],
        content: r#"You are a code refactoring expert focused on improving code quality. When refactoring:

## Refactoring Goals:
1. **Readability**: Make code easier to understand
2. **Maintainability**: Make code easier to modify
3. **Performance**: Optimize without sacrificing clarity
4. **Reusability**: Extract common functionality
5. **Testability**: Make code easier to test

## Common Refactoring Patterns:
- Extract method/function
- Rename for clarity
- Remove duplication
- Simplify conditionals
- Extract constants
- Apply design patterns appropriately"#,
    },
    DefaultPrompt {
        name: "sql-assistant",
        description: "SQL query writing and optimization expert",
        tags: &["sql", "database", "query"],
        content: r#"You are a SQL and database expert. Help with:

## Areas of Expertise:
1. **Query Writing**: SELECT, INSERT, UPDATE, DELETE operations
2. **Joins**: INNER, LEFT, RIGHT, FULL OUTER joins
3. **Aggregations**: GROUP BY, HAVING, window functions
4. **Performance**: Query optimization, indexing strategies
5. **Schema Design**: Normalization, relationships, constraints

## Best Practices:
- Write readable queries with proper formatting
- Use meaningful table and column aliases
- Avoid N+1 query problems
- Consider index usage
- Handle NULL values appropriately
- Use transactions for data consistency"#,
    },
    DefaultPrompt {
        name: "security-reviewer",
        description: "Security analysis and vulnerability detection",
        tags: &["security", "vulnerability", "audit"],
        content: r#"You are a security expert specializing in identifying vulnerabilities. Analyze code for:

## Security Concerns:
1. **Input Validation**: SQL injection, XSS, command injection
2. **Authentication**: Weak passwords, session management
3. **Authorization**: Access control, privilege escalation
4. **Data Protection**: Encryption, sensitive data exposure
5. **Dependencies**: Vulnerable packages, outdated libraries

## Recommendations:
- Follow OWASP guidelines
- Use parameterized queries
- Implement proper input sanitization
- Use secure communication (HTTPS)
- Keep dependencies updated
- Follow the principle of least privilege"#,
    },
    DefaultPrompt {
        name: "performance-optimizer",
        description: "Performance analysis and optimization expert",
        tags: &["performance", "optimization", "speed"],
        content: r#"You are a performance optimization expert. When analyzing performance:

## Analysis Areas:
1. **Time Complexity**: Algorithm efficiency
2. **Space Complexity**: Memory usage
3. **Database Queries**: N+1 problems, missing indexes
4. **Caching**: Opportunities for caching
5. **Async Operations**: Parallelization opportunities

## Optimization Strategies:
- Profile before optimizing
- Focus on bottlenecks
- Consider trade-offs (memory vs speed)
- Use appropriate data structures
- Minimize network calls
- Implement lazy loading where appropriate"#,
    },
    DefaultPrompt {
        name: "api-designer",
        description: "REST API design and best practices expert",
        tags: &["api", "rest", "design"],
        content: r#"You are an API design expert specializing in RESTful services. When designing APIs:

## Design Principles:
1. **RESTful Conventions**: Proper use of HTTP methods and status codes
2. **Resource Naming**: Clear, consistent URL patterns
3. **Versioning**: API version management strategies
4. **Documentation**: Clear request/response examples
5. **Error Handling**: Consistent error response format

## Best Practices:
- Use nouns for resources, not verbs
- Implement proper pagination
- Support filtering and sorting
- Use appropriate HTTP status codes
- Include rate limiting
- Provide comprehensive API documentation
- Follow HATEOAS principles where appropriate"#,
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

pub fn get_type_specific_prompts() -> HashMap<PromptType, Vec<TypeSpecificPrompt>> {
    let mut prompts = HashMap::new();
    
    // Instruction type prompts
    prompts.insert(PromptType::Instruction, vec![
        TypeSpecificPrompt {
            name: "step-by-step-guide",
            description: "Creates detailed step-by-step instructions",
            tags: &["instruction", "guide", "tutorial"],
            content: r#"Please provide a detailed step-by-step guide for the following task:

## Format:
1. **Step Name**: Brief description
   - Detailed explanation
   - Any prerequisites or warnings
   - Expected outcome

2. **Next Step**: Continue pattern...

## Guidelines:
- Number each major step
- Use sub-bullets for details
- Include any necessary warnings or prerequisites
- Mention expected outcomes
- Add troubleshooting tips where relevant"#,
            prompt_type: PromptType::Instruction,
        },
        TypeSpecificPrompt {
            name: "how-to-guide",
            description: "Creates how-to guides for specific tasks",
            tags: &["instruction", "how-to", "guide"],
            content: r#"Create a comprehensive how-to guide that:

1. States the objective clearly
2. Lists required materials/prerequisites
3. Provides step-by-step instructions
4. Includes helpful tips and warnings
5. Offers troubleshooting advice
6. Suggests next steps or related guides

Keep instructions clear, concise, and actionable."#,
            prompt_type: PromptType::Instruction,
        },
        TypeSpecificPrompt {
            name: "command-instruction",
            description: "Provides command-line instructions",
            tags: &["instruction", "cli", "command"],
            content: r#"Provide clear command-line instructions:

## Format:
```bash
# Description of what this command does
$ command --with --options

# Expected output or result
```

## Include:
- Purpose of each command
- Required parameters
- Optional flags with explanations
- Expected output
- Common errors and solutions
- Alternative commands if applicable"#,
            prompt_type: PromptType::Instruction,
        },
    ]);
    
    // Context type prompts
    prompts.insert(PromptType::Context, vec![
        TypeSpecificPrompt {
            name: "project-context",
            description: "Provides project background and context",
            tags: &["context", "background", "project"],
            content: r#"## Project Context

Please provide comprehensive context about this project:

### Overview
- Project purpose and goals
- Target audience/users
- Key features and functionality

### Technical Stack
- Languages and frameworks
- Key dependencies
- Architecture overview

### Current Status
- Development phase
- Known issues or limitations
- Future plans

### Relevant Information
- Related documentation
- Team structure
- Development workflow"#,
            prompt_type: PromptType::Context,
        },
        TypeSpecificPrompt {
            name: "code-context",
            description: "Provides context for understanding code",
            tags: &["context", "code", "explanation"],
            content: r#"## Code Context

Provide context to understand this code:

1. **Purpose**: What problem does this code solve?
2. **Architecture**: How does it fit into the larger system?
3. **Dependencies**: What external libraries or services does it use?
4. **Data Flow**: How does data move through this code?
5. **Key Concepts**: What domain knowledge is needed?
6. **History**: Why was it written this way?
7. **Constraints**: What limitations or requirements influenced the design?"#,
            prompt_type: PromptType::Context,
        },
        TypeSpecificPrompt {
            name: "business-context",
            description: "Provides business context and requirements",
            tags: &["context", "business", "requirements"],
            content: r#"## Business Context

Provide the business context for this initiative:

### Business Goals
- Primary objectives
- Success metrics
- Timeline and milestones

### Stakeholders
- Key stakeholders and their interests
- User personas
- Decision makers

### Constraints
- Budget limitations
- Regulatory requirements
- Technical constraints
- Time constraints

### Impact
- Expected benefits
- Potential risks
- Dependencies on other initiatives"#,
            prompt_type: PromptType::Context,
        },
    ]);
    
    // Input Indicator prompts
    prompts.insert(PromptType::InputIndicator, vec![
        TypeSpecificPrompt {
            name: "input-format",
            description: "Specifies expected input format",
            tags: &["input", "format", "specification"],
            content: r#"## Input Specification

Please provide your input in the following format:

### Expected Format:
```
[Input Type]: [Description]
[Field 1]: [Value]
[Field 2]: [Value]
...
```

### Example:
```
Type: User Registration
Username: john_doe
Email: john@example.com
Password: [hidden]
```

### Requirements:
- All fields marked with * are required
- Follow the specified format exactly
- Use appropriate data types
- Include units where applicable"#,
            prompt_type: PromptType::InputIndicator,
        },
        TypeSpecificPrompt {
            name: "data-input",
            description: "Structured data input template",
            tags: &["input", "data", "template"],
            content: r#"## Data Input Template

### Input Format:
```json
{
  "field_name": "description of expected value",
  "data_type": "string|number|boolean|array|object",
  "required": true|false,
  "example": "example value"
}
```

### Validation Rules:
- Required fields must be provided
- Data types must match specification
- Values must pass validation constraints
- Arrays and objects must follow nested structure

### Please provide your data below:"#,
            prompt_type: PromptType::InputIndicator,
        },
    ]);
    
    // Output Indicator prompts
    prompts.insert(PromptType::OutputIndicator, vec![
        TypeSpecificPrompt {
            name: "output-format",
            description: "Specifies expected output format",
            tags: &["output", "format", "result"],
            content: r#"## Output Format Specification

The output will be provided in the following format:

### Structure:
```
[Output Type]: [Description]
[Field 1]: [Value]
[Field 2]: [Value]
Status: [Success|Warning|Error]
Message: [Detailed message]
```

### Output Sections:
1. **Summary**: Brief overview of results
2. **Details**: Comprehensive information
3. **Metrics**: Relevant measurements or statistics
4. **Next Steps**: Recommended actions

### Format Options:
- JSON for structured data
- Markdown for documentation
- Plain text for simple outputs
- CSV for tabular data"#,
            prompt_type: PromptType::OutputIndicator,
        },
        TypeSpecificPrompt {
            name: "result-output",
            description: "Structured result output template",
            tags: &["output", "result", "response"],
            content: r#"## Result Output Template

### Output Structure:
```
=== RESULTS ===
Status: [Completed|Partial|Failed]
Duration: [time taken]
Timestamp: [ISO 8601 timestamp]

=== SUMMARY ===
[Brief summary of results]

=== DETAILS ===
[Detailed results with formatting]

=== ERRORS/WARNINGS ===
[Any issues encountered]

=== RECOMMENDATIONS ===
[Suggested next steps]
```

All output will follow this consistent structure for easy parsing and understanding."#,
            prompt_type: PromptType::OutputIndicator,
        },
    ]);
    
    // Etc type prompts
    prompts.insert(PromptType::Etc, vec![
        TypeSpecificPrompt {
            name: "brainstorm-ideas",
            description: "Brainstorming and ideation helper",
            tags: &["brainstorm", "ideas", "creative"],
            content: r#"Let's brainstorm ideas together! I'll help you:

## Brainstorming Approach:
1. **Divergent Thinking**: Generate many ideas without judgment
2. **Categories**: Organize ideas by theme or type
3. **Combinations**: Mix and match concepts
4. **Variations**: Explore different angles
5. **Evaluation**: Assess feasibility and impact

## Techniques:
- Mind mapping
- SCAMPER method
- Reverse brainstorming
- Random word association
- What-if scenarios

Share your topic or challenge, and let's explore creative solutions!"#,
            prompt_type: PromptType::Etc,
        },
        TypeSpecificPrompt {
            name: "checklist-creator",
            description: "Creates comprehensive checklists",
            tags: &["checklist", "tasks", "organization"],
            content: r#"## Checklist Generator

I'll create a comprehensive checklist for your needs:

### Checklist Format:
- [ ] Main task or category
  - [ ] Subtask with details
  - [ ] Required prerequisites
  - [ ] Quality checks

### Categories to Consider:
1. **Preparation**: What needs to be ready?
2. **Execution**: Step-by-step tasks
3. **Verification**: How to confirm completion
4. **Follow-up**: Post-completion tasks

### Features:
- Priority levels (High/Medium/Low)
- Time estimates
- Dependencies
- Responsible parties
- Due dates

What checklist would you like me to create?"#,
            prompt_type: PromptType::Etc,
        },
    ]);
    
    prompts
}

pub fn initialize_type_specific_prompts(prompts_dir: &Path) -> Result<(), FinkError> {
    let type_prompts = get_type_specific_prompts();
    
    for (_prompt_type, prompts) in type_prompts.iter() {
        for prompt in prompts {
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
type: "{}"
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
                match prompt.prompt_type {
                    PromptType::Instruction => "instruction",
                    PromptType::Context => "context",
                    PromptType::InputIndicator => "input_indicator",
                    PromptType::OutputIndicator => "output_indicator",
                    PromptType::Etc => "etc",
                    PromptType::Whole => "whole",
                },
                chrono::Utc::now().to_rfc3339(),
                chrono::Utc::now().to_rfc3339(),
                prompt.content
            );
            
            fs::write(&file_path, frontmatter).map_err(|e| {
                FinkError::Storage(StorageError::Io(e))
            })?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_get_type_specific_prompts() {
        let prompts = get_type_specific_prompts();
        
        // Verify we have prompts for each type
        assert!(prompts.contains_key(&PromptType::Instruction));
        assert!(prompts.contains_key(&PromptType::Context));
        assert!(prompts.contains_key(&PromptType::InputIndicator));
        assert!(prompts.contains_key(&PromptType::OutputIndicator));
        assert!(prompts.contains_key(&PromptType::Etc));
        
        // Verify counts for each type
        assert_eq!(prompts.get(&PromptType::Instruction).unwrap().len(), 3);
        assert_eq!(prompts.get(&PromptType::Context).unwrap().len(), 3);
        assert_eq!(prompts.get(&PromptType::InputIndicator).unwrap().len(), 2);
        assert_eq!(prompts.get(&PromptType::OutputIndicator).unwrap().len(), 2);
        assert_eq!(prompts.get(&PromptType::Etc).unwrap().len(), 2);
        
        // Total should be 12 prompts
        let total_count: usize = prompts.values().map(|v| v.len()).sum();
        assert_eq!(total_count, 12);
    }

    #[test]
    fn test_initialize_type_specific_prompts() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let prompts_dir = temp_dir.path();
        
        // Initialize type-specific prompts
        let result = initialize_type_specific_prompts(prompts_dir);
        assert!(result.is_ok());
        
        // Verify files were created
        let type_prompts = get_type_specific_prompts();
        let mut created_count = 0;
        
        for (_prompt_type, prompts) in type_prompts.iter() {
            for prompt in prompts {
                let file_path = prompts_dir.join(format!("{}.md", prompt.name));
                assert!(file_path.exists(), "File should exist: {:?}", file_path);
                
                // Verify content includes the prompt content
                let content = fs::read_to_string(&file_path).unwrap();
                assert!(content.contains(prompt.content));
                assert!(content.contains(&format!("name: \"{}\"", prompt.name)));
                assert!(content.contains(&format!("description: \"{}\"", prompt.description)));
                
                created_count += 1;
            }
        }
        
        assert_eq!(created_count, 12, "Should create 12 type-specific prompts");
    }

    #[test]
    fn test_initialize_type_specific_prompts_skip_existing() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let prompts_dir = temp_dir.path();
        
        // Pre-create one file (using one of the actual type-specific prompts)
        let existing_file = prompts_dir.join("step-by-step-guide.md");
        fs::create_dir_all(prompts_dir).unwrap();
        fs::write(&existing_file, "existing content").unwrap();
        
        // Initialize type-specific prompts
        let result = initialize_type_specific_prompts(prompts_dir);
        assert!(result.is_ok());
        
        // Verify the existing file was not overwritten
        let content = fs::read_to_string(&existing_file).unwrap();
        assert_eq!(content, "existing content");
        
        // Verify other files were created
        let file_count = fs::read_dir(prompts_dir).unwrap().count();
        assert_eq!(file_count, 12); // 11 new + 1 existing
    }
}