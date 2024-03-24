use crate::gen::Generator;
use crate::gen::project_gen::ProjectGenerator;
use crate::model::export::Export;
use crate::model::user::User;

pub(crate) struct ExportGenerator<'l> {
    user: User,
    project_gen: &'l mut ProjectGenerator<'l>,
}

impl<'l> ExportGenerator<'l> {
    pub fn new(
        user: User,
        project_gen: &'l mut ProjectGenerator<'l>,
    ) -> Self {
        Self { user, project_gen }
    }
}

impl<'l> Generator<Export> for ExportGenerator<'l> {
    fn generate(&mut self) -> Export {
        Export::new(
            vec![self.user.clone()],
            vec![self.project_gen.generate()],
        )
    }
}
