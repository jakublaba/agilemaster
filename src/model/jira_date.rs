use chrono::{DateTime, Local};
use serde::{Serialize, Serializer};

pub enum JiraDate<'l> {
    Absolute(DateTime<Local>),
    Relative(&'l str),
}

impl<'l> Serialize for JiraDate<'l> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            JiraDate::Absolute(date) => {
                let json = date.to_rfc3339();
                serializer.serialize_str(&json)
            }
            JiraDate::Relative(date) => {
                serializer.serialize_str(date)
            }
        }
    }
}
