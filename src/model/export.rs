use serde::Serialize;

use crate::model::project::Project;
use crate::model::user::User;

#[derive(Debug, Serialize)]
pub struct Export {
    #[serde(borrow)]
    users: Vec<User>,
    projects: Vec<Project>,
}

impl Export {
    pub fn new(
        users: Vec<User>,
        projects: Vec<Project>,
    ) -> Self {
        Self { users, projects }
    }
}
