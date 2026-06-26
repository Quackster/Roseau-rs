use crate::game::player::PlayerDetails;
use crate::game::room::{Room, RoomEntryOutcome};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEntryIncomingPlan;

impl RoomEntryIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        room: &Room,
        player: &PlayerDetails,
        room_players: &[PlayerDetails],
        has_room_all_rights: bool,
    ) -> Vec<RoomEntryOutcome> {
        let IncomingExecutionEffect::TryFlat { password, .. } = effect else {
            return Vec::new();
        };

        vec![room.try_flat(player, room_players, password, has_room_all_rights)]
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        room: &Room,
        player: &PlayerDetails,
        room_players: &[PlayerDetails],
        has_room_all_rights: bool,
    ) -> Vec<RoomEntryOutcome> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room, player, room_players, has_room_all_rights))
            .collect()
    }
}

#[cfg(test)]
#[path = "room_entry_incoming_plan_tests.rs"]
mod tests;
