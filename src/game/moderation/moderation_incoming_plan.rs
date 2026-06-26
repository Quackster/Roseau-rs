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
#[path = "moderation_incoming_plan_tests.rs"]
mod tests;
