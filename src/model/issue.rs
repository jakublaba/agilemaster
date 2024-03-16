use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::history_entry::HistoryEntry;
use crate::model::serialize_date;

#[derive(Debug, Serialize)]
pub struct Issue<'l> {
    status: &'l str,
    reporter: &'l str,
    #[serde(rename = "issueType")]
    issue_type: &'l str,
    #[serde(serialize_with = "serialize_date")]
    created: DateTime<Utc>,
    #[serde(serialize_with = "serialize_date")]
    updated: DateTime<Utc>,
    summary: &'l str,
    history: Vec<HistoryEntry<'l>>,
}

impl<'l> Issue<'l> {
    pub fn new(
        status: &'l str,
        reporter: &'l str,
        issue_type: &'l str,
        created: DateTime<Utc>,
        updated: DateTime<Utc>,
        summary: &'l str,
        history: Vec<HistoryEntry<'l>>,
    ) -> Self {
        Self { status, reporter, issue_type, created, updated, summary, history }
    }
}
