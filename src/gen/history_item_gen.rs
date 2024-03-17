use crate::gen::Generator;
use crate::model::history_item::HistoryItem;

pub(crate) struct HistoryItemGenerator<'l> {
    statuses: Vec<&'l str>,
}

impl<'l> HistoryItemGenerator<'l> {
    pub fn new(statuses: Vec<&'l str>) -> Self {
        Self { statuses }
    }
}

impl<'l> Generator<HistoryItem> for HistoryItemGenerator<'l> {
    fn next(&self) -> HistoryItem {
        todo!()
    }
}
