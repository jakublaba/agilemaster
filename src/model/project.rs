use serde::{Deserialize, Serialize};

use crate::model::issue::Issue;

#[derive(Serialize, Deserialize)]
pub struct Project<'l> {
    name: &'l str,
    key: &'l str,
    issues: Vec<Issue<'l>>,
}
