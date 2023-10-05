use serde::Serialize;

use crate::model::project::Project;
use crate::model::user::User;

#[derive(Serialize)]
pub struct Export<'l> {
    #[serde(borrow)]
    users: Vec<User<'l>>,
    projects: Vec<Project<'l>>,
}

impl<'l> Export<'l> {
    pub fn new(
        users: Vec<User<'l>>,
        projects: Vec<Project<'l>>,
    ) -> Self {
        Self { users, projects }
    }
}
