use serde::Serialize;

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
        field: String,
        from: String,
        to: String,
    ) -> Self {
        Self {
            field: field.clone(),
            field_type: field,
            from_string: from.clone(),
            from,
            to_string: to.clone(),
            to,
        }
    }
}
