use crate::gen::Generator;
use crate::gen::issue_gen::IssueGenerator;
use crate::model::project::Project;

pub(crate) struct ProjectGenerator<'l> {
    issue_gen: &'l IssueGenerator<'l>,
}

impl<'l> ProjectGenerator<'l> {
    pub fn new(issue_gen: &'l IssueGenerator<'l>) -> Self {
        Self { issue_gen }
    }
}

impl<'l> Generator<Project<'l>> for ProjectGenerator<'l> {
    fn next(&self) -> Project<'l> {
        todo!()
    }
}
