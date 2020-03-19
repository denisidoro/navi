use crate::structures::cheat::SuggestionType;

pub struct Opts {
    pub query: Option<String>,
    pub filter: Option<String>,
    pub prompt: Option<String>,
    pub preview: Option<String>,
    pub autoselect: bool,
    pub overrides: Option<String>,
    pub header_lines: u8,
    pub header: Option<String>,
    pub suggestion_type: SuggestionType,
    pub delimiter: Option<String>,
    pub column: Option<u8>,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            query: None,
            filter: None,
            autoselect: true,
            preview: None,
            overrides: None,
            header_lines: 0,
            header: None,
            prompt: None,
            suggestion_type: SuggestionType::SingleSelection,
            column: None,
            delimiter: None,
        }
    }
}
