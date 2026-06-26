use crate::game::room::entity::RoomUser;
use crate::game::room::model::Position;
use crate::game::room::schedulers::events::{
    BotMoveRoomEvent, ClubMassivaDiscoEvent, HabboLidoEvent, UserStatusEvent,
};

/// Per-room scheduler state. Mirrors Java's `Room.firstPlayerEntry`, which
/// registers the room events and starts the 500 ms walk + event schedulers.
/// The bot entities are persisted here so both schedulers tick them.
#[derive(Debug, Clone, PartialEq)]
pub struct RoomScheduler {
    user_status_event: UserStatusEvent,
    bot_move_event: Option<BotMoveRoomEvent>,
    lido_event: Option<HabboLidoEvent>,
    disco_event: Option<ClubMassivaDiscoEvent>,
    bots: Vec<RoomUser>,
    bot_start_positions: Vec<Position>,
    bot_patrol_positions: Vec<Vec<(i32, i32)>>,
}

impl RoomScheduler {
    pub fn register(
        model_name: &str,
        bots: Vec<RoomUser>,
        bot_start_positions: Vec<Position>,
        bot_patrol_positions: Vec<Vec<(i32, i32)>>,
    ) -> Self {
        Self {
            user_status_event: UserStatusEvent::new(),
            // Java firstPlayerEntry: bots register BotMoveRoomEvent,
            // `bar_b` registers ClubMassivaDiscoEvent, `pool_b` registers
            // HabboLidoEvent, and UserStatusEvent is always registered.
            bot_move_event: (!bots.is_empty()).then(BotMoveRoomEvent::new),
            lido_event: (model_name == "pool_b").then(HabboLidoEvent::new),
            disco_event: (model_name == "bar_b").then(ClubMassivaDiscoEvent::new),
            bots,
            bot_start_positions,
            bot_patrol_positions,
        }
    }

    pub fn bot_start_positions(&self) -> &[Position] {
        &self.bot_start_positions
    }

    pub fn bots(&self) -> &[RoomUser] {
        &self.bots
    }

    pub fn bots_mut(&mut self) -> &mut Vec<RoomUser> {
        &mut self.bots
    }

    pub fn bot_patrol_positions(&self) -> &[Vec<(i32, i32)>] {
        &self.bot_patrol_positions
    }

    pub fn user_status_event_mut(&mut self) -> &mut UserStatusEvent {
        &mut self.user_status_event
    }

    pub fn bot_move_event_mut(&mut self) -> Option<&mut BotMoveRoomEvent> {
        self.bot_move_event.as_mut()
    }

    pub fn lido_event_mut(&mut self) -> Option<&mut HabboLidoEvent> {
        self.lido_event.as_mut()
    }

    pub fn disco_event_mut(&mut self) -> Option<&mut ClubMassivaDiscoEvent> {
        self.disco_event.as_mut()
    }
}

#[cfg(test)]
#[path = "room_scheduler_tests.rs"]
mod tests;
