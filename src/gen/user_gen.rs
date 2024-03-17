use crate::gen::Generator;
use crate::model::user::User;

pub(crate) struct UserGenerator {}

impl UserGenerator {}

impl Generator<User> for UserGenerator {
    fn next(&self) -> User {
        todo!()
    }
}
