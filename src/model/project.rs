use serde::Serialize;

use crate::model::issue::Issue;

const PROJECT_TYPE: String = String::from("software");
const TEMPLATE: String = String::from("com.pyxis.greenhopper.jira:gh-simplified-kanban-classic");

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
            name,
            external_name: name.clone(),
            key,
            project_type: PROJECT_TYPE,
            template: TEMPLATE,
            issues,
        }
    }
}
