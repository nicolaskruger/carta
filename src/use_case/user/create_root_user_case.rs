use uuid::Uuid;

use crate::{
    entity::user::User,
    repository::{auth_repository::IAuthRepository, user_repository::IUserRepository},
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

#[derive(Debug, PartialEq, Eq)]
pub enum CreateRootUserError {
    ExitsRoot,
    CreateUser,
    EncryptPassword,
}

impl<UR: IUserRepository, UA: IAuthRepository> CreateRootUserCase<UR, UA> {
    pub fn new(user_repository: UR, auth_repository: UA) -> Self {
        Self {
            user_repository,
            auth_repository,
        }
    }

    pub async fn exec(&self, input: CreateRootUserInput) -> Result<(), CreateRootUserError> {
        let exists_root = self
            .user_repository
            .exists_root()
            .await
            .map_err(|_| CreateRootUserError::ExitsRoot)?;

        if exists_root {
            let password = self
                .auth_repository
                .hash(&input.user.password)
                .await
                .map_err(|_| CreateRootUserError::EncryptPassword)?;

            let root_user = User::new(None, input.user.name, password, Uuid::new_v4());

            self.user_repository
                .create(root_user)
                .await
                .map_err(|_| CreateRootUserError::CreateUser)?;

            Ok(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::repository::{
        auth_repository::{AuthRepositoryErr, MockIAuthRepository},
        user_repository::{MockIUserRepository, UserRepositoryError},
    };

    #[test]
    fn new() {
        let mock = MockIUserRepository::new();
        let auth_repository = MockIAuthRepository::new();

        let _ = CreateRootUserCase::new(mock, auth_repository);
    }

    #[tokio::test]
    async fn should_not_create_an_root_user_when_error() {
        let mut user_repository = MockIUserRepository::new();
        let auth_repository = MockIAuthRepository::new();

        user_repository
            .expect_exists_root()
            .once()
            .returning(|| Err(UserRepositoryError::ExistRootSql));

        user_repository.expect_create().never();

        let create_root_usercase = CreateRootUserCase::new(user_repository, auth_repository);

        let input = CreateRootUserInput {
            user: CreateRootUser {
                name: "my name".into(),
                password: "my password".into(),
            },
        };

        let res = create_root_usercase.exec(input).await;

        assert!(res.is_err());

        if let Err(res) = res {
            assert_eq!(res, CreateRootUserError::ExitsRoot);
        }
    }

    #[tokio::test]
    async fn should_not_create_an_root_user_when_already_exist() {
        let mut user_repository = MockIUserRepository::new();
        let auth_repository = MockIAuthRepository::new();

        user_repository
            .expect_exists_root()
            .once()
            .returning(|| Ok(false));

        user_repository.expect_create().never();

        let create_root_usercase = CreateRootUserCase::new(user_repository, auth_repository);

        let input = CreateRootUserInput {
            user: CreateRootUser {
                name: "my name".into(),
                password: "my password".into(),
            },
        };

        let res = create_root_usercase.exec(input).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn should_not_create_an_root_user_when_created_err() {
        let mut user_repository = MockIUserRepository::new();
        let mut auth_repository = MockIAuthRepository::new();

        user_repository
            .expect_exists_root()
            .once()
            .returning(|| Ok(true));

        auth_repository
            .expect_hash()
            .once()
            .returning(|_| Ok("encrypt".to_string()));

        user_repository
            .expect_create()
            .once()
            .returning(|_| Err(UserRepositoryError::UserNotCreated));

        let create_root_usercase = CreateRootUserCase::new(user_repository, auth_repository);

        let input = CreateRootUserInput {
            user: CreateRootUser {
                name: "my name".into(),
                password: "my password".into(),
            },
        };

        let res = create_root_usercase.exec(input).await;

        assert!(res.is_err());

        if let Err(res) = res {
            assert_eq!(res, CreateRootUserError::CreateUser)
        }
    }

    #[tokio::test]
    async fn should_not_create_on_encrypt_password_err() {
        let mut user_repository = MockIUserRepository::new();
        let mut auth_repository = MockIAuthRepository::new();

        user_repository
            .expect_exists_root()
            .once()
            .returning(|| Ok(true));

        auth_repository
            .expect_hash()
            .once()
            .returning(|_| Err(AuthRepositoryErr::Hash));

        let create_root_usercase = CreateRootUserCase::new(user_repository, auth_repository);

        let input = CreateRootUserInput {
            user: CreateRootUser {
                name: "my name".into(),
                password: "my password".into(),
            },
        };

        let res = create_root_usercase.exec(input).await;

        assert!(res.is_err());

        if let Err(res) = res {
            assert_eq!(res, CreateRootUserError::EncryptPassword)
        }
    }

    #[tokio::test]
    async fn should_create() {
        let mut user_repository = MockIUserRepository::new();
        let mut auth_repository = MockIAuthRepository::new();

        user_repository
            .expect_exists_root()
            .once()
            .returning(|| Ok(true));

        auth_repository
            .expect_hash()
            .once()
            .returning(|_| Ok("encrypt".to_string()));

        user_repository.expect_create().once().returning(Ok);

        let create_root_usercase = CreateRootUserCase::new(user_repository, auth_repository);

        let input = CreateRootUserInput {
            user: CreateRootUser {
                name: "my name".into(),
                password: "my password".into(),
            },
        };

        let res = create_root_usercase.exec(input).await;

        assert!(res.is_ok());
    }
}
