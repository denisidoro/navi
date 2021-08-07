use crate::config::CONFIG;
use crate::filesystem;

#[derive(Debug, PartialEq, Clone)]
pub struct Opts {
    pub query: Option<String>,
    pub filter: Option<String>,
    pub prompt: Option<String>,
    pub preview: Option<String>,
    pub preview_window: Option<String>,
    pub overrides: Option<String>,
    pub header_lines: u8,
    pub header: Option<String>,
    pub suggestion_type: SuggestionType,
    pub delimiter: Option<String>,
    pub column: Option<u8>,
    pub map: Option<String>,
    pub prevent_select1: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            preview_window: None,
            header_lines: 0,
            header: None,
            prompt: None,
            suggestion_type: SuggestionType::SnippetSelection,
            column: None,
            delimiter: None,
            map: None,
            overrides: CONFIG.fzf_overrides(),
            preview: Some(format!("{} preview {{}}", filesystem::exe_string())),
            prevent_select1: !CONFIG.best_match(),
            query: if CONFIG.best_match() {
                None
            } else {
                CONFIG.get_query()
            },
            filter: if CONFIG.best_match() {
                CONFIG.get_query()
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestionType {
    /// finder will not print any suggestions
    Disabled,
    /// finder will only select one of the suggestions
    SingleSelection,
    /// finder will select multiple suggestions
    MultipleSelections,
    /// finder will select one of the suggestions or use the query
    SingleRecommendation,
    /// initial snippet selection
    SnippetSelection,
}
