#!/bin/bash

# Script to generate test prompts for jkms
# Usage: ./generate_test_prompts.sh [path_to_jkms_directory]

JKMS_DIR="${1:-$HOME/.jkms/jkms}"

echo "Creating test prompts in: $JKMS_DIR"
mkdir -p "$JKMS_DIR"

# Instruction type prompts
cat > "$JKMS_DIR/code-reviewer.md" << 'EOF'
---
name: "code-reviewer"
tags: ["code", "review", "analysis"]
type: "instruction"
---
You are an experienced software engineer conducting a thorough code review. Focus on:
- Code quality and readability
- Performance implications
- Security vulnerabilities
- Best practices and design patterns
- Potential bugs or edge cases

Provide constructive feedback with specific examples and suggestions for improvement.
EOF

cat > "$JKMS_DIR/teacher-assistant.md" << 'EOF'
---
name: "teacher-assistant"
tags: ["education", "teaching", "explanation"]
type: "instruction"
---
You are a patient and knowledgeable teaching assistant. Your role is to:
- Break down complex concepts into simple, understandable parts
- Use analogies and real-world examples
- Encourage questions and critical thinking
- Adapt explanations based on the student's level
- Provide step-by-step guidance when needed
EOF

cat > "$JKMS_DIR/technical-writer.md" << 'EOF'
---
name: "technical-writer"
tags: ["documentation", "writing", "technical"]
type: "instruction"
---
You are a professional technical writer specializing in clear, concise documentation. Focus on:
- Writing for the target audience's technical level
- Using consistent terminology and style
- Including practical examples and use cases
- Organizing information logically
- Making complex topics accessible
EOF

# Context type prompts
cat > "$JKMS_DIR/rust-project-context.md" << 'EOF'
---
name: "rust-project-context"
tags: ["rust", "project", "development"]
type: "context"
---
Context: Working on a Rust CLI application that:
- Uses async/await with Tokio runtime
- Follows clean architecture principles
- Has comprehensive error handling with anyhow
- Includes both unit and integration tests
- Targets cross-platform compatibility (Windows, macOS, Linux)
EOF

cat > "$JKMS_DIR/startup-context.md" << 'EOF'
---
name: "startup-context"
tags: ["startup", "business", "agile"]
type: "context"
---
Context: Early-stage startup environment where:
- Resources are limited and efficiency is crucial
- Quick iteration and MVP approach is preferred
- Code should be maintainable but not over-engineered
- Team is small (3-5 developers)
- Focus on delivering user value quickly
EOF

cat > "$JKMS_DIR/legacy-system-context.md" << 'EOF'
---
name: "legacy-system-context"
tags: ["legacy", "refactoring", "maintenance"]
type: "context"
---
Context: Maintaining and modernizing a legacy system that:
- Has been in production for 5+ years
- Contains mix of old and new code patterns
- Must maintain backward compatibility
- Has limited test coverage
- Requires careful refactoring to avoid breaking changes
EOF

# Input indicator prompts
cat > "$JKMS_DIR/code-input-indicator.md" << 'EOF'
---
name: "code-input-indicator"
tags: ["input", "code", "format"]
type: "input_indicator"
---
### Input Code:
```
[Your code goes here]
```

### Specific concerns or questions:
[Optional: What specific aspects would you like reviewed or explained?]
EOF

cat > "$JKMS_DIR/user-story-input.md" << 'EOF'
---
name: "user-story-input"
tags: ["input", "requirements", "user-story"]
type: "input_indicator"
---
### User Story:
As a [type of user]
I want [functionality]
So that [benefit/value]

### Acceptance Criteria:
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

### Additional Context:
[Any additional information, constraints, or technical requirements]
EOF

cat > "$JKMS_DIR/error-input-indicator.md" << 'EOF'
---
name: "error-input-indicator"
tags: ["input", "error", "debugging"]
type: "input_indicator"
---
### Error Information:
**Error Message:**
```
[Paste the full error message here]
```

