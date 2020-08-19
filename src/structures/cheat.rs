use crate::structures::finder::Opts;
use crate::structures::fnv::HashLine;
use std::collections::HashMap;

pub type Suggestion = (String, Option<Opts>);

#[derive(Clone)]
pub struct VariableMap(HashMap<u64, HashMap<String, Suggestion>>);

impl VariableMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, tags: &str, variable: &str, value: Suggestion) {
        let k1 = tags.hash_line();
        let k2 = String::from(variable);
        if let Some(m) = self.0.get_mut(&k1) {
            m.insert(k2, value);
        } else {
            let mut m = HashMap::new();
            m.insert(k2, value);
            self.0.insert(k1, m);
        }
    }

    pub fn get(&self, tags: &str, variable: &str) -> Option<&Suggestion> {
        self.0.get(&tags.hash_line())?.get(variable)
    }
}
