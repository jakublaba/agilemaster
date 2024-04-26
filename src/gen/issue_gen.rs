use std::collections::HashMap;

use crate::cli::cli::Cli;
use crate::gen::{AgileMasterError, Generator};
use crate::gen::date_gen::DateGenerator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::model::custom_field::CustomField;
use crate::model::issue::{Issue, RESOLUTION_STATUS};
use crate::model::jira_rest::request_statuses;

pub(crate) struct IssueGenerator<'l> {
    counter: i32,
    date_gen: DateGenerator,
    reporter: String,
    status_ids: HashMap<String, i32>,
    history_entry_gen: &'l mut HistoryEntryGenerator<'l>,
}

impl<'l> IssueGenerator<'l> {
    pub async fn new(
        cli_args: &'l Cli,
        history_entry_gen: &'l mut HistoryEntryGenerator<'l>,
    ) -> Result<Self, AgileMasterError> {
        let counter = 1;
        let date_gen = DateGenerator::new(cli_args).map_err(|_| AgileMasterError)?;
        let reporter = cli_args.author.name.clone();
        let status_ids = request_statuses(&cli_args.auth_params, &cli_args.statuses)
            .await
            .map_err(|_| AgileMasterError)?;
        Ok(Self {
            counter,
            date_gen,
            reporter,
            status_ids,
            history_entry_gen,
        })
    }
}

impl<'l> Generator<Issue> for IssueGenerator<'l> {
    fn generate(&mut self) -> Issue {
        let (status, history) = self.history_entry_gen.generate();
        let created = if let Some(entry) = history.first() {
            entry.created
        } else {
            self.date_gen.next()
        };
        let updated = if let Some(entry) = history.last() {
            entry.created
        } else {
            self.date_gen.next_after(created).unwrap()
        };
        let resolution_date = if status == RESOLUTION_STATUS {
            Some(updated)
        } else {
            None
        };
        let custom_fields = CustomField::time_in_status(&self.status_ids, &history);
        let issue = Issue::new(
            status,
            self.reporter.clone(),
            created,
            updated,
            format!("Summary {}", self.counter),
            history,
            resolution_date,
            custom_fields,
        );
        self.counter += 1;
        issue
    }
}
