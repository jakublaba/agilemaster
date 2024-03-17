use crate::gen::Generator;
use crate::gen::issue_gen::IssueGenerator;
use crate::model::issue::Issue;
use crate::model::project::Project;

fn proj_key(name: &str) -> String {
    let words = name.split_whitespace().collect::<Vec<_>>();
    let mut key = words.iter()
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();
    key.retain(|c| c.is_ascii_alphanumeric());

    key
}

pub(crate) struct ProjectGenerator<'l> {
    name: &'l str,
    issue_amount: i32,
    issue_gen: &'l IssueGenerator<'l>,
}

impl<'l> ProjectGenerator<'l> {
    pub fn new(name: &'l str, issue_amount: i32, issue_gen: &'l IssueGenerator<'l>) -> Self {
        Self { name, issue_amount, issue_gen }
    }
}

impl<'l> Generator<Project<'l>> for ProjectGenerator<'l> {
    fn next(&self) -> Project<'l> {
        todo!();
        let mut issues = Vec::<Issue<'l>>::new();
        for _ in 0..self.issue_amount {
            issues.push(self.issue_gen.next());
        }
        Project::new(
            self.name,
            &proj_key(self.name),
            issues,
        )
    }
}
