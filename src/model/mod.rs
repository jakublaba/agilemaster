use chrono::{DateTime, Utc};
use serde::Serializer;

pub(crate) mod project;
pub(crate) mod issue;
pub(crate) mod user;
pub(crate) mod export;
pub(crate) mod history_entry;
pub(crate) mod history_item;

pub(crate) fn serialize_date<S: Serializer>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    let str = date.to_rfc3339();
    serializer.serialize_str(&str)
}
