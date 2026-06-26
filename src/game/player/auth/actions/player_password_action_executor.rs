use crate::dao::{DaoError, PlayerDao};
use crate::game::player::{
    PasswordAction, PlayerDetails, PlayerLoginExecutor, PlayerLoginOutcome, PlayerLoginRequest,
    PlayerManager, PlayerPasswordActionOutcome, PlayerProfileUpdateExecutor,
    PlayerProfileUpdateOutcome, PlayerRegistrationExecutor, PlayerRegistrationOutcome,
    PlayerRegistrationRequest,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerPasswordActionExecutor;

impl PlayerPasswordActionExecutor {
    #[allow(clippy::too_many_arguments)]
    pub fn execute(
        player_dao: &dyn PlayerDao,
        player_manager: &PlayerManager,
        current: &PlayerDetails,
        action: &PasswordAction,
        connection_id: i32,
        server_port: i32,
        base_server_port: i32,
        default_credits: i32,
    ) -> Result<PlayerPasswordActionOutcome, DaoError> {
        match action {
            PasswordAction::VerifyLogin { .. } => Self::execute_login(
                player_dao,
                player_manager,
                action,
                connection_id,
                server_port,
                base_server_port,
            )
            .map(|outcome| PlayerPasswordActionOutcome::Login(outcome.expect("login action"))),
            PasswordAction::HashRegistration { .. } => {
                Self::execute_registration(player_dao, action, default_credits).map(|outcome| {
                    PlayerPasswordActionOutcome::Registration(outcome.expect("registration action"))
                })
            }
            PasswordAction::HashProfileUpdate { .. } => {
                Self::execute_profile_update(player_dao, current, action).map(|outcome| {
                    PlayerPasswordActionOutcome::ProfileUpdate(outcome.expect("profile action"))
                })
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute_all(
        player_dao: &dyn PlayerDao,
        player_manager: &PlayerManager,
        current: &PlayerDetails,
        actions: &[PasswordAction],
        connection_id: i32,
        server_port: i32,
        base_server_port: i32,
        default_credits: i32,
    ) -> Result<Vec<PlayerPasswordActionOutcome>, DaoError> {
        actions
            .iter()
            .map(|action| {
                Self::execute(
                    player_dao,
                    player_manager,
                    current,
                    action,
                    connection_id,
                    server_port,
                    base_server_port,
                    default_credits,
                )
            })
            .collect()
    }

    pub fn execute_login(
        player_dao: &dyn PlayerDao,
        player_manager: &PlayerManager,
        action: &PasswordAction,
        connection_id: i32,
        server_port: i32,
        base_server_port: i32,
    ) -> Result<Option<PlayerLoginOutcome>, DaoError> {
        let PasswordAction::VerifyLogin {
            username,
            password,
            room_login,
        } = action
        else {
            return Ok(None);
        };

        PlayerLoginExecutor::login(
            player_dao,
            player_manager,
            PlayerLoginRequest::new(
                username,
                password,
                *room_login,
                connection_id,
                server_port,
                base_server_port,
            ),
        )
        .map(Some)
    }

    pub fn execute_registration(
        player_dao: &dyn PlayerDao,
        action: &PasswordAction,
        default_credits: i32,
    ) -> Result<Option<PlayerRegistrationOutcome>, DaoError> {
        let PasswordAction::HashRegistration {
            username,
            password,
            email,
            mission,
            figure,
            sex,
            birthday,
        } = action
        else {
            return Ok(None);
        };

        PlayerRegistrationExecutor::register(
            player_dao,
            PlayerRegistrationRequest::new(
                username,
                password,
                email,
                mission,
                figure,
                sex,
                birthday,
                default_credits,
            ),
        )
        .map(Some)
    }

    pub fn execute_profile_update(
        player_dao: &dyn PlayerDao,
        current: &PlayerDetails,
        action: &PasswordAction,
    ) -> Result<Option<PlayerProfileUpdateOutcome>, DaoError> {
        let PasswordAction::HashProfileUpdate { .. } = action else {
            return Ok(None);
        };

        PlayerProfileUpdateExecutor::update_profile(player_dao, current, action).map(Some)
    }
}
