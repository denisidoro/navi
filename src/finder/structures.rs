use crate::filesystem;
use crate::prelude::*;

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
    pub show_all_columns: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            query: None,
            filter: None,
            preview: None,
            preview_window: None,
            overrides: None,
            header_lines: 0,
            header: None,
            prompt: None,
            suggestion_type: SuggestionType::SingleSelection,
            column: None,
            delimiter: None,
            map: None,
            prevent_select1: true,
            show_all_columns: false,
        }
    }
}

impl Opts {
    pub fn snippet_default() -> Self {
        Self {
            suggestion_type: SuggestionType::SnippetSelection,
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
            ..Default::default()
        }
    }

    pub fn var_default() -> Self {
        Self {
            overrides: CONFIG.fzf_overrides_var(),
            suggestion_type: SuggestionType::SingleRecommendation,
            prevent_select1: false,
            ..Default::default()
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
