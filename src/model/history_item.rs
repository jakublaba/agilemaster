use serde::Serialize;

const FIELD: &str = "status";

#[derive(Debug, Serialize)]
pub struct HistoryItem {
    field: String,
    #[serde(rename = "fieldType")]
    field_type: String,
    #[serde(rename = "fromString")]
    from_string: String,
    from: String,
    #[serde(rename = "toString")]
    to_string: String,
    to: String,
}

impl HistoryItem {
    pub fn new(
        from: String,
        to: String,
    ) -> Self {
        Self {
            field: String::from(FIELD),
            field_type: String::from(FIELD),
            from_string: from.clone(),
            from,
            to_string: to.clone(),
            to,
        }
    }
}
