use serde::Serialize;

use crate::model::issue::Issue;

#[derive(Serialize)]
pub struct Project<'l> {
    name: &'l str,
    key: &'l str,
    issues: Vec<Issue<'l>>,
}

impl<'l> Project<'l> {
    pub fn new(
        name: &'l str,
        key: &'l str,
        issues: Vec<Issue<'l>>,
    ) -> Self {
        Self { name, key, issues }
    }
}
