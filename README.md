# fink

A beautiful TUI (Terminal User Interface) for managing AI prompts with version control support.

## Overview

fink is a command-line tool designed to help you efficiently manage, organize, and access your AI prompts. It provides both a quick selection mode for rapid prompt copying and a comprehensive management interface for organizing your prompt library.

## Features

- 🚀 **Quick Selection Mode**: Launch fink without arguments for instant prompt access
- 📝 **Prompt Management**: Create, edit, delete, and organize prompts with ease
- 🏷️ **Tag System**: Categorize prompts with tags for better organization
- 🔍 **Search Functionality**: Find prompts by name, content, or tags
- 📋 **Clipboard Integration**: Automatically copy selected prompts to clipboard
- 🎨 **Beautiful TUI**: Intuitive terminal interface with keyboard navigation
- 🏗️ **Template System**: Create prompts from predefined templates
- ✏️ **External Editor Support**: Edit prompts in your favorite editor
- 🔧 **Build Mode**: Combine multiple prompts into complex workflows
- ⚙️ **Configurable**: Customize behavior through configuration file

## Installation


### From Source

```bash
# Clone the repository
git clone https://github.com/SmallzooDev/fink.git
cd fink

# Build and install
cargo install --path .
```

### Prerequisites

- Rust 1.70 or higher
- Cargo

## Usage

### Quick Selection Mode (Default)

Simply run `fink` to enter quick selection mode:

```bash
fink
```

Navigate with arrow keys and press Enter to copy a prompt to clipboard.

### Management Mode

Access the full management interface:

```bash
fink --manage
# or
fink -m
```

### CLI Commands

```bash
# List all prompts
fink list

# Get a specific prompt
fink get <prompt-name>

# Create a new prompt
fink create <prompt-name> [--template <template-name>]

# Edit an existing prompt
fink edit <prompt-name>

# Delete a prompt
fink delete <prompt-name> [--force]

# Copy a prompt to clipboard
fink copy <prompt-name>

# Search for prompts
fink search <query>
```

### Keyboard Shortcuts

#### Quick Selection Mode
- `↑/↓` or `j/k`: Navigate prompts
- `Enter`: Copy selected prompt and exit
- `/`: Start search
- `t`: Open tag filter
- `Tab`: Switch to management mode
- `Esc` or `q`: Exit

#### Management Mode
- `n`: Create new prompt
- `e`: Edit selected prompt
- `d`: Delete selected prompt
- `b`: Enter build mode
- `Space`: Toggle tag filter
- `Ctrl+C`: Copy to clipboard
- Additional vim-style navigation supported

## Configuration

fink stores its configuration at `~/.config/fink/config.toml`:

```toml
[storage]
path = "~/.fink/prompts"

[editor]
command = "vim"
```

## Prompt Storage

Prompts are stored as Markdown files with frontmatter metadata in `~/.fink/prompts/` by default:

```markdown
---
name: "Code Review Assistant"
description: "Python code review prompt"
tags: ["code", "review", "python"]
type: "prompt"
created_at: "2024-01-15T10:30:00Z"
modified_at: "2024-01-16T14:20:00Z"
---

You are an experienced Python developer...
```

## Build Mode

Build mode allows you to combine multiple prompts into a single output:

1. Enter build mode with `b` in management mode
2. Select prompts to include
3. Arrange them in desired order
4. Copy the combined result

## Templates

fink includes several built-in templates:

- **basic**: Simple prompt structure
- **role-based**: Define role, context, and examples
- **few-shot**: Include multiple examples
- **chain-of-thought**: Step-by-step reasoning

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Clipboard functionality via [clipboard](https://github.com/aweinstock314/rust-clipboard)
