use uuid::Uuid;

use crate::{
    entity::user::User,
    repository::{
        auth_repository::{AuthRepositoryErr, IAuthRepository},
        user_repository::IUserRepository,
    },
};

pub struct CreateUserCase<UR: IUserRepository, UA: IAuthRepository> {
    user_repository: UR,
    auth_repository: UA,
}

pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub master_id: Option<Uuid>,
}

pub struct CreateUserInput {
    user: CreateUser,
}

pub struct CreateUserOutput {}

pub enum CreateUserError {
    UserNotCreated,
    MasterNotFound,
    AuthError(AuthRepositoryErr),
}

impl<UR: IUserRepository, UA: IAuthRepository> CreateUserCase<UR, UA> {
    pub fn new(user_repository: UR, auth_repository: UA) -> Self {
        Self {
            user_repository,
            auth_repository,
        }
    }

    async fn fetch_master(
        &self,
        create_user: &CreateUser,
    ) -> Result<Option<User>, CreateUserError> {
        if let Some(master_id) = create_user.master_id {
            let master = self
                .user_repository
                .find(master_id)
                .await
                .map_err(|_| CreateUserError::MasterNotFound)?;
            Ok(Some(master))
        } else {
            Ok(None)
        }
    }

    pub async fn exec(&self, input: CreateUserInput) -> Result<(), CreateUserError> {
        let create_user = input.user;

        let master = self.fetch_master(&create_user).await?;

        let password = self
            .auth_repository
            .hash(&create_user.password)
            .await
            .map_err(CreateUserError::AuthError)?;

        let user = User::new(master, create_user.name, password, Uuid::new_v4());

        self.user_repository
            .create(user)
            .await
            .map_err(|_| CreateUserError::UserNotCreated)?;

        Ok(())
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

        let _ = CreateUserCase::new(mock, auth_repository);
    }

    #[tokio::test]
    async fn auth_error() {
        let mut user_repository = MockIUserRepository::new();

        let master_id = Uuid::new_v4();
        let master = User::new(None, "Obi".to_string(), "password".into(), master_id);

        user_repository
            .expect_find()
            .once()
            .withf(move |id| id.to_string() == master_id.clone().to_string())
            .returning(move |_| {
                let master = User::new(None, "Obi".to_string(), "password".into(), master_id);
                Ok(master)
            });

        let create_user = CreateUser {
            name: "anakin".into(),
            password: "password".into(),
            master_id: Some(master.id),
        };

        let mut auth_repository = MockIAuthRepository::new();

        auth_repository
            .expect_hash()
            .once()
            .withf(|password| password == "password")
            .returning(|_| Err(AuthRepositoryErr::Hash));

        let create_user_case = CreateUserCase::new(user_repository, auth_repository);

        let is_err = create_user_case
            .exec(CreateUserInput { user: create_user })
            .await
            .is_err();

        assert!(is_err);
    }

    #[tokio::test]
    async fn create_user() {
        let mut user_repository = MockIUserRepository::new();

        let master_id = Uuid::new_v4();
        let master = User::new(None, "Obi".to_string(), "password".into(), master_id);

        user_repository
            .expect_find()
            .once()
            .withf(move |id| id.to_string() == master_id.clone().to_string())
            .returning(move |_| {
                let master = User::new(None, "Obi".to_string(), "password".into(), master_id);
                Ok(master)
            });

        user_repository
            .expect_create()
            .once()
            .withf(|u| u.name == "anakin" && u.id != Uuid::new_v4())
            .returning(Ok);

        let create_user = CreateUser {
            name: "anakin".into(),
            password: "password".into(),
            master_id: Some(master.id),
        };

        let mut auth_repository = MockIAuthRepository::new();

        auth_repository
            .expect_hash()
            .once()
            .withf(|password| password == "password")
            .returning(|_| Ok("hashed_password".to_string()));

        let create_user_case = CreateUserCase::new(user_repository, auth_repository);

        let is_ok = create_user_case
            .exec(CreateUserInput { user: create_user })
            .await
            .is_ok();

        assert!(is_ok);
    }

    #[tokio::test]
    async fn do_not_create_user_with_master_not_found() {
        let mut user_repository = MockIUserRepository::new();

        let master_id = Uuid::new_v4();
        let master = User::new(None, "Obi".to_string(), "password".into(), master_id);

        user_repository
            .expect_find()
            .once()
            .withf(move |id| id.to_string() == master_id.clone().to_string())
            .returning(|_| Err(UserRepositoryError::UserNotFound));

        let create_user = CreateUser {
            name: "anakin".into(),
            password: "password".into(),
            master_id: Some(master.id),
        };

        let mut auth_repository = MockIAuthRepository::new();
        let create_user_case = CreateUserCase::new(user_repository, auth_repository);

        let is_err = create_user_case
            .exec(CreateUserInput { user: create_user })
            .await
            .is_err();

        assert!(is_err);
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
            password: "password".into(),
            master_id: None,
        };

        let mut auth_repository = MockIAuthRepository::new();

        auth_repository
            .expect_hash()
            .once()
            .withf(|password| password == "password")
            .returning(|_| Ok("hashed_password".to_string()));

        let create_user_case = CreateUserCase::new(user_repository, auth_repository);

        let is_ok = create_user_case
            .exec(CreateUserInput { user: create_user })
            .await
            .is_ok();

        assert!(is_ok);
    }
}
