use chrono::{DateTime, Utc};
use serde::Serializer;

pub mod project;
pub mod issue;
pub mod user;
pub mod export;
pub mod history_entry;
pub mod item;

pub(crate) fn serialize_date<S: Serializer>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    let str = date.to_rfc3339();
    serializer.serialize_str(&str)
}
