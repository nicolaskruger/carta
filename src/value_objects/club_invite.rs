use uuid::Uuid;

pub struct ClubInvite {
    _id: Uuid,
}

impl ClubInvite {
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
}
