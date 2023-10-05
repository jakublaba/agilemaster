use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User<'l> {
    name: &'l str,
    groups: Vec<&'l str>,
    active: bool,
    email: &'l str,
    #[serde(rename = "fullname")]
    full_name: &'l str,
}
