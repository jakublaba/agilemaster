use crate::gen::Generator;
use crate::model::history_item::HistoryItem;

pub(crate) struct HistoryItemGenerator {
    statuses: Vec<String>,
}

impl HistoryItemGenerator {
    pub fn new(statuses: Vec<String>) -> Self {
        Self { statuses }
    }
}

impl Generator<HistoryItem> for HistoryItemGenerator {
    fn generate(&self) -> HistoryItem {
        todo!()
    }
}
