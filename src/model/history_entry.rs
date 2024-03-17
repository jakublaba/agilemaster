use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::history_item::HistoryItem;
use crate::model::serialize_date;

#[derive(Debug, Serialize)]
pub struct HistoryEntry<'l> {
    author: &'l str,
    #[serde(serialize_with = "serialize_date")]
    created: DateTime<Utc>,
    items: Vec<HistoryItem<'l>>,
}

impl<'l> HistoryEntry<'l> {
    pub fn new(
        author: &'l str,
        created: DateTime<Utc>,
        items: Vec<HistoryItem<'l>>,
    ) -> Self {
        Self { author, created, items }
    }
}
