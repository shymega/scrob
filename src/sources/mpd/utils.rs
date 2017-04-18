use std::collections::BTreeMap;

#[derive(Debug)]
pub enum RetrieveError {
    InvalidTag,
}

pub type SongTags = BTreeMap<String, String>;

pub fn get_tag(tags: &SongTags, tag: &str) -> Result<String, RetrieveError> {
    match tags.get(tag) {
        Some(x) => Ok(x.to_string()),
        None => Ok("None defined".to_string()),
    }
}
