# Changelog for v0.1.31

## Bug Fixes
- ✅ Fixed version detection issue
- ✅ Removed hjkl key mappings that conflicted with arrow keys
- ✅ Fixed input field scrolling for long text that exceeds visible width

## New Features

### Config Mode
- ✅ Added new config mode (access with 'c' key)
- ✅ Added clipboard prefix/postfix configuration
- ✅ Added editor selection with arrow key navigation (vim, nvim, hx, vi, code)
- ✅ Config changes are saved with Ctrl+S

### State Restoration
- ✅ Application now saves cursor position when navigating or quitting
- ✅ Cursor position is restored when reopening the app
- ✅ State is saved to ~/.config/fink/state.json

### VS Code Integration
- ✅ Added VS Code as a configurable editor option
- ✅ Platform-specific VS Code launching (macOS uses `open` command)
- ✅ Visual feedback when editing with external editors
- ✅ Press 'e' again while editing externally to refresh the prompt

## Technical Improvements
- Added RefCell for interior mutability in editor launcher
- Improved config system with proper path expansion
- Added test isolation to prevent config corruption during tests
- Fixed borrow checker issues with external editing state

## Testing
- All unit tests passing
- Integration tests passing (except one flaky test)
- State restoration tests confirmed working
- Config mode tests updated for new editor field