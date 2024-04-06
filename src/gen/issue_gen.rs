use rand::rngs::ThreadRng;

use crate::cli::cli::Cli;
use crate::gen::{AgileMasterError, Generator};
use crate::gen::date_gen::DateGenerator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::model::issue::Issue;

pub(crate) struct IssueGenerator<'l> {
    counter: i32,
    rng: ThreadRng,
    date_gen: DateGenerator,
    reporter: String,
    history_entry_gen: &'l mut HistoryEntryGenerator<'l>,
    statuses: &'l Vec<String>,
}

impl<'l> IssueGenerator<'l> {
    pub fn new(
        cli_args: &'l Cli,
        history_entry_gen: &'l mut HistoryEntryGenerator<'l>,
        statuses: &'l Vec<String>,
    ) -> Result<Self, AgileMasterError> {
        let counter = 1;
        let rng = rand::thread_rng();
        let date_gen = DateGenerator::new(cli_args).map_err(|_| AgileMasterError)?;
        let reporter = cli_args.author.clone();
        Ok(Self { counter, rng, date_gen, reporter, history_entry_gen, statuses })
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
        let issue = Issue::new(
            status,
            self.reporter.clone(),
            created,
            updated,
            format!("Summary {}", self.counter),
            history,
        );
        self.counter += 1;
        issue
    }
}
