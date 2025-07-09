use crate::presentation::tui::tui::{TUIApp, AppMode};
use crate::presentation::tui::screens::QuickSelectScreen;
use crate::utils::config::Config;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, terminal};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::path::PathBuf;

// TUI runner functions for managing the terminal user interface

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    fn handle_global_shortcuts(&self, app: &mut TUIApp, key: &KeyEvent) -> bool {
        // Global Ctrl+C handler - exits from anywhere
        if key.code == KeyCode::Char('c') && key.modifiers.contains(event::KeyModifiers::CONTROL) {
            app.quit();
            return true;
        }
        
        // Clear any error or success message on key press
        if app.has_error() {
            app.clear_error();
            return true;
        }
        
        if app.has_success() {
            app.clear_success();
            return true;
        }
        
        false
    }
    
    fn handle_init_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Err(e) = app.accept_init_dialog() {
                    app.set_error(format!("Failed to initialize prompts: {}", e));
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                if let Err(e) = app.decline_init_dialog() {
                    app.set_error(format!("Failed to save preference: {}", e));
                }
            }
            _ => {} // Ignore other keys while init dialog is showing
        }
        Ok(())
    }
    
    fn handle_type_prompts_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Err(e) = app.accept_type_prompts_dialog() {
                    app.set_error(format!("Failed to initialize type-specific prompts: {}", e));
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.decline_type_prompts_dialog();
            }
            _ => {} // Ignore other keys while type prompts dialog is showing
        }
        Ok(())
    }
    
    fn handle_confirmation_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                app.confirm_action()?;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.cancel_confirmation();
            }
            _ => {} // Ignore other keys while confirmation dialog is showing
        }
        Ok(())
    }
    
    fn handle_tag_filter_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        let mut should_close = false;
        let mut should_apply = false;
        
        if let Some(filter_dialog) = app.get_tag_filter_dialog_mut() {
            if filter_dialog.is_searching {
                // Search mode
                match key.code {
                    KeyCode::Esc => {
                        should_close = true;
                    }
                    KeyCode::Tab => {
                        filter_dialog.toggle_mode();
                    }
                    KeyCode::Enter => {
                        should_apply = true;
                        should_close = true;
                    }
                    KeyCode::Char(c) => {
                        filter_dialog.add_char(c);
                    }
                    KeyCode::Backspace => {
                        filter_dialog.delete_char();
                    }
                    _ => {}
                }
            } else {
                // Selection mode
                match key.code {
                    KeyCode::Esc => {
                        should_close = true;
                    }
                    KeyCode::Tab => {
                        filter_dialog.toggle_mode();
                    }
                    KeyCode::Up => {
                        filter_dialog.move_up();
                    }
                    KeyCode::Down => {
                        filter_dialog.move_down();
                    }
                    KeyCode::Char(' ') => {
                        filter_dialog.toggle_selected_tag();
                    }
                    KeyCode::Enter => {
                        should_apply = true;
                        should_close = true;
                    }
                    KeyCode::Char('c') => {
                        filter_dialog.clear_selection();
                    }
                    _ => {}
                }
            }
        }
        
        if should_apply {
            if let Some(filter_dialog) = app.get_tag_filter_dialog() {
                let selected_tags = filter_dialog.get_selected_tags();
                app.set_tag_filters(selected_tags);
            }
        }
        
        if should_close {
            app.close_tag_filter();
        }
        
        Ok(())
    }
    
    fn handle_tag_management_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        use crate::presentation::tui::components::TagInputMode;
        
        let mut should_close = false;
        let mut new_tag_to_add = None;
        let mut tag_to_remove = None;
        let mut should_refresh = false;
        
        // First, handle the dialog input
        if let Some(tag_dialog) = app.get_tag_dialog_mut() {
            match tag_dialog.input_mode() {
                TagInputMode::ViewTags => {
                    match key.code {
                        KeyCode::Esc => {
                            should_close = true;
                        }
                        KeyCode::Char('a') => {
                            tag_dialog.start_adding_tag();
                        }
                        KeyCode::Char('r') => {
                            tag_dialog.start_removing_tag();
                        }
                        _ => {}
                    }
                }
                TagInputMode::AddingTag => {
                    match key.code {
                        KeyCode::Esc => {
                            tag_dialog.cancel_input();
                        }
                        KeyCode::Enter => {
                            new_tag_to_add = tag_dialog.get_new_tag();
                            tag_dialog.cancel_input();
                            should_refresh = true;
                        }
                        KeyCode::Char(c) => {
                            tag_dialog.add_char(c);
                        }
                        KeyCode::Backspace => {
                            tag_dialog.delete_char();
                        }
                        _ => {}
                    }
                }
                TagInputMode::RemovingTag => {
                    match key.code {
                        KeyCode::Esc => {
                            tag_dialog.cancel_input();
                        }
                        KeyCode::Up => {
                            tag_dialog.move_selection_up();
                        }
                        KeyCode::Down => {
                            tag_dialog.move_selection_down();
                        }
                        KeyCode::Enter => {
                            tag_to_remove = tag_dialog.get_selected_tag_for_removal();
                            tag_dialog.cancel_input();
                            should_refresh = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Now handle the actions outside of the mutable borrow
        if should_close {
            app.close_tag_management();
        }
        
        if let Some(new_tag) = new_tag_to_add {
            if let Err(e) = app.add_tag_to_selected(&new_tag) {
                app.set_error(format!("Error adding tag: {}", e));
            }
        }
        
        if let Some(tag) = tag_to_remove {
            if let Err(e) = app.remove_tag_from_selected(&tag) {
                app.set_error(format!("Error removing tag: {}", e));
            }
        }
        
        if should_refresh {
            // Refresh the dialog with updated tags
            let updated_tags = app.get_selected_prompt_tags();
            app.tag_dialog = Some(crate::presentation::tui::components::TagManagementDialog::new(updated_tags));
        }
        
        Ok(())
    }
    
    fn handle_create_dialog(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        use crate::presentation::tui::components::DialogField;
        
        let mut should_close = false;
        let mut should_confirm = false;
        
        if let Some(create_dialog) = app.get_create_dialog_mut() {
            match key.code {
                KeyCode::Esc => {
                    should_close = true;
                }
                KeyCode::Tab => {
                    create_dialog.next_field();
                }
                KeyCode::Enter => {
                    if create_dialog.is_valid() {
                        should_confirm = true;
                    }
                }
                KeyCode::Left => {
                    if create_dialog.current_field() == DialogField::Template {
                        create_dialog.previous_template();
                    } else if create_dialog.current_field() == DialogField::Type {
                        create_dialog.previous_type();
                    }
                }
                KeyCode::Right => {
                    if create_dialog.current_field() == DialogField::Template {
                        create_dialog.next_template();
                    } else if create_dialog.current_field() == DialogField::Type {
                        create_dialog.next_type();
                    }
                }
                KeyCode::Char('h') => {
                    if create_dialog.current_field() == DialogField::Template {
                        create_dialog.previous_template();
                    } else if create_dialog.current_field() == DialogField::Type {
                        create_dialog.previous_type();
                    } else {
                        create_dialog.add_char('h');
                    }
                }
                KeyCode::Char('l') => {
                    if create_dialog.current_field() == DialogField::Template {
                        create_dialog.next_template();
                    } else if create_dialog.current_field() == DialogField::Type {
                        create_dialog.next_type();
                    } else {
                        create_dialog.add_char('l');
                    }
                }
                KeyCode::Char(c) => {
                    if create_dialog.current_field() == DialogField::Filename {
                        create_dialog.add_char(c);
                    }
                }
                KeyCode::Backspace => {
                    if create_dialog.current_field() == DialogField::Filename {
                        create_dialog.delete_char();
                    }
                }
                _ => {}
            }
        }
        
        if should_close {
            app.close_create_dialog();
        }
        
        if should_confirm {
            if let Err(e) = app.confirm_create() {
                app.set_error(format!("Failed to create prompt: {}", e));
            }
        }
        
        Ok(())
    }
    
    fn handle_build_mode(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        if let Some(panel) = app.get_interactive_build_panel_mut() {
            use crate::presentation::tui::components::BuildStep;
            
            match panel.current_step {
                BuildStep::AddComment => {
                    // Handle comment input
                    match key.code {
                        KeyCode::Esc => {
                            panel.finish_comment();
                        }
                        KeyCode::Enter => {
                            panel.finish_comment();
                        }
                        KeyCode::Char(c) => {
                            panel.add_comment_char(c);
                        }
                        KeyCode::Backspace => {
                            panel.delete_comment_char();
                        }
                        KeyCode::Left => {
                            panel.move_cursor_left();
                        }
                        KeyCode::Right => {
                            panel.move_cursor_right();
                        }
                        _ => {}
                    }
                }
                BuildStep::Complete => {
                    // Handle completion
                    match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            if let Err(e) = app.combine_and_copy_selected_prompts() {
                                app.set_error(format!("Failed to combine prompts: {}", e));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    // Handle prompt selection steps
                    match key.code {
                        KeyCode::Esc => {
                            app.exit_build_mode();
                        }
                        KeyCode::Up => {
                            panel.previous();
                        }
                        KeyCode::Down => {
                            panel.next();
                        }
                        KeyCode::Enter => {
                            panel.select_current();
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // Fallback to old build panel if interactive panel is not available
            match key.code {
                KeyCode::Esc => {
                    app.exit_build_mode();
                }
                KeyCode::Up => {
                    if let Some(build_panel) = app.get_build_panel_mut() {
                        build_panel.previous();
                    }
                }
                KeyCode::Down => {
                    if let Some(build_panel) = app.get_build_panel_mut() {
                        build_panel.next();
                    }
                }
                KeyCode::Char(' ') => {
                    if let Some(build_panel) = app.get_build_panel_mut() {
                        build_panel.toggle_selection();
                    }
                }
                KeyCode::Enter => {
                    if let Err(e) = app.combine_and_copy_selected_prompts() {
                        app.set_error(format!("Failed to combine prompts: {}", e));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn handle_search_mode(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                app.deactivate_search();
            }
            KeyCode::Backspace => {
                let current_query = app.get_search_query();
                if !current_query.is_empty() {
                    // Use character-based manipulation to properly handle Unicode
                    let mut chars: Vec<char> = current_query.chars().collect();
                    chars.pop();
                    let new_query: String = chars.into_iter().collect();
                    app.set_search_query(&new_query);
                }
            }
            KeyCode::Enter => {
                // Keep search active but allow selection
                if matches!(app.mode(), AppMode::QuickSelect) {
                    match app.copy_selected_to_clipboard() {
                        Ok(_) => app.quit(),
                        Err(e) => app.set_error(format!("Cannot copy: {}", e)),
                    }
                } else if matches!(app.mode(), AppMode::Management) {
                    // In management mode, just close search
                    app.deactivate_search();
                }
            }
            KeyCode::Down => {
                // Allow navigation while searching
                app.next();
            }
            KeyCode::Up => {
                // Allow navigation while searching
                app.previous();
            }
            KeyCode::Char(c) => {
                // Add character to search query
                let current_query = app.get_search_query().to_string();
                app.set_search_query(&format!("{}{}", current_query, c));
            }
            _ => {} // Ignore other keys in search mode
        }
        Ok(())
    }
    
    fn handle_normal_mode(&self, app: &mut TUIApp, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Down => {
                app.next();
            }
            KeyCode::Up => {
                app.previous();
            }
            KeyCode::Enter => {
                // In QuickSelect mode, copy and quit
                // In Management mode, we'll handle this differently later
                if matches!(app.mode(), AppMode::QuickSelect) {
                    match app.copy_selected_to_clipboard() {
                        Ok(_) => app.quit(),
                        Err(e) => app.set_error(format!("Cannot copy: {}", e)),
                    }
                }
            }
            KeyCode::Char('m') => {
                app.toggle_mode();
            }
            KeyCode::Char('s') => {
                if matches!(app.mode(), AppMode::QuickSelect | AppMode::Management) {
                    if let Err(e) = app.toggle_star_on_selected() {
                        app.set_error(e.to_string());
                    }
                }
            }
            KeyCode::Char('e') => {
                if matches!(app.mode(), AppMode::Management) {
                    // For now, just mark that edit was requested
                    // The actual editor launch will be handled in the main loop
                    app.set_pending_action(Some(crate::presentation::tui::tui::PendingAction::Edit));
                }
            }
            KeyCode::Char('d') => {
                if matches!(app.mode(), AppMode::Management) {
                    app.show_delete_confirmation();
                }
            }
            KeyCode::Char('n') => {
                if matches!(app.mode(), AppMode::Management) {
                    if let Err(e) = app.create_new_prompt() {
                        app.set_error(format!("Error creating prompt: {}", e));
                    }
                }
            }
            KeyCode::Char('t') => {
                if matches!(app.mode(), AppMode::Management) {
                    app.open_tag_management();
                }
            }
            KeyCode::Char('f') => {
                // Open tag filter dialog in both modes
                app.open_tag_filter();
            }
            KeyCode::Char('F') => {
                // Clear all tag filters
                app.clear_tag_filters();
            }
            KeyCode::Char('b') => {
                // Enter build mode from QuickSelect or Management mode
                if matches!(app.mode(), AppMode::QuickSelect | AppMode::Management) {
                    app.enter_build_mode();
                }
            }
            KeyCode::Char('/') => {
                app.activate_search();
            }
            _ => {}
        }
        Ok(())
    }
    
    pub fn handle_event(&self, app: &mut TUIApp, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            // Handle global shortcuts first
            if self.handle_global_shortcuts(app, &key) {
                return Ok(());
            }
            
            // Handle initialization dialog if showing
            if app.is_showing_init_dialog() {
                return self.handle_init_dialog(app, &key);
            }
            
            // Handle type prompts dialog if showing
            if app.is_showing_type_prompts_dialog() {
                return self.handle_type_prompts_dialog(app, &key);
            }
            
            // Handle confirmation dialog if showing
            if app.is_showing_confirmation() {
                return self.handle_confirmation_dialog(app, &key);
            }
            
            // Handle tag filter dialog if showing
            if app.is_tag_filter_dialog_active() {
                return self.handle_tag_filter_dialog(app, &key);
            }
            
            // Handle tag management dialog if showing
            if app.is_tag_management_active() {
                return self.handle_tag_management_dialog(app, &key);
            }
            
            // Handle create dialog if showing
            if app.is_create_dialog_active() {
                return self.handle_create_dialog(app, &key);
            }

            // Handle build mode
            if app.is_build_mode() {
                return self.handle_build_mode(app, &key);
            }

            // Handle search mode
            if app.is_search_active() {
                return self.handle_search_mode(app, &key);
            }

            // Normal key handling
            self.handle_normal_mode(app, &key)
        } else {
            Ok(())
        }
    }
}

pub fn run(base_path: PathBuf, config: &Config) -> Result<()> {
    run_with_mode(base_path, config, false)
}

pub fn run_manage_mode(base_path: PathBuf, config: &Config) -> Result<()> {
    run_with_mode(base_path, config, true)
}

fn run_with_mode(_base_path: PathBuf, config: &Config, manage_mode: bool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mode = if manage_mode { AppMode::Management } else { AppMode::QuickSelect };
    let mut app = TUIApp::new_with_mode_and_config(config, mode)?;
    let event_handler = EventHandler::new();

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| {
            // Always render the basic screen first
            let screen = QuickSelectScreen::new(&app);
            screen.render(f, f.size());
            
            // Handle build mode rendering in the same draw call
            if app.is_build_mode() {
                if let Some(panel) = app.get_interactive_build_panel_mut() {
                    panel.render(f, f.size());
                }
            }
        })?;

        // Handle events
        if let Ok(event) = event::read() {
            event_handler.handle_event(&mut app, event)?;
        }

        // Handle pending actions that require exiting TUI temporarily
        if let Some(action) = app.take_pending_action() {
            match action {
                crate::presentation::tui::tui::PendingAction::Edit => {
                    // Exit TUI temporarily
                    disable_raw_mode()?;
                    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
                    
                    // Edit the prompt
                    let result = app.edit_selected();
                    
                    // Restore TUI
                    enable_raw_mode()?;
                    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
                    
                    // Force a full redraw by clearing the terminal
                    terminal.clear()?;
                    
                    if let Err(e) = result {
                        app.set_error(format!("Error editing prompt: {}", e));
                    }
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}