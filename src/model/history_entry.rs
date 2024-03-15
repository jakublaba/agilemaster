use serde::Serialize;

use crate::model::item::Item;
use crate::model::jira_date::JiraDate;

#[derive(Debug, Serialize)]
pub struct HistoryEntry<'l> {
    author: &'l str,
    created: JiraDate<'l>,
    items: Vec<Item<'l>>,
}

impl<'l> HistoryEntry<'l> {
    pub fn new(
        author: &'l str,
        created: JiraDate<'l>,
        items: Vec<Item<'l>>,
    ) -> Self {
        Self { author, created, items }
    }
}
