use crate::structures::fnv::HashLine;
use std::collections::HashMap;
use crate::structures::fzf::Opts;

pub type Suggestion = (String, Option<Opts>);

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
