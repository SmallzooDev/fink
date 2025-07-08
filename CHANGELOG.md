# Changelog

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