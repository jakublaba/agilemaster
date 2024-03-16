use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::item::Item;
use crate::model::serialize_date;

#[derive(Debug, Serialize)]
pub struct HistoryEntry<'l> {
    author: &'l str,
    #[serde(serialize_with = "serialize_date")]
    created: DateTime<Utc>,
    items: Vec<Item<'l>>,
}

impl<'l> HistoryEntry<'l> {
    pub fn new(
        author: &'l str,
        created: DateTime<Utc>,
        items: Vec<Item<'l>>,
    ) -> Self {
        Self { author, created, items }
    }
}
