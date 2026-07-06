use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::entity::user::User;

#[derive(Debug)]
pub enum UserRepositoryError {
    UserNotFound,
    UserNotCreated,
    ExistRootSql,
    ExistRootOption,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find(&self, id: Uuid) -> Result<User, UserRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserRepositoryError>;
    async fn exists_root(&self) -> Result<bool, UserRepositoryError>;
}

pub struct UserTable {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub master_id: Option<Uuid>,
}

pub struct PostgresqlUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresqlUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn _find(&self, id: Option<Uuid>) -> Option<User> {
        if let Some(id) = id {
            let user_row = sqlx::query_as!(UserTable, "SELECT * FROM users where id = $1", id)
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

    fn _master_id(&self, master: &Option<User>) -> Option<Uuid> {
        master.as_ref().map(|u| u.id)
    }
}

#[async_trait]
impl IUserRepository for PostgresqlUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        let _ = sqlx::query!(
            "INSERT INTO Users (id, name, password, master_id) VALUES ($1, $2, $3, $4);",
            user.id,
            user.name,
            user.password,
            self._master_id(&user.master),
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserRepositoryError::UserNotCreated)?;

        Ok(user)
    }

    async fn find(&self, id: Uuid) -> Result<User, UserRepositoryError> {
        let user_row = sqlx::query_as!(UserTable, "SELECT * FROM users where id = $1", id)
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

    async fn delete(&self, id: Uuid) -> Result<(), UserRepositoryError> {
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1;", id,)
            .execute(&self.pool)
            .await
            .map_err(|_| UserRepositoryError::UserNotCreated)?;

        Ok(())
    }

    async fn exists_root(&self) -> Result<bool, UserRepositoryError> {
        let exists: Option<bool> = sqlx::query_scalar!(
            "
                SELECT EXISTS (
                    SELECT 1
                    FROM users 
                    WHERE master_id IS NULL
                );
                "
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserRepositoryError::ExistRootSql)?;

        Ok(exists.ok_or(UserRepositoryError::ExistRootOption)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::infraestruture::config::{db_pool::db_pool, env::load_env};

    use super::*;

    #[tokio::test]
    #[ignore = "mutation test"]
    async fn basic_operation() {
        let env = load_env();

        assert!(env.is_ok());

        let env = env.unwrap();

        let pool = db_pool(&env.database_url).await;

        assert!(pool.is_ok());

        let repository = PostgresqlUserRepository::new(pool.unwrap());

        let user = User::new(
            None,
            "test_user".to_string(),
            "password_user".to_string(),
            Uuid::new_v4(),
        );

        let operation = repository.create(user.clone()).await;

        assert!(operation.is_ok());

        let fetch_user = repository.find(user.id).await;

        assert!(fetch_user.is_ok());

        let fetch_user = fetch_user.unwrap();

        assert_eq!(user.name, fetch_user.name);
        assert_eq!(user.password, fetch_user.password);
        assert_eq!(user.id, fetch_user.id);

        let exists_master = repository.exists_root().await;

        assert!(exists_master.is_ok());
        assert!(exists_master.unwrap());

        let deleted_result = repository.delete(fetch_user.id).await;

        assert!(deleted_result.is_ok());
    }
}
