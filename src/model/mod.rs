use chrono::{DateTime, Utc};
use serde::Serializer;

pub(crate) mod project;
pub(crate) mod issue;
pub(crate) mod user;
pub(crate) mod export;
pub(crate) mod history_entry;
pub(crate) mod history_item;
pub(crate) mod custom_field;
pub(crate) mod time_in_status;
pub(crate) mod auth_params;

pub(crate) fn serialize_date<S: Serializer>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    let str = date.to_rfc3339();
    serializer.serialize_str(&str)
}

pub(crate) fn serialize_date_opt<S: Serializer>(opt_date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error> {
    let str = match opt_date {
        Some(date) => date.to_rfc3339(),
        None => String::new(),
    };
    serializer.serialize_str(&str)
}
