use crate::gen::Generator;
use crate::gen::history_entry_gen::HistoryEntryGenerator;
use crate::gen::history_item_gen::HistoryItemGenerator;
use crate::gen::issue_gen::IssueGenerator;
use crate::gen::project_gen::ProjectGenerator;
use crate::model::export::Export;
use crate::model::user::User;

pub(crate) struct ExportGenerator<'l> {
    user: User,
    project_gen: &'l ProjectGenerator<'l>,
    issue_gen: &'l IssueGenerator<'l>,
    hist_entry_gen: &'l HistoryEntryGenerator<'l>,
    hist_item_gen: &'l HistoryItemGenerator,
}

impl<'l> ExportGenerator<'l> {
    pub fn new(
        user: User,
        project_gen: &'l ProjectGenerator<'l>,
        issue_gen: &'l IssueGenerator<'l>,
        hist_entry_gen: &'l HistoryEntryGenerator<'l>,
        hist_item_gen: &'l HistoryItemGenerator,
    ) -> Self {
        Self { user, project_gen, issue_gen, hist_entry_gen, hist_item_gen }
    }
}

impl<'l> Generator<Export> for ExportGenerator<'l> {
    fn generate(&self) -> Export {
        let project = self.project_gen.generate();
        Export::new(
            vec![self.user.clone()],
            vec![self.project_gen.generate()],
        )
    }
}
