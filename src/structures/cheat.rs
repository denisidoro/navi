use crate::structures::fnv::HashLine;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SuggestionOpts {
    pub header_lines: u8,
    pub column: Option<u8>,
    pub delimiter: Option<String>,
    pub suggestion_type: SuggestionType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestionType {
    /// fzf will not print any suggestions
    Disabled,
    /// fzf will only select one of the suggestions
    SingleSelection,
    /// fzf will select multiple suggestions
    MultipleSelections,
    /// fzf will select one of the suggestions or use the query
    SingleRecommendation,
    /// initial snippet selection
    SnippetSelection,
}

pub type Suggestion = (String, Option<SuggestionOpts>);

fn gen_key(tags: &str, variable: &str) -> u64 {
    format!("{};{}", tags, variable).hash_line()
}

pub struct VariableMap(HashMap<u64, Suggestion>);

impl VariableMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, tags: &str, variable: &str, value: Suggestion) -> Option<Suggestion> {
        self.0.insert(gen_key(tags, variable), value)
    }

    pub fn get(&self, tags: &str, variable: &str) -> Option<&Suggestion> {
        self.0.get(&gen_key(tags, variable))
    }
}
