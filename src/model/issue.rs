use chrono::{DateTime, Local};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub struct Issue<'l> {
    status: &'l str,
    reporter: &'l str,
    #[serde(rename = "issueType")]
    issue_type: &'l str,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    created: DateTime<Local>,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    updated: DateTime<Local>,
    resolution: Option<&'l str>,
    summary: &'l str,
}

fn serialize_date<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let date_str = date.to_rfc3339(); // ISO 8601 date
    serializer.serialize_str(&date_str)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    let parsed_date = DateTime::parse_from_rfc3339(&date_str)
        .map_err(serde::de::Error::custom)?;
    Ok(parsed_date.with_timezone(&Local))
}

impl<'l> Issue<'l> {
    pub fn new(
        status: &'l str,
        reporter: &'l str,
        issue_type: &'l str,
        created: DateTime<Local>,
        updated: DateTime<Local>,
        resolution: Option<&'l str>,
        summary: &'l str,
    ) -> Self {
        Self { status, reporter, issue_type, created, updated, resolution, summary }
    }
}
