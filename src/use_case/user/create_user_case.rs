use uuid::Uuid;

use crate::{
    entity::user::User,
    repository::user_repository::{IUserRepository, UserRepositoryError},
};

pub struct CreateUserCase<UR: IUserRepository> {
    user_repository: UR,
}

pub struct CreateUser {
    pub name: String,
    pub master: Option<Uuid>,
}

pub struct CreateUserInput {
    user: CreateUser,
}

pub struct CreateUserOutput {}

pub enum CreateUserError {
    UserNotCreated,
}

impl<UR: IUserRepository> CreateUserCase<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }

    pub async fn exec(&self, input: CreateUserInput) -> Result<(), CreateUserError> {
        let create_user = input.user;

        let user = User::new(None, create_user.name, Uuid::new_v4());

        match self.user_repository.create(user).await {
            Ok(_) => Ok(()),
            Err(_) => Err(CreateUserError::UserNotCreated),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::{
        repository::user_repository::MockIUserRepository,
        use_case::user::create_user_case::{self, CreateUserCase},
    };

    #[test]
    fn new() {
        let mock = MockIUserRepository::new();

        let _ = CreateUserCase::new(mock);
    }
    #[tokio::test]
    async fn create_user_root() {
        let mut user_repository = MockIUserRepository::new();

        user_repository
            .expect_create()
            .once()
            .withf(|u| u.name == "anakin" && u.master.is_none() && u.id != Uuid::new_v4())
            .returning(Ok);

        let create_user = CreateUser {
            name: "anakin".into(),
            master: None,
        };

        let create_user_case = CreateUserCase::new(user_repository);

        let is_ok = create_user_case
            .exec(CreateUserInput { user: create_user })
            .await
            .is_ok();

        assert!(is_ok);
    }
}
