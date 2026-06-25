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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryPlayerDao;
    use crate::dao::{CreatePlayer, PlayerDao};
    use crate::game::player::{PlayerEffect, PlayerSession};

    const MAIN_PORT: i32 = 30001;

    fn create_player(username: &str, password: &str) -> CreatePlayer {
        CreatePlayer::new(
            username,
            password,
            format!("{username}@example.test"),
            "hello",
            "hd=100",
            50,
            "F",
            "1990-01-01",
        )
    }

    #[test]
    fn executes_login_password_action_through_login_executor() {
        let dao = InMemoryPlayerDao::new();
        dao.create_player(&create_player("alice", "secret"))
            .unwrap();
        let existing = dao.details_by_username("alice").unwrap().unwrap();
        let existing_id = existing.id();
        let mut manager = PlayerManager::new(vec![]);
        manager.insert(PlayerSession::new(10, MAIN_PORT, existing));
        let action = PasswordAction::verify_login("alice", "secret", false);

        let outcome = PlayerPasswordActionExecutor::execute_login(
            &dao, &manager, &action, 11, MAIN_PORT, MAIN_PORT,
        )
        .unwrap()
        .unwrap();

        assert!(outcome.details().unwrap().is_authenticated());
        assert_eq!(
            outcome.effects(),
            &[
                PlayerEffect::CloseConnection { connection_id: 10 },
                PlayerEffect::UpdateLastLogin {
                    user_id: existing_id
                },
            ]
        );
    }

    #[test]
    fn executes_registration_password_action_through_registration_executor() {
        let dao = InMemoryPlayerDao::new();
        let action = PasswordAction::hash_registration(
            "bob",
            "door",
            "bob@example.test",
            "mission",
            "hd=200",
            "M",
            "1992-02-02",
        );

        let outcome =
            PlayerPasswordActionExecutor::execute_registration(&dao, &action, 100).unwrap();

        let details = dao.details_by_username("bob").unwrap().unwrap();
        assert_eq!(outcome, Some(PlayerRegistrationOutcome::Created));
        assert_eq!(details.password(), "door");
        assert_eq!(details.credits(), 100);
    }

    #[test]
    fn executes_profile_update_password_action_through_profile_executor() {
        let dao = InMemoryPlayerDao::new();
        dao.create_player(&create_player("alice", "secret"))
            .unwrap();
        let current = dao.details_by_username("alice").unwrap().unwrap();
        let action = PasswordAction::hash_profile_update(
            Some(current.id()),
            "changed",
            "new@example.test",
            "hd=300",
            "new mission",
            "M",
        );

        let outcome =
            PlayerPasswordActionExecutor::execute_profile_update(&dao, &current, &action).unwrap();

        let updated = dao.details_by_username("alice").unwrap().unwrap();
        assert!(matches!(
            outcome,
            Some(PlayerProfileUpdateOutcome::Updated(_))
        ));
        assert_eq!(updated.password(), "changed");
        assert_eq!(updated.email(), "new@example.test");
        assert_eq!(updated.figure(), "hd=300");
        assert_eq!(updated.mission(), "new mission");
        assert_eq!(updated.sex(), "M");
    }

    #[test]
    fn ignores_password_actions_for_other_execution_paths() {
        let dao = InMemoryPlayerDao::new();
        let manager = PlayerManager::new(vec![]);
        let current = crate::game::player::PlayerDetails::new();
        let login = PasswordAction::verify_login("alice", "secret", false);
        let register = PasswordAction::hash_registration(
            "bob",
            "door",
            "bob@example.test",
            "mission",
            "hd=200",
            "M",
            "1992-02-02",
        );

        assert_eq!(
            PlayerPasswordActionExecutor::execute_registration(&dao, &login, 100).unwrap(),
            None
        );
        assert_eq!(
            PlayerPasswordActionExecutor::execute_profile_update(&dao, &current, &login).unwrap(),
            None
        );
        assert!(PlayerPasswordActionExecutor::execute_login(
            &dao, &manager, &register, 1, MAIN_PORT, MAIN_PORT,
        )
        .unwrap()
        .is_none());

        let profile = PasswordAction::hash_profile_update(None, "x", "e", "f", "m", "F");
        assert!(
            PlayerPasswordActionExecutor::execute_profile_update(&dao, &current, &register)
                .unwrap()
                .is_none()
        );
        assert!(
            PlayerPasswordActionExecutor::execute_registration(&dao, &profile, 100)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn executes_password_actions_into_typed_outcome_stream() {
        let dao = InMemoryPlayerDao::new();
        dao.create_player(&create_player("alice", "secret"))
            .unwrap();
        let current = dao.details_by_username("alice").unwrap().unwrap();
        let manager = PlayerManager::new(vec![]);

        let outcomes = PlayerPasswordActionExecutor::execute_all(
            &dao,
            &manager,
            &current,
            &[
                PasswordAction::verify_login("alice", "secret", false),
                PasswordAction::hash_registration(
                    "bob",
                    "door",
                    "bob@example.test",
                    "mission",
                    "hd=200",
                    "M",
                    "1992-02-02",
                ),
                PasswordAction::hash_profile_update(
                    Some(current.id()),
                    "changed",
                    "new@example.test",
                    "hd=300",
                    "new mission",
                    "M",
                ),
            ],
            11,
            MAIN_PORT,
            MAIN_PORT,
            100,
        )
        .unwrap();

        assert_eq!(outcomes.len(), 3);
        assert!(outcomes[0].login().unwrap().details().is_some());
        assert_eq!(
            outcomes[1].registration(),
            Some(PlayerRegistrationOutcome::Created)
        );
        assert!(matches!(
            outcomes[2].profile_update(),
            Some(PlayerProfileUpdateOutcome::Updated(_))
        ));
    }
}
