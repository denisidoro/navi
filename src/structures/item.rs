pub struct Item<'a> {
    pub tags: &'a str,
    pub comment: &'a str,
    pub snippet: &'a str,
    pub file_index: &'a usize,
}
