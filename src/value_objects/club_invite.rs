use uuid::Uuid;

pub struct ClubInvite {
    _id: Uuid,
}

impl ClubInvite {
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

        let club_invite = ClubInvite { _id: id };

        let _id = club_invite.id();

        assert_eq!(id.to_string(), _id.to_string());
    }

    #[test]
    fn set_id() {
        let id = Uuid::new_v4();

        let club_invite = ClubInvite { _id: id };

        let club_invite = club_invite.set_id(id);

        assert_eq!(id.to_string(), club_invite.id().to_string());
    }
}
