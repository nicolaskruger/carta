use uuid::Uuid;

use crate::entity::user::User;

pub trait IUserRepository {
    async fn create(user: User) -> User;
    async fn find(id: Uuid) -> User;
}

pub struct UserRepository {}

pub impl IUserRepository for UserRepository {
    async fn create(user: User) -> User {
        todo!()
    }

    async fn find(id: Uuid) -> User {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {}
}
