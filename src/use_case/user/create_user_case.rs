use crate::entity::user::User;

pub struct CreateUserCase {}

pub struct CreateUserInput {
    user: User,
}

pub struct CreateUserOutput {}

impl CreateUserCase {
    async fn exec(input: CreateUserInput) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create_user_root() {}
}
