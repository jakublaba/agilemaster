use serde::Serialize;

use crate::model::issue::Issue;

#[derive(Debug, Serialize)]
pub struct Project<'l> {
    name: &'l str,
    #[serde(rename = "externalName")]
    external_name: &'l str,
    key: &'l str,
    #[serde(rename = "type")]
    project_type: &'l str,
    template: &'l str,
    issues: Vec<Issue<'l>>,
}

impl<'l> Project<'l> {
    pub fn new(
        name: &'l str,
        key: &'l str,
        issues: Vec<Issue<'l>>,
    ) -> Self {
        Self {
            name,
            external_name: name,
            key,
            project_type: "software",
            template: "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic",
            issues,
        }
    }
}
