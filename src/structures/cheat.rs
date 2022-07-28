use crate::common::hash::fnv;
use crate::finder::structures::Opts;
use crate::prelude::*;

pub type Suggestion = (String, Option<Opts>);

#[derive(Clone, Default)]
pub struct VariableMap {
    variables: HashMap<u64, HashMap<String, Suggestion>>,
    dependencies: HashMap<u64, Vec<u64>>,
}

impl VariableMap {
    pub fn insert_dependency(&mut self, tags: &str, tags_dependency: &str) {
        let k = fnv(&tags);
        if let Some(v) = self.dependencies.get_mut(&k) {
            v.push(fnv(&tags_dependency));
        } else {
            let v: Vec<u64> = vec![fnv(&tags_dependency)];
            self.dependencies.insert(k, v);
        }
    }

    pub fn insert_suggestion(&mut self, tags: &str, variable: &str, value: Suggestion) {
        let k1 = fnv(&tags);
        let k2 = String::from(variable);
        if let Some(m) = self.variables.get_mut(&k1) {
            m.insert(k2, value);
        } else {
            let mut m = HashMap::new();
            m.insert(k2, value);
            self.variables.insert(k1, m);
        }
    }

    pub fn get_suggestion(&self, tags: &str, variable: &str) -> Option<&Suggestion> {
        let k = fnv(&tags);

        if let Some(vm) = self.variables.get(&k) {
            let res = vm.get(variable);
            if res.is_some() {
                return res;
            }
        }

        if let Some(dependency_keys) = self.dependencies.get(&k) {
            for dependency_key in dependency_keys {
                if let Some(vm) = self.variables.get(dependency_key) {
                    let res = vm.get(variable);
                    if res.is_some() {
                        return res;
                    }
                }
            }
        }

        None
    }
}
