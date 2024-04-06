use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::history_entry::HistoryEntry;
use crate::model::serialize_date;

const ISSUE_TYPE: &str = "Story";

#[derive(Debug, Serialize)]
pub struct Issue {
    status: String,
    reporter: String,
    #[serde(rename = "issueType")]
    issue_type: String,
    #[serde(serialize_with = "serialize_date")]
    created: DateTime<Utc>,
    #[serde(serialize_with = "serialize_date")]
    updated: DateTime<Utc>,
    summary: String,
    history: Vec<HistoryEntry>,
}

impl Issue {
    pub fn new(
        status: String,
        reporter: String,
        created: DateTime<Utc>,
        updated: DateTime<Utc>,
        summary: String,
        history: Vec<HistoryEntry>,
    ) -> Self {
        let issue_type = String::from(ISSUE_TYPE);
        Self { status, reporter, issue_type, created, updated, summary, history }
    }
}
