#[derive(Default)]
pub struct Item {
    pub tags: String,
    pub comment: String,
    pub snippet: String,
    pub file_index: Option<usize>,
}

impl Item {
    pub fn new(file_index: Option<usize>) -> Self {
        Self {
            file_index,
            ..Default::default()
        }
    }
}
