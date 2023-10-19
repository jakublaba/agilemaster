use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::jira_date::JiraDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue<'l> {
    status: &'l str,
    reporter: &'l str,
    #[serde(rename = "issueType")]
    issue_type: &'l str,
    created: JiraDate<'l>,
    updated: JiraDate<'l>,
    summary: &'l str,
}

impl<'l> Issue<'l> {
    pub fn new(
        status: &'l str,
        reporter: &'l str,
        issue_type: &'l str,
        created: JiraDate<'l>,
        updated: JiraDate<'l>,
        summary: &'l str,
    ) -> Self {
        Self { status, reporter, issue_type, created, updated, summary }
    }
}
