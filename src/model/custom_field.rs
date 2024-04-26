use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CustomField {
    #[serde(rename = "fieldName")]
    field_name: String,
    #[serde(rename = "fieldType")]
    field_type: String,
    value: String,
}

impl CustomField {
    pub fn from_time_in_status() {}
}
