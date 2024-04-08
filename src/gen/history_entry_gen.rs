use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;

use crate::cli::cli::Cli;
use crate::gen::date_gen::DateGenerator;
use crate::gen::{AgileMasterError, Generator};
use crate::model::history_entry::HistoryEntry;
use crate::model::history_item::HistoryItem;

pub(crate) struct HistoryEntryGenerator<'l> {
    rng: ThreadRng,
    author: &'l String,
    date_gen: DateGenerator,
    statuses: &'l Vec<String>,
}

impl<'l> HistoryEntryGenerator<'l> {
    pub fn new(cli_args: &'l Cli, statuses: &'l Vec<String>) -> Result<Self, AgileMasterError> {
        let rng = thread_rng();
        let author = &cli_args.author.name;
        let date_gen = DateGenerator::new(cli_args).map_err(|_| AgileMasterError)?;
        dbg!(&date_gen);
        Ok(Self {
            rng,
            author,
            date_gen,
            statuses,
        })
    }

    fn rand_status(&mut self) -> &String {
        self.statuses.choose(&mut self.rng).unwrap()
    }
}

impl<'l> Generator<(String, Vec<HistoryEntry>)> for HistoryEntryGenerator<'l> {
    fn generate(&mut self) -> (String, Vec<HistoryEntry>) {
        let status = self.rand_status().clone();
        let mut entries = Vec::<HistoryEntry>::new();
        let mut created = self.date_gen.next();
        let mut i = 0;
        while self.statuses[i] != status {
            let from = self.statuses[i].clone();
            let to = self.statuses[i + 1].clone();
            let items = vec![HistoryItem::new(from, to)];
            let entry = HistoryEntry::new(self.author.clone(), created.clone(), items);
            entries.push(entry);
            i += 1;
            created = self.date_gen.next_after(created).unwrap();
        }

        (status, entries)
    }
}
