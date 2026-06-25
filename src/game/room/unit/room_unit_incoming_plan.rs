use crate::game::player::PlayerManager;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomManager, RoomSummary, RoomUnitOutcome};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUnitIncomingPlan;

impl RoomUnitIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        base_port: i32,
    ) -> Vec<RoomUnitOutcome> {
        match effect {
            IncomingExecutionEffect::InitUnitListener => {
                vec![RoomUnitOutcome::listener(public_rooms(room_manager))]
            }
            IncomingExecutionEffect::GetUnitUsers { room_name } => {
                vec![unit_members(
                    room_manager,
                    player_manager,
                    room_name,
                    base_port,
                )]
            }
            _ => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        base_port: i32,
    ) -> Vec<RoomUnitOutcome> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room_manager, player_manager, base_port))
            .collect()
    }
}

fn public_rooms(room_manager: &RoomManager) -> Vec<RoomSummary> {
    room_manager
        .get_public_rooms()
        .into_iter()
        .cloned()
        .collect()
}

fn unit_members(
    room_manager: &RoomManager,
    player_manager: &PlayerManager,
    room_name: &str,
    base_port: i32,
) -> RoomUnitOutcome {
    let Some(room) = room_manager.get_public_rooms().into_iter().find(|room| {
        room.data().name() == room_name
            && room.data().room_type() == RoomType::Public
            && !room.data().is_hidden()
    }) else {
        return RoomUnitOutcome::missing_room();
    };

    let room_port = room.data().server_port(base_port);
    let mut sessions = player_manager
        .players()
        .values()
        .filter(|session| session.server_port() == room_port)
        .collect::<Vec<_>>();
    sessions.sort_by_key(|session| session.connection_id());

    RoomUnitOutcome::unit_members(
        [room.clone()],
        sessions
            .into_iter()
            .map(|session| session.details().username().to_owned()),
    )
}
