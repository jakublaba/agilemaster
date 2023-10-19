use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User<'l> {
    name: &'l str,
    groups: Vec<&'l str>,
    active: bool,
    email: &'l str,
    #[serde(rename = "fullname")]
    full_name: &'l str,
}

impl<'l> User<'l> {
    pub fn new(
        name: &'l str,
        groups: Vec<&'l str>,
        active: bool,
        email: &'l str,
        full_name: &'l str,
    ) -> Self {
        Self { name, groups, active, email, full_name }
    }
}

