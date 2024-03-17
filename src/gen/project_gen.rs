use crate::gen::Generator;
use crate::gen::issue_gen::IssueGenerator;
use crate::model::project::Project;

pub(crate) struct ProjectGenerator<'l> {
    issue_gen: &'l IssueGenerator<'l>,
}

impl ProjectGenerator {
    pub fn new(issue_gen: &IssueGenerator) -> Self {
        Self { issue_gen }
    }
}

impl Generator<Project> for ProjectGenerator {
    fn next(&self) -> Project {
        todo!()
    }
}
