use serde::Serialize;

use crate::model::history_entry::HistoryEntry;
use crate::model::jira_date::JiraDate;

#[derive(Serialize)]
pub struct Issue<'l> {
    status: &'l str,
    reporter: &'l str,
    #[serde(rename = "issueType")]
    issue_type: &'l str,
    created: JiraDate<'l>,
    updated: JiraDate<'l>,
    summary: &'l str,
    history: Vec<HistoryEntry<'l>>,
}

impl<'l> Issue<'l> {
    pub fn new(
        status: &'l str,
        reporter: &'l str,
        issue_type: &'l str,
        created: JiraDate<'l>,
        updated: JiraDate<'l>,
        summary: &'l str,
        history: Vec<HistoryEntry<'l>>,
    ) -> Self {
        Self { status, reporter, issue_type, created, updated, summary, history }
    }
}
