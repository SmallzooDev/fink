# jkms TODO - Phase 2 Remaining Tasks

## Phase 2: Management Features

### CLI Commands Implementation
- [x] Implement `jkms list` command to list all prompts
- [x] Implement `jkms get <name>` command to display prompt content
- [ ] Implement `jkms create <name> [--template <template>]` command
- [ ] Implement `jkms edit <name>` command with external editor integration
- [ ] Implement `jkms delete <name>` command with confirmation
- [ ] Implement `jkms copy <name>` command (non-interactive copy)
- [ ] Implement `jkms search <query>` command

### Management Mode UI
- [ ] Add `--manage` / `-m` flag to enter management mode
- [ ] Create management screen with options (Edit, Delete, New, Settings)
- [ ] Implement preview pane in management mode
- [ ] Add confirmation dialogs for destructive actions

### Search and Filtering
- [ ] Implement search functionality in TUI (Ctrl+F to activate)
- [ ] Add search by name, content, and tags
- [ ] Implement real-time filtering as user types
- [ ] Add search highlighting in results

### Tag System
- [x] Parse tags from frontmatter
- [x] Display tags in prompt list
- [ ] Implement tag-based filtering
- [ ] Add tag management (add/remove tags)

### Additional UI Improvements
- [ ] Add status bar showing current mode and hints
- [ ] Implement favorites/starred prompts (⭐ indicator)
- [ ] Add prompt metadata display (created date, modified date)
- [ ] Improve error handling and user feedback

### Code Organization
- [x] Create CLI commands module structure
- [ ] Add input validation for all commands
- [x] Implement proper error messages
- [x] Add integration tests for all CLI commands