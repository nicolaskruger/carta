use uuid::Uuid;

use crate::{
    entity::user::User,
    repository::{
        auth_repository::{AuthRepositoryErr, IAuthRepository},
        user_repository::IUserRepository,
    },
};

pub struct CreateRootUserCase<UR: IUserRepository, UA: IAuthRepository> {
    user_repository: UR,
    auth_repository: UA,
}

pub struct CreateRootUser {
    pub name: String,
    pub password: String,
}

pub struct CreateRootUserInput {
    user: CreateRootUser,
}

pub struct CreateRootUserOutput {}

pub enum CreateUserError {
    HasRootUser,
    MasterNotFound,
    AuthError(AuthRepositoryErr),
}

impl<UR: IUserRepository, UA: IAuthRepository> CreateRootUserCase<UR, UA> {
    pub fn new(user_repository: UR, auth_repository: UA) -> Self {
        Self {
            user_repository,
            auth_repository,
        }
    }

    pub async fn exec(&self, input: CreateRootUserInput) -> Result<(), CreateUserError> {
        todo!();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::{
        repository::{
            auth_repository::MockIAuthRepository,
            user_repository::{MockIUserRepository, UserRepositoryError},
        },
        use_case::user::create_user_case::CreateUserCase,
    };

    #[test]
    fn new() {
        let mock = MockIUserRepository::new();
        let auth_repository = MockIAuthRepository::new();

        let _ = CreateRootUserCase::new(mock, auth_repository);
    }
}
