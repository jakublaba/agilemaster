use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::history_item::HistoryItem;
use crate::model::serialize_date;

#[derive(Debug, Serialize)]
pub struct HistoryEntry {
    author: String,
    #[serde(serialize_with = "serialize_date")]
    created: DateTime<Utc>,
    items: Vec<HistoryItem>,
}

impl HistoryEntry {
    pub fn new(
        author: String,
        created: DateTime<Utc>,
        items: Vec<HistoryItem>,
    ) -> Self {
        Self { author, created, items }
    }
}
