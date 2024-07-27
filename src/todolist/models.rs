use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Clone, Deserialize)]
pub struct UpdateEntryData {
    pub title: String,
}
