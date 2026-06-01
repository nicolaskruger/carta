use crate::{entity::user::User, repository::user_repository::IUserRepository};

pub struct CreateUserCase {}

pub struct CreateUserInput {
    user: User,
    user_repository: Box<dyn IUserRepository>,
}

pub struct CreateUserOutput {}

impl CreateUserCase {
    async fn exec(input: CreateUserInput) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create_user_root() {}
}
