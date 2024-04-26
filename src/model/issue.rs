use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model::{serialize_date, serialize_date_opt};
use crate::model::custom_field::CustomField;
use crate::model::history_entry::HistoryEntry;

const ISSUE_TYPE: &str = "Task";
pub const RESOLUTION_STATUS: &str = "DONE";

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
    resolution: Option<String>,
    #[serde(rename = "resolutionDate", serialize_with = "serialize_date_opt")]
    resolution_date: Option<DateTime<Utc>>,
    #[serde(rename = "customFields")]
    custom_fields: Vec<CustomField>,
}

impl Issue {
    pub fn new(
        status: String,
        reporter: String,
        created: DateTime<Utc>,
        updated: DateTime<Utc>,
        summary: String,
        history: Vec<HistoryEntry>,
        resolution_date: Option<DateTime<Utc>>,
        custom_fields: Vec<CustomField>,
    ) -> Self {
        let issue_type = String::from(ISSUE_TYPE);
        let resolution = if resolution_date.is_some() {
            Some(String::from(RESOLUTION_STATUS))
        } else {
            None
        };
        Self {
            status,
            reporter,
            issue_type,
            created,
            updated,
            summary,
            history,
            resolution,
            resolution_date,
            custom_fields,
        }
    }
}
