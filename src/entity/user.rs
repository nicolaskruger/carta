use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub master: Box<Option<User>>,
    pub name: String,
    pub id: Uuid,
}

impl User {
    pub fn new(master: Option<User>, name: String, id: Uuid) -> Self {
        Self {
            master: Box::new(master),
            name,
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
        User::new(None, "anakin".into(), Uuid::new_v4());
    }
}
