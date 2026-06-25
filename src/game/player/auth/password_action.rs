use crate::game::player::PlayerDetails;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordAction {
    VerifyLogin {
        username: String,
        password: String,
        room_login: bool,
    },
    HashRegistration {
        username: String,
        password: String,
        email: String,
        mission: String,
        figure: String,
        sex: String,
        birthday: String,
    },
    HashProfileUpdate {
        user_id: Option<i32>,
        password: String,
        email: String,
        figure: String,
        mission: String,
        sex: String,
    },
}

impl PasswordAction {
    pub fn verify_login(
        username: impl Into<String>,
        password: impl Into<String>,
        room_login: bool,
    ) -> Self {
        Self::VerifyLogin {
            username: username.into(),
            password: password.into(),
            room_login,
        }
    }

    pub fn hash_registration(
        username: impl Into<String>,
        password: impl Into<String>,
        email: impl Into<String>,
        mission: impl Into<String>,
        figure: impl Into<String>,
        sex: impl Into<String>,
        birthday: impl Into<String>,
    ) -> Self {
        Self::HashRegistration {
            username: username.into(),
            password: password.into(),
            email: email.into(),
            mission: mission.into(),
            figure: figure.into(),
            sex: sex.into(),
            birthday: birthday.into(),
        }
    }

    pub fn hash_profile_update(
        user_id: Option<i32>,
        password: impl Into<String>,
        email: impl Into<String>,
        figure: impl Into<String>,
        mission: impl Into<String>,
        sex: impl Into<String>,
    ) -> Self {
        Self::HashProfileUpdate {
            user_id,
            password: password.into(),
            email: email.into(),
            figure: figure.into(),
            mission: mission.into(),
            sex: sex.into(),
        }
    }

    pub fn password(&self) -> &str {
        match self {
            Self::VerifyLogin { password, .. }
            | Self::HashRegistration { password, .. }
            | Self::HashProfileUpdate { password, .. } => password,
        }
    }

    pub fn updated_profile_details(&self, current: &PlayerDetails) -> Option<PlayerDetails> {
        let Self::HashProfileUpdate {
            user_id,
            password,
            email,
            figure,
            mission,
            sex,
        } = self
        else {
            return None;
        };

        let mut details = current.clone();
        if let Some(user_id) = user_id {
            if details.id() != *user_id {
                return None;
            }
        }

        if details.sex() != sex {
            details.set_pool_figure("");
        }

        details.set_password(password);
        details.set_email(email);
        details.set_figure(figure);
        details.set_mission(mission);
        details.set_sex(sex);
        Some(details)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_login_password_operation() {
        let action = PasswordAction::verify_login("alice", "secret", true);

        assert_eq!(
            action,
            PasswordAction::VerifyLogin {
                username: "alice".to_owned(),
                password: "secret".to_owned(),
                room_login: true,
            }
        );
        assert_eq!(action.password(), "secret");
    }

    #[test]
    fn records_hashing_operations() {
        assert_eq!(
            PasswordAction::hash_registration(
                "bob",
                "door",
                "bob@example.test",
                "hello",
                "hd-100",
                "M",
                "1990-01-01",
            ),
            PasswordAction::HashRegistration {
                username: "bob".to_owned(),
                password: "door".to_owned(),
                email: "bob@example.test".to_owned(),
                mission: "hello".to_owned(),
                figure: "hd-100".to_owned(),
                sex: "M".to_owned(),
                birthday: "1990-01-01".to_owned(),
            }
        );
        assert_eq!(
            PasswordAction::hash_profile_update(
                Some(7),
                "changed",
                "alice@example.test",
                "hd-200",
                "new mission",
                "F",
            ),
            PasswordAction::HashProfileUpdate {
                user_id: Some(7),
                password: "changed".to_owned(),
                email: "alice@example.test".to_owned(),
                figure: "hd-200".to_owned(),
                mission: "new mission".to_owned(),
                sex: "F".to_owned(),
            }
        );
    }

    #[test]
    fn applies_profile_update_to_existing_details_and_clears_pool_figure_on_sex_change() {
        let mut details = PlayerDetails::new();
        details.fill_full(
            7,
            "alice",
            "old mission",
            "hd-100",
            "pool",
            "old@example.test",
            1,
            50,
            "F",
            "UK",
            "",
            "1990-01-01",
            1234,
            "hello",
            2,
        );
        let action = PasswordAction::hash_profile_update(
            Some(7),
            "changed",
            "alice@example.test",
            "hd-200",
            "new mission",
            "M",
        );

        let updated = action.updated_profile_details(&details).unwrap();

        assert_eq!(updated.password(), "changed");
        assert_eq!(updated.email(), "alice@example.test");
        assert_eq!(updated.figure(), "hd-200");
        assert_eq!(updated.mission(), "new mission");
        assert_eq!(updated.sex(), "M");
        assert_eq!(updated.pool_figure(), "");
    }

    #[test]
    fn refuses_profile_update_for_mismatched_user_id() {
        let mut details = PlayerDetails::new();
        details.fill_basic(8, "alice", "mission", "figure");
        let action = PasswordAction::hash_profile_update(
            Some(7),
            "changed",
            "alice@example.test",
            "hd-200",
            "new mission",
            "F",
        );

        assert_eq!(action.updated_profile_details(&details), None);
    }
}
