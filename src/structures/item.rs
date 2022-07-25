#[derive(Default)]
pub struct Item {
    pub tags: String,
    pub comment: String,
    pub snippet: String,
    pub file_index: Option<usize>,
}
