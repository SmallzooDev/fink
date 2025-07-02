use crate::presentation::tui::tui::{TUIApp, AppMode};
use crate::presentation::tui::screens::QuickSelectScreen;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, terminal};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::path::PathBuf;

pub struct TUI {
    app: TUIApp,
}

impl TUI {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let app = TUIApp::new(base_path)?;
        Ok(Self { app })
    }

    pub fn app(&self) -> &TUIApp {
        &self.app
    }
}

pub fn run_app(base_path: PathBuf) -> Result<TUI> {
    TUI::new(base_path)
}

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
    pub fn handle_event(&self, app: &mut TUIApp, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.next();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.previous();
                }
                KeyCode::Enter => {
                    // In QuickSelect mode, copy and quit
                    // In Management mode, we'll handle this differently later
                    if matches!(app.mode(), AppMode::QuickSelect) {
                        app.copy_selected_to_clipboard()?;
                        app.quit();
                    }
                }
                KeyCode::Char('m') => {
                    app.toggle_mode();
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub fn run(base_path: PathBuf) -> Result<()> {
    run_with_mode(base_path, false)
}

pub fn run_manage_mode(base_path: PathBuf) -> Result<()> {
    run_with_mode(base_path, true)
}

fn run_with_mode(base_path: PathBuf, manage_mode: bool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mode = if manage_mode { AppMode::Management } else { AppMode::QuickSelect };
    let mut app = TUIApp::new_with_mode(base_path.clone(), mode)?;
    let event_handler = EventHandler::new();

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| {
            if manage_mode {
                // TODO: Render management screen
                let screen = QuickSelectScreen::new(&app);
                screen.render(f, f.size());
            } else {
                let screen = QuickSelectScreen::new(&app);
                screen.render(f, f.size());
            }
        })?;

        // Handle events
        if let Ok(event) = event::read() {
            event_handler.handle_event(&mut app, event)?;
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