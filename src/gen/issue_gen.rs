use rand::rngs::ThreadRng;

use crate::gen::date_gen::DateGenerator;
use crate::gen::Generator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::model::issue::Issue;

pub(crate) struct IssueGenerator<'l> {
    rng: ThreadRng,
    date_gen: &'l DateGenerator,
    history_entry_gen: &'l HistoryEntryGenerator<'l>,
    statuses: &'l Vec<String>,
}

impl<'l> IssueGenerator<'l> {
    pub fn new(
        date_gen: &'l DateGenerator,
        history_entry_gen: &'l HistoryEntryGenerator<'l>,
        statuses: &'l Vec<String>,
    ) -> Self {
        let rng = rand::thread_rng();
        Self { rng, date_gen, history_entry_gen, statuses }
    }
}

impl<'l> Generator<Issue> for IssueGenerator<'l> {
    fn generate(&mut self) -> Issue {
        todo!()
    }
}
