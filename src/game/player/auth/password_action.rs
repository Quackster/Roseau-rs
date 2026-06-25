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
