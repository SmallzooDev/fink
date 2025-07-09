use fink::presentation::tui::app::TUIApp;
use fink::utils::text_scroll::get_visible_text_end_aligned;
use tempfile::tempdir;

#[test]
fn test_search_input_scrolling() {
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Type a very long search query
    let long_text = "this is a very long search query that definitely exceeds the visible width of the search input field";
    app.set_search_query(long_text);
    
    // The search query should be stored completely
    assert_eq!(app.get_search_query(), long_text);
    
    // Test the scrolling function
    let visible = get_visible_text_end_aligned(long_text, 20);
    assert_eq!(visible.len(), 20);
    assert!(visible.ends_with("field"));
}

#[test]
fn test_text_scroll_utility() {
    // Test short text that fits
    let short = "hello";
    let visible = get_visible_text_end_aligned(short, 10);
    assert_eq!(visible, "hello");
    
    // Test long text that needs scrolling
    let long = "this is a very long text that needs scrolling";
    let visible = get_visible_text_end_aligned(long, 10);
    assert_eq!(visible, " scrolling");
    assert_eq!(visible.len(), 10);
}