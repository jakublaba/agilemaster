use serde::Serialize;

#[derive(Serialize)]
pub struct Item<'l> {
    #[serde(rename = "fieldType")]
    field_type: &'l str,
    field: &'l str,
    from: &'l str,
    #[serde(rename = "fromString")]
    from_string: &'l str,
    to: &'l str,
    #[serde(rename = "toString")]
    to_string: &'l str,
}

impl<'l> Item<'l> {
    pub fn new(
        field_type: &'l str,
        field: &'l str,
        from: &'l str,
        from_string: &'l str,
        to: &'l str,
        to_string: &'l str,
    ) -> Self {
        Self { field_type, field, from, from_string, to, to_string }
    }
}
