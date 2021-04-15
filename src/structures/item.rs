pub struct Item {
    pub tags: String,
    pub comment: String,
    pub snippet: String,
    pub file_index: usize,
}

impl Item {
    pub fn new() -> Self {
        Self {
            tags: "".to_string(),
            comment: "".to_string(),
            snippet: "".to_string(),
            file_index: 0,
        }
    }
}
