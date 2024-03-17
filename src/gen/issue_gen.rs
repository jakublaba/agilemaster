use crate::gen::date_gen::DateGenerator;
use crate::gen::Generator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::model::issue::Issue;

pub(crate) struct IssueGenerator<'l> {
    date_gen: &'l DateGenerator,
    history_entry_gen: &'l HistoryEntryGenerator<'l>,
}

impl<'l> IssueGenerator<'l> {
    pub fn new(date_gen: &'l DateGenerator, history_entry_gen: &'l HistoryEntryGenerator<'l>) -> Self {
        Self { date_gen, history_entry_gen }
    }
}

impl<'l> Generator<Issue<'l>> for IssueGenerator<'l> {
    fn next(&self) -> Issue<'l> {
        todo!()
    }
}
