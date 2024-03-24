use serde::Serialize;

use crate::model::issue::Issue;

const PROJECT_TYPE: &str = "software";
const TEMPLATE: &str = "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic";

#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    #[serde(rename = "externalName")]
    external_name: String,
    key: String,
    #[serde(rename = "type")]
    project_type: String,
    template: String,
    issues: Vec<Issue>,
}

impl Project {
    pub fn new(
        name: String,
        key: String,
        issues: Vec<Issue>,
    ) -> Self {
        Self {
            name: name.clone(),
            external_name: name,
            key,
            project_type: String::from(PROJECT_TYPE),
            template: String::from(TEMPLATE),
            issues,
        }
    }
}
