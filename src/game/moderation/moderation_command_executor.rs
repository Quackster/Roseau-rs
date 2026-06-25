use crate::game::moderation::{ModerationEffect, ModerationManager};
use crate::game::player::PlayerManager;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModerationCommandExecutor;

impl ModerationCommandExecutor {
    pub const ANSWER_CALL_FOR_HELP_PERMISSION: &'static str = "answer_call_for_help";

    pub fn call_for_help(
        player_manager: &PlayerManager,
        moderation_manager: &ModerationManager,
        main_server_port: i32,
        room_name: &str,
        from_name: &str,
        distress_message: &str,
        time: &str,
    ) -> Vec<ModerationEffect> {
        let moderator_connections = player_manager
            .main_server_players(main_server_port)
            .into_iter()
            .filter(|session| {
                player_manager.has_permission(
                    session.details().rank(),
                    Self::ANSWER_CALL_FOR_HELP_PERMISSION,
                )
            })
            .map(|session| session.connection_id());

        moderation_manager.call_for_help(
            moderator_connections,
            room_name,
            from_name,
            distress_message,
            time,
        )
    }
}
