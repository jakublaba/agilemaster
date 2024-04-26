use std::collections::HashMap;

use serde::Serialize;

use crate::model::history_entry::HistoryEntry;

const TIME_IN_STATUS_NAME: &str = "[CHART] Time in Status";
const TIME_IN_STATUS_TYPE: &str = "com.atlassian.jira.ext.charting:timeinstatus";
pub(crate) const RECORD_SEP: &str = "_*|*_";
pub(crate) const DATA_SEP: &str = "_*:*_";

#[derive(Debug, Serialize)]
pub struct CustomField {
    #[serde(rename = "fieldName")]
    field_name: String,
    #[serde(rename = "fieldType")]
    field_type: String,
    value: String,
}

impl CustomField {
    pub fn time_in_status(
        status_ids: &HashMap<String, i32>,
        history: &Vec<HistoryEntry>,
    ) -> Self {
        let mut value = String::new();
        let time_spent = calculate_time_spent(&history);
        for (status, millis) in time_spent {
            let id = status_ids.get(&status).unwrap();
            value.push_str(&format!("{id}{DATA_SEP}1{DATA_SEP}{millis}{RECORD_SEP}"));
        }
        // get rid of trailing data record separator
        value.truncate(value.len() - RECORD_SEP.len());

        let field_name = String::from(TIME_IN_STATUS_NAME);
        let field_type = String::from(TIME_IN_STATUS_TYPE);
        Self { field_name, field_type, value }
    }
}

// TODO this has skill issue
fn calculate_time_spent(history: &Vec<HistoryEntry>) -> HashMap<String, i64> {
    let mut time_spent = HashMap::<String, i64>::new();
    if history.len() < 2 {
        return time_spent;
    }

    for w in history.windows(2) {
        let current = &w[0];
        let next = &w[1];
        let status = &current.items[0].to;
        let duration = (next.created - current.created).num_milliseconds();
        let entry = time_spent.entry(status.clone()).or_insert(0);
        *entry += duration;
    }

    time_spent
}
