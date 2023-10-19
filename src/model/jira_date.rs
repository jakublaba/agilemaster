use chrono::{DateTime, Local};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum JiraDate<'l> {
    Absolute(DateTime<Local>),
    Relative(&'l str),
}

impl<'l> Serialize for JiraDate<'l> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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

impl<'l, 'de: 'l> Deserialize<'de> for JiraDate<'l> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let date_str: &str = Deserialize::deserialize(deserializer)?;
        if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
            Ok(Self::Absolute(date.with_timezone(&Local)))
        } else {
            Ok(Self::Relative(date_str.clone()))
        }
    }
}
