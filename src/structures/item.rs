use crate::common::hash::fnv;

#[derive(Default, Debug)]
pub struct Item {
    pub tags: String,
    pub comment: String,
    pub snippet: String,
    pub file_index: Option<usize>,
    pub icon: Option<String>,
}

impl Item {
    pub fn new(file_index: Option<usize>) -> Self {
        Self {
            file_index,
            ..Default::default()
        }
    }

    pub fn hash(&self) -> u64 {
        fnv(&format!("{}{}", &self.tags.trim(), &self.comment.trim()))
    }
}
