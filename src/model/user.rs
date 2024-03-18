use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    name: String,
    groups: Vec<String>,
    active: bool,
    email: String,
    #[serde(rename = "fullname")]
    full_name: String,
}

impl User {
    pub fn new(
        name: String,
        groups: Vec<String>,
        active: bool,
        email: String,
        full_name: String,
    ) -> Self {
        Self { name, groups, active, email, full_name }
    }
}
