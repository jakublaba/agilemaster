use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Item<'l> {
    field: &'l str,
    #[serde(rename = "fieldType")]
    field_type: &'l str,
    #[serde(rename = "fromString")]
    from_string: &'l str,
    from: &'l str,
    #[serde(rename = "toString")]
    to_string: &'l str,
    to: &'l str,
}

impl<'l> Item<'l> {
    pub fn new(
        field: &'l str,
        from: &'l str,
        to: &'l str,
    ) -> Self {
        Self {
            field,
            field_type: field,
            from_string: from,
            from,
            to_string: to,
            to,
        }
    }
}
