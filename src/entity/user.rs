use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub master: Box<Option<User>>,
    pub name: String,
    pub password: String,
    pub id: Uuid,
}

impl User {
    pub fn new(master: Option<User>, name: String, password: String, id: Uuid) -> Self {
        Self {
            master: Box::new(master),
            name,
            password,
            id,
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::entity::user::User;

    #[test]
    fn new() {
        User::new(None, "anakin".into(), "password".into(), Uuid::new_v4());
    }
}
