use crate::config::Config;
use crate::filesystem;
use anyhow::Result;

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
    pub select1: bool,
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
            suggestion_type: SuggestionType::SingleRecommendation,
            column: None,
            delimiter: None,
            map: None,
            select1: true,
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

impl Opts {
    pub fn from_config(config: &Config) -> Result<Opts> {
        let opts = Opts {
            preview: Some(format!("{} preview {{}}", filesystem::exe_string())),
            overrides: config.fzf_overrides(),
            suggestion_type: SuggestionType::SnippetSelection,
            query: if config.best_match() {
                None
            } else {
                config.get_query()
            },
            filter: if config.best_match() {
                config.get_query()
            } else {
                None
            },
            ..Default::default()
        };

        Ok(opts)
    }
}
