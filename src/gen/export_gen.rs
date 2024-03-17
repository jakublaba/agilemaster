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
    issue_gen: &'l IssueGenerator<'l>,
    hist_entry_gen: &'l HistoryEntryGenerator<'l>,
    hist_item_gen: &'l HistoryItemGenerator<'l>,
}

impl<'l> ExportGenerator<'l> {
    pub fn new(
        user: User<'l>,
        project_gen: &'l ProjectGenerator<'l>,
        issue_gen: &'l IssueGenerator<'l>,
        hist_entry_gen: &'l HistoryEntryGenerator<'l>,
        hist_item_gen: &'l HistoryItemGenerator<'l>,
    ) -> Self {
        Self { user, project_gen, issue_gen, hist_entry_gen, hist_item_gen }
    }
}

impl<'l> Generator<Export<'l>> for ExportGenerator<'l> {
    fn next(&self) -> Export<'l> {
        Export::new(
            vec![self.user.clone()],
            vec![self.project_gen.next()],
        )
    }
}
