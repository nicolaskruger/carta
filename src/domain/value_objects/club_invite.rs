use uuid::Uuid;

pub struct ClubInvite {
    _id: Uuid,
}

impl ClubInvite {
    pub fn build() -> Self {
        Self {
            _id: Uuid::new_v4(),
        }
    }

    pub fn new(id: Uuid) -> Self {
        Self { _id: id }
    }

    pub fn set_id(mut self, id: Uuid) -> ClubInvite {
        self._id = id;
        self
    }
    pub fn id(self) -> Uuid {
        self._id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        let id = Uuid::new_v4();

        let club_invite = ClubInvite::new(id);

        let _id = club_invite.id();

        assert_eq!(id.to_string(), _id.to_string());
    }

    #[test]
    fn set_id() {
        let id = Uuid::new_v4();

        let club_invite = ClubInvite::new(id);
        let club_invite = club_invite.set_id(id);

        assert_eq!(id.to_string(), club_invite.id().to_string());
    }

    #[test]
    fn build() {
        let id = Uuid::new_v4();

        let club_invite = ClubInvite::build();

        assert_ne!(id.to_string(), club_invite.id().to_string());
    }
}