**Code Context:**
```
[Relevant code where the error occurs]
```

**Steps to Reproduce:**
1. [First step]
2. [Second step]
3. [Error occurs]

**Expected Behavior:**
[What should happen instead]
EOF

# Output indicator prompts
cat > "$JKMS_DIR/structured-analysis-output.md" << 'EOF'
---
name: "structured-analysis-output"
tags: ["output", "analysis", "structured"]
type: "output_indicator"
---
Please provide your response in the following format:

## Summary
[Brief overview of the main points]

## Detailed Analysis
### Strengths
- [Positive aspect 1]
- [Positive aspect 2]

### Areas for Improvement
- [Issue 1]: [Explanation and suggestion]
- [Issue 2]: [Explanation and suggestion]

## Recommendations
1. [Specific actionable recommendation]
2. [Another recommendation]

## Code Examples (if applicable)
```language
// Example implementation
```
EOF

cat > "$JKMS_DIR/json-output-format.md" << 'EOF'
---
name: "json-output-format"
tags: ["output", "json", "api"]
type: "output_indicator"
---
Provide the response in the following JSON format:

```json
{
  "status": "success|error",
  "summary": "Brief summary of the analysis",
  "details": {
    "category": "Type of response",
    "confidence": 0.95,
    "key_points": [
      "Point 1",
      "Point 2"
    ]
  },
  "recommendations": [
    {
      "priority": "high|medium|low",
      "action": "Specific action to take",
      "rationale": "Why this is recommended"
    }
  ],
  "metadata": {
    "timestamp": "ISO 8601 timestamp",
    "version": "1.0"
  }
}
```
EOF

cat > "$JKMS_DIR/markdown-report-output.md" << 'EOF'
---
name: "markdown-report-output"
tags: ["output", "markdown", "report"]
type: "output_indicator"
---
Format your response as a Markdown report:

# [Report Title]

## Executive Summary
[2-3 sentence overview]

## Table of Contents
1. [Section 1]
2. [Section 2]
3. [Section 3]

## Detailed Findings

### [Section 1 Title]
[Content with proper markdown formatting]

### [Section 2 Title]
[Content with tables, lists, or code blocks as needed]

## Conclusion
[Summary and next steps]

## Appendix (Optional)
[Additional technical details or references]
EOF

# Etc type prompts (utilities and helpers)
cat > "$JKMS_DIR/thinking-process.md" << 'EOF'
---
name: "thinking-process"
tags: ["utility", "thinking", "reasoning"]
type: "etc"
---
Before providing the final answer, please think through the problem step by step:

1. Understanding: What is being asked?
2. Analysis: What are the key components or challenges?
3. Approach: What strategies or methods apply here?
4. Implementation: How to execute the solution?
5. Validation: How to verify the solution is correct?

Show your reasoning process before giving the final answer.
EOF

cat > "$JKMS_DIR/constraints-reminder.md" << 'EOF'
---
name: "constraints-reminder"
tags: ["utility", "constraints", "requirements"]
type: "etc"
---
Please ensure your response adheres to these constraints:
- Keep explanations concise but complete
- Use industry-standard terminology
- Prefer practical solutions over theoretical ones
- Consider performance implications
- Maintain security best practices
- Follow the principle of least surprise
EOF

cat > "$JKMS_DIR/examples-request.md" << 'EOF'
---
name: "examples-request"
tags: ["utility", "examples", "clarification"]
type: "etc"
---
Please include in your response:
- At least 2-3 concrete examples
- Both positive and negative cases (what to do and what to avoid)
- Edge cases or special scenarios to consider
- Real-world applications or use cases
- References or links for further reading (if applicable)
EOF

echo "✓ Created 15 test prompts:"
echo "  - 3 instruction prompts"
echo "  - 3 context prompts"
echo "  - 3 input indicator prompts"
echo "  - 3 output indicator prompts"
echo "  - 3 utility (etc) prompts"
echo ""
echo "You can now test the build mode with these prompts!"