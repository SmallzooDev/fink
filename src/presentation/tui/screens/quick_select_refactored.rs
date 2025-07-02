use crate::presentation::tui::tui::{TUIApp, AppMode};
use crate::presentation::tui::rendering::{
    StandardLayout, UIStyles, SplitPane, FooterBuilder, ListItemBuilder, PreviewRenderer,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, Paragraph},
};

pub struct QuickSelectScreen<'a> {
    app: &'a TUIApp,
}

impl<'a> QuickSelectScreen<'a> {
    pub fn new(app: &'a TUIApp) -> Self {
        Self { app }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Use StandardLayout for consistent structure
        let layout = StandardLayout::new()
            .header_height(3)
            .footer_height(3)
            .build(area);

        // Render header
        self.render_header(f, layout.header);

        // Split content area for list and preview
        let (list_area, preview_area) = SplitPane::horizontal()
            .ratio(40, 60)
            .split(layout.content);
        
        self.render_prompt_list(f, list_area);
        self.render_preview_pane(f, preview_area);

        // Render footer
        self.render_footer(f, layout.footer);

        // Render confirmation dialog if showing
        if let Some(dialog) = self.app.get_confirmation_dialog() {
            dialog.render(f, area);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let mode_text = match self.app.mode() {
            AppMode::QuickSelect => "jkms Manager - Quick Select",
            AppMode::Management => "jkms Manager - Management Mode",
        };
        
        let header = Paragraph::new(mode_text)
            .block(UIStyles::header_block(""));
        f.render_widget(header, area);
    }

    fn render_prompt_list(&self, f: &mut Frame, area: Rect) {
        let prompts = self.app.get_prompts();
        let selected_index = self.app.selected_index();
        
        // Use ListItemBuilder for consistent item creation
        let items: Vec<_> = prompts
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                ListItemBuilder::build_prompt_item(
                    &p.name,
                    &p.tags,
                    idx == selected_index
                )
            })
            .collect();

        let list = List::new(items)
            .block(UIStyles::header_block("Prompts"))
            .highlight_style(UIStyles::selection_highlight());

        let mut list_state = self.app.get_list_state();
        f.render_stateful_widget(list, area, &mut list_state);
    }

    fn render_preview_pane(&self, f: &mut Frame, area: Rect) {
        let content = self.app.get_selected_content();
        
        // Use PreviewRenderer for consistent preview rendering
        PreviewRenderer::render(
            f,
            area,
            content.as_deref(),
            "Preview"
        );
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_content = match self.app.mode() {
            AppMode::QuickSelect => FooterBuilder::quick_select_footer(),
            AppMode::Management => FooterBuilder::management_footer(),
        };
        
        let footer = Paragraph::new(footer_content)
            .block(UIStyles::footer_block());
        f.render_widget(footer, area);
    }
}