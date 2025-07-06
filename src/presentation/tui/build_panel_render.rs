use crate::presentation::tui::tui::TUIApp;
use crate::presentation::tui::screens::BuildScreen;
use ratatui::{
    prelude::*,
};

pub fn render_build_mode(app: &mut TUIApp, f: &mut Frame, area: Rect) {
    let mut screen = BuildScreen::new(app);
    screen.render(f, area);
}