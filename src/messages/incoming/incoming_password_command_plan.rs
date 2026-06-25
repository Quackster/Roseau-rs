use crate::game::player::PasswordAction;
use crate::messages::{IncomingCommand, IncomingExecutionEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct IncomingPasswordCommandPlan;

impl IncomingPasswordCommandPlan {
    pub(super) fn plan(command: &IncomingCommand) -> Option<IncomingExecutionEffect> {
        match command {
            IncomingCommand::Login {
                username,
                password,
                room_login,
            } => Some(IncomingExecutionEffect::Password(
                PasswordAction::verify_login(username, password, *room_login),
            )),
            IncomingCommand::RegisterPlayer {
                name,
                password,
                email,
                mission,
                figure,
                sex,
                birthday,
            } => Some(IncomingExecutionEffect::Password(
                PasswordAction::hash_registration(
                    name, password, email, mission, figure, sex, birthday,
                ),
            )),
            IncomingCommand::UpdateProfile {
                password,
                email,
                figure,
                mission,
                sex,
            } => Some(IncomingExecutionEffect::Password(
                PasswordAction::hash_profile_update(None, password, email, figure, mission, sex),
            )),
            _ => None,
        }
    }
}
