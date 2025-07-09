#[derive(Debug, PartialEq)]
pub struct Segment {
    pub text: String,
    pub is_match: bool,
}

pub struct HighlightedText;

impl Default for HighlightedText {
    fn default() -> Self {
        Self
    }
}

impl HighlightedText {
    pub fn new() -> Self {
        Self
    }

    pub fn highlight(&self, text: &str, query: &str) -> HighlightedResult {
        if query.is_empty() {
            return HighlightedResult {
                segments: vec![Segment {
                    text: text.to_string(),
                    is_match: false,
                }],
            };
        }

        let mut segments = Vec::new();
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();
        
        let mut last_end = 0;
        
        // Find all matches
        let mut matches = Vec::new();
        let mut search_start = 0;
        
        while let Some(start) = text_lower[search_start..].find(&query_lower) {
            let absolute_start = search_start + start;
            let absolute_end = absolute_start + query.len();
            matches.push((absolute_start, absolute_end));
            search_start = absolute_end;
        }
        
        // Build segments
        for (start, end) in matches {
            // Add non-matching text before this match
            if last_end < start {
                segments.push(Segment {
                    text: text[last_end..start].to_string(),
                    is_match: false,
                });
            }
            
            // Add the matching text (preserving original case)
            segments.push(Segment {
                text: text[start..end].to_string(),
                is_match: true,
            });
            
            last_end = end;
        }
        
        // Add any remaining text after the last match
        if last_end < text.len() {
            segments.push(Segment {
                text: text[last_end..].to_string(),
                is_match: false,
            });
        }
        
        HighlightedResult { segments }
    }
}

pub struct HighlightedResult {
    pub segments: Vec<Segment>,
}