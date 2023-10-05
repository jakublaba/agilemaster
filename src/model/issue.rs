use chrono::{DateTime, Local};
use serde::{Deserializer, Serialize, Serializer};

#[derive(Serialize)]
pub struct Issue<'l> {
    status: &'l str,
    reporter: &'l str,
    #[serde(rename = "issueType")]
    issue_type: &'l str,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    created: DateTime<Local>,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    updated: DateTime<Local>,
    summary: &'l str,
}

fn serialize_date<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let date_str = date.to_rfc3339(); // ISO 8601 date
    serializer.serialize_str(&date_str)
}

impl<'l> Issue<'l> {
    pub fn new(
        status: &'l str,
        reporter: &'l str,
        issue_type: &'l str,
        created: DateTime<Local>,
        updated: DateTime<Local>,
        summary: &'l str,
    ) -> Self {
        Self { status, reporter, issue_type, created, updated, summary }
    }
}
