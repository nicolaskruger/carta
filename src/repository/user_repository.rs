use uuid::Uuid;

use crate::entity::user::User;

pub trait IUserRepository {
    async fn create(&self, user: User) -> User;
    async fn find(&self, id: Uuid) -> User;
}

pub struct UserRepository {}

impl IUserRepository for UserRepository {
    async fn create(&self, user: User) -> User {
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
