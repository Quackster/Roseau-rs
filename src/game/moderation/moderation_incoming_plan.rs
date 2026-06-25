use crate::game::moderation::{
    DistressMessage, ModerationCommandExecutor, ModerationEffect, ModerationManager,
};
use crate::game::player::PlayerManager;
use crate::game::room::settings::RoomType;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModerationIncomingPlan;

impl ModerationIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        player_manager: &PlayerManager,
        moderation_manager: &ModerationManager,
        main_server_port: i32,
        room: ModerationRoomContext<'_>,
        from_name: &str,
        time: &str,
    ) -> Vec<ModerationEffect> {
        let IncomingExecutionEffect::CryForHelp { message } = effect else {
            return Vec::new();
        };

        let first_tab_argument = message.split('\t').next().unwrap_or_default();
        let distress = DistressMessage::from_payload(
            room.room_type,
            room.name,
            room.id,
            message,
            first_tab_argument,
        );

        ModerationCommandExecutor::call_for_help(
            player_manager,
            moderation_manager,
            main_server_port,
            room.name,
            from_name,
            distress.text(),
            time,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModerationRoomContext<'a> {
    id: i32,
    name: &'a str,
    room_type: RoomType,
}

impl<'a> ModerationRoomContext<'a> {
    pub fn new(id: i32, name: &'a str, room_type: RoomType) -> Self {
        Self {
            id,
            name,
            room_type,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn room_type(&self) -> RoomType {
        self.room_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::moderation::{CallForHelp, ModerationManager};
    use crate::game::player::{Permission, PlayerDetails, PlayerSession};

    fn details(id: i32, username: &str, rank: i32) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_full(
            id, username, "mission", "figure", "", "email", rank, 0, "M", "GB", "", "", 0, "", 0,
        );
        details
    }

    fn player_manager() -> PlayerManager {
        let mut player_manager = PlayerManager::new(vec![Permission::new(
            ModerationCommandExecutor::ANSWER_CALL_FOR_HELP_PERMISSION,
            true,
            5,
        )]);
        player_manager.insert(PlayerSession::new(10, 30000, details(1, "mod", 5)));
        player_manager.insert(PlayerSession::new(11, 30000, details(2, "guest", 1)));
        player_manager.insert(PlayerSession::new(12, 30001, details(3, "other", 7)));
        player_manager
    }

    #[test]
    fn plans_private_room_call_for_help_from_raw_incoming_payload() {
        let effects = ModerationIncomingPlan::plan(
            &IncomingExecutionEffect::CryForHelp {
                message: "/Private Room: den;0;please;39\tden\tAlex".to_owned(),
            },
            &player_manager(),
            &ModerationManager::new(),
            30000,
            ModerationRoomContext::new(39, "den", RoomType::Private),
            "alice",
            "2026-06-25 12:00",
        );

        assert_eq!(
            effects,
            vec![ModerationEffect::SendCallForHelp {
                moderator_connection_id: 10,
                call: CallForHelp::new("den", "alice", "please", "2026-06-25 12:00"),
            }]
        );
    }

    #[test]
    fn plans_public_room_call_for_help_from_raw_incoming_payload() {
        let effects = ModerationIncomingPlan::plan(
            &IncomingExecutionEffect::CryForHelp {
                message: "/Habbo Lido;0;splash;ignored".to_owned(),
            },
            &player_manager(),
            &ModerationManager::new(),
            30000,
            ModerationRoomContext::new(0, "Habbo Lido", RoomType::Public),
            "alice",
            "now",
        );

        assert_eq!(
            effects,
            vec![ModerationEffect::SendCallForHelp {
                moderator_connection_id: 10,
                call: CallForHelp::new("Habbo Lido", "alice", "splash", "now"),
            }]
        );
    }

    #[test]
    fn ignores_unrelated_incoming_effects() {
        assert!(ModerationIncomingPlan::plan(
            &IncomingExecutionEffect::GoAway,
            &player_manager(),
            &ModerationManager::new(),
            30000,
            ModerationRoomContext::new(0, "Habbo Lido", RoomType::Public),
            "alice",
            "now",
        )
        .is_empty());
    }
}
