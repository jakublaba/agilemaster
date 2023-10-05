use serde::{Deserialize, Serialize};

use crate::model::project::Project;
use crate::model::user::User;

#[derive(Serialize, Deserialize)]
pub struct Export<'l> {
    #[serde(borrow)]
    users: Vec<User<'l>>,
    projects: Vec<Project<'l>>,
}
