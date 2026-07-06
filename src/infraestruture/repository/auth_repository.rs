use async_trait::async_trait;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc},
};

#[derive(Debug)]
pub enum AuthRepositoryErr {
    Hash,
    Verify,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IAuthRepository {
    async fn verify(&self, password: &str, hash_password: &str) -> Result<bool, AuthRepositoryErr>;
    async fn hash(&self, password: &str) -> Result<String, AuthRepositoryErr>;
}

pub struct AragornAuthRepository {}

#[async_trait]
impl IAuthRepository for AragornAuthRepository {
    async fn verify(&self, password: &str, hash_password: &str) -> Result<bool, AuthRepositoryErr> {
        let password_hash =
            phc::PasswordHash::new(hash_password).map_err(|_| AuthRepositoryErr::Hash)?;
        Argon2::default()
            .verify_password(password.as_ref(), &password_hash)
            .map_err(|_| AuthRepositoryErr::Verify)?;
        Ok(true)
    }
    async fn hash(&self, password: &str) -> Result<String, AuthRepositoryErr> {
        let hash = Argon2::default()
            .hash_password(password.as_bytes())
            .map_err(|_| AuthRepositoryErr::Hash)?
            .to_string();
        Ok(hash)
    }
}

#[cfg(test)]
mod test {

    use crate::infraestruture::repository::auth_repository::{
        AragornAuthRepository, AuthRepositoryErr, IAuthRepository,
    };

    #[derive(Debug)]
    pub struct GenericaAuthRepositoryTest<A: IAuthRepository> {
        auth_repository: A,
    }

    impl<A: IAuthRepository> GenericaAuthRepositoryTest<A> {
        pub fn new(auth_repository: A) -> Self {
            Self { auth_repository }
        }

        pub async fn test(&self) -> Result<(), AuthRepositoryErr> {
            let p_a = "password";
            let p_b = self.auth_repository.hash("password").await?;

            assert!(self.auth_repository.verify(p_a, &p_b).await?);

            Ok(())
        }
    }

    #[tokio::test]
    async fn aragorn() -> Result<(), AuthRepositoryErr> {
        let test = GenericaAuthRepositoryTest::new(AragornAuthRepository {});
        test.test().await?;
        Ok(())
    }
}
