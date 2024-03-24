use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;

use crate::gen::date_gen::DateGenerator;
use crate::gen::Generator;
use crate::model::history_entry::HistoryEntry;
use crate::model::history_item::HistoryItem;

pub(crate) struct HistoryEntryGenerator<'l> {
    rng: ThreadRng,
    author: &'l String,
    date_gen: &'l DateGenerator,
    statuses: &'l Vec<String>,
}

impl<'l> HistoryEntryGenerator<'l> {
    pub fn new(author: &'l String, date_gen: &'l DateGenerator, statuses: &'l Vec<String>) -> Self {
        let rng = thread_rng();
        Self { rng, author, date_gen, statuses }
    }

    fn rand_status(&mut self) -> &String {
        self.statuses.choose(&mut self.rng).unwrap()
    }
}

impl<'l> Generator<HistoryEntry> for HistoryEntryGenerator<'l> {
    fn generate(&mut self) -> HistoryEntry {
        let status = self.rand_status().clone();
        let created = self.date_gen.next();
        let mut items = Vec::<HistoryItem>::new();
        let mut i = 0;
        while self.statuses[i] != status {
            let from = self.statuses[i].clone();
            let to = self.statuses[i + 1].clone();
            let item = HistoryItem::new(from, to);
            items.push(item);
            i += 1;
        }

        HistoryEntry::new(
            self.author.clone(),
            created,
            items,
        )
    }
}
