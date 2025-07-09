# Changelog

## [0.1.4] - 2025-01-09

### Added
- **Config Mode** - New configuration mode accessible with 'c' key
  - Configure clipboard prefix/postfix that gets added to all copied prompts
  - Select preferred editor (vim, nvim, hx, vi, code) with arrow keys
  - Save configuration with Ctrl+S
- **State Persistence** - Application now remembers your cursor position between sessions
  - Automatically saves state when navigating or quitting
  - Restores last selected prompt when reopening the app
  - State saved to `~/.config/fink/state.json`
- **VS Code Integration** - Full support for Visual Studio Code as an editor
  - Platform-specific launching (macOS uses `open` command)
  - Visual feedback dialog when editing with external editors
  - Press 'e' again while editing to refresh the prompt
- **Text Scrolling** - Input fields now properly scroll when text exceeds visible width
  - Applies to search bar, tag filter, and all text input fields
  - Cursor position properly maintained during scrolling

### Changed
- **Editor Priority** - Config file editor setting now takes precedence over EDITOR environment variable
- **Path Handling** - Improved path expansion for `~` in storage paths
- **Code Quality** - Major refactoring to fix all clippy warnings
  - Renamed modules to avoid inception issues (`application/app.rs`, `tui/app.rs`)
  - Improved error handling with proper use of `if let` patterns
  - Added proper `Default` implementations
  - Removed redundant code patterns

### Fixed
- **Config System**
  - Fixed issue where test runs would corrupt user's actual config file
  - Added proper test isolation with `FINK_TEST_CONFIG_PATH` environment variable
  - Fixed automatic removal of `/prompts` suffix from storage path
- **Key Bindings** - Removed hjkl navigation keys that conflicted with text input
- **Clipboard** - Added newlines after prefix and before postfix for better formatting
- **External Editor** - Fixed VS Code launching on macOS
- **Borrow Checker** - Fixed multiple borrow checker issues with RefCell usage

### Technical Improvements
- Added `RefCell` for interior mutability in editor launcher
- Improved module structure and naming conventions
- Enhanced test coverage and test isolation
- Better separation of concerns in the codebase

## [0.1.3] - 2025-01-08

### Added
- **Korean/Unicode Support**: Fixed crash when using backspace with Korean and other multi-byte Unicode characters
- **Version Update Detection**: Shows release notes when launching after an update
- **Type-Specific Prompts**: Added 12 specialized prompts organized by type (Instruction, Context, Input/Output, Etc)
- **Two-Step Initialization**: First dialog for basic prompts, second dialog for type-specific prompts

### Fixed
- Unicode character handling in search, tag filter, and interactive build panel
- Proper character-based string manipulation instead of byte-based

### Changed
- Improved first-launch experience with two-step prompt initialization
- Enhanced build mode with type-specific prompt examples

## [0.1.1] - 2025-01-07

### Changed
- **Major refactoring**: Reduced `handle_event` function from 504 to 47 lines (91% reduction)
- Extracted dialog handling into separate, focused methods
- Improved error handling consistency throughout the codebase

### Optimized
- String allocations reduced in `get_filtered_prompts` method
- Added `STARRED_TAG` constant to avoid repeated string allocations
- Optimized tag comparisons to avoid unnecessary `.to_string()` calls
- Improved sorting performance using `sort_by_cached_key`
- Merged redundant terminal draw calls for better UI performance

### Fixed
- Removed unused code and TODO comments
- Fixed error messages to use consistent UI error handling

### Technical
- All tests remain green
- No breaking changes to user-facing functionality
- Maintained full backward compatibility

## [0.1.0] - 2025-01-06

### Added
- Initial release of fink CLI
- TUI-based prompt manager with quick selection mode
- Management mode for CRUD operations
- Starred prompts feature with sorting
- Tag-based filtering and management
- Search functionality
- Build mode for combining prompts
- External editor integration
- Clipboard integration
- Template system for new prompts
- YAML frontmatter support