use async_trait::async_trait;
use sqlx::{Pool, Postgres};
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
    async fn find(&self, id: Uuid) -> Result<User, UserRepositoryError>;
}

pub struct UserTable {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub master_id: Option<Uuid>,
}

pub struct PostgressUserRepository {
    pool: Pool<Postgres>,
}

impl PostgressUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn _find(&self, id: Option<Uuid>) -> Option<User> {
        if let Some(id) = id {
            let user_row = sqlx::query_as!(UserTable, "select * from users where id = $1", id)
                .fetch_one(&self.pool)
                .await
                .ok()?;

            Some(User::new(
                None,
                user_row.name,
                user_row.password,
                user_row.id,
            ))
        } else {
            None
        }
    }
}

#[async_trait]
impl IUserRepository for PostgressUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        todo!()
    }

    async fn find(&self, id: Uuid) -> Result<User, UserRepositoryError> {
        let user_row = sqlx::query_as!(UserTable, "select * from users where id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| UserRepositoryError::UserNotFound)?;

        Ok(User::new(
            self._find(user_row.master_id).await,
            user_row.name,
            user_row.password,
            user_row.id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {}
}
