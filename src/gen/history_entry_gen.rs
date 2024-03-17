use crate::gen::date_gen::DateGenerator;
use crate::gen::Generator;
use crate::gen::history_item_gen::HistoryItemGenerator;
use crate::model::history_entry::HistoryEntry;

pub(crate) struct HistoryEntryGenerator<'l> {
    date_gen: &'l DateGenerator,
    history_item_gen: &'l HistoryItemGenerator<'l>,
}

impl<'l> HistoryEntryGenerator<'l> {
    pub fn new(date_gen: &'l DateGenerator, history_item_gen: &'l HistoryItemGenerator<'l>) -> Self {
        Self { date_gen, history_item_gen }
    }
}

impl<'l> Generator<HistoryEntry<'l>> for HistoryEntryGenerator<'l> {
    fn next(&self) -> HistoryEntry<'l> {
        todo!()
    }
}
