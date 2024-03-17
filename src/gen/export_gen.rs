use chrono::{DateTime, Utc};

use crate::gen::date_gen::DateGenerator;
use crate::gen::Generator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::gen::history_item_gen::HistoryItemGenerator;
use crate::gen::issue_gen::IssueGenerator;
use crate::gen::project_gen::ProjectGenerator;
use crate::model::export::Export;
use crate::model::user::User;

pub(crate) struct ExportGenerator<'l> {
    user: User<'l>,
    project_gen: &'l ProjectGenerator<'l>,
}

impl<'l> ExportGenerator<'l> {
    pub fn new(
        user: User,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        statuses: Vec<&str>,
    ) -> Self {
        let date_gen = &DateGenerator::new(start_date, end_date);
        let hist_item_gen = &HistoryItemGenerator::new(statuses);
        let hist_entry_gen = &HistoryEntryGenerator::new(date_gen, hist_item_gen);
        let issue_gen = &IssueGenerator::new(date_gen, hist_entry_gen);
        let project_gen = &ProjectGenerator::new(issue_gen);
        Self { user, project_gen }
    }
}

impl Generator<Export> for ExportGenerator {
    fn next(&self) -> Export {
        Export::new(
            vec![self.user],
            vec![self.project_gen.next()],
        )
    }
}
