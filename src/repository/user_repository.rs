use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::user::User;

pub enum UserRepositoryError {
    UserNotFound,
    UserNotCreated,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find(&self, id: Uuid) -> User;
}

pub struct UserRepository {}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        todo!()
    }

    async fn find(&self, id: Uuid) -> User {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {}
}
