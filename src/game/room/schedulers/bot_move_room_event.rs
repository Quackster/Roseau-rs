use crate::game::room::model::Position;
use crate::game::room::schedulers::{RoomEvent, SchedulerEffect};

#[derive(Debug, Clone, PartialEq)]
pub struct BotTickState {
    entity_id: i32,
    position: Position,
    start_position: Position,
    walking: bool,
    nearby_player_count: usize,
    patrol_positions: Vec<(i32, i32)>,
}

impl BotTickState {
    pub fn new(entity_id: i32, position: Position, start_position: Position) -> Self {
        Self {
            entity_id,
            position,
            start_position,
            walking: false,
            nearby_player_count: 0,
            patrol_positions: Vec::new(),
        }
    }

    pub fn walking(mut self, walking: bool) -> Self {
        self.walking = walking;
        self
    }

    pub fn nearby_player_count(mut self, nearby_player_count: usize) -> Self {
        self.nearby_player_count = nearby_player_count;
        self
    }

    pub fn patrol_positions(mut self, patrol_positions: Vec<(i32, i32)>) -> Self {
        self.patrol_positions = patrol_positions;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotMoveRoomEvent {
    event: RoomEvent,
}

impl BotMoveRoomEvent {
    pub fn new() -> Self {
        Self {
            event: RoomEvent::new(),
        }
    }

    pub fn tick(&mut self, bots: &[BotTickState], patrol_index: usize) -> Vec<SchedulerEffect> {
        let mut effects = Vec::new();

        for bot in bots {
            if bot.nearby_player_count > 0 {
                if !bot.position.is_match(bot.start_position) && !bot.walking {
                    if self.event.can_tick(10) {
                        effects.push(SchedulerEffect::WalkTo {
                            entity_id: bot.entity_id,
                            x: bot.start_position.x(),
                            y: bot.start_position.y(),
                        });
                    }
                } else if !bot.walking && bot.position.rotation() != bot.start_position.rotation() {
                    effects.push(SchedulerEffect::SetRotation {
                        entity_id: bot.entity_id,
                        rotation: bot.start_position.rotation(),
                    });
                    effects.push(SchedulerEffect::MarkNeedsUpdate {
                        entity_id: bot.entity_id,
                    });
                }
            } else if self.event.can_tick(10) && !bot.patrol_positions.is_empty() {
                let (x, y) = bot.patrol_positions[patrol_index % bot.patrol_positions.len()];
                effects.push(SchedulerEffect::WalkTo {
                    entity_id: bot.entity_id,
                    x,
                    y,
                });
            }
        }

        self.event.increase_ticked();
        effects
    }
}

impl Default for BotMoveRoomEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_bot_back_to_start_when_players_are_nearby() {
        let mut event = BotMoveRoomEvent::new();
        let bot = BotTickState::new(
            3,
            Position::new(7, 7, 0.0),
            Position::with_rotation(1, 2, 0.0, 4),
        )
        .nearby_player_count(1);

        assert_eq!(
            event.tick(&[bot], 0),
            vec![SchedulerEffect::WalkTo {
                entity_id: 3,
                x: 1,
                y: 2
            }]
        );
    }

    #[test]
    fn rotates_bot_to_start_rotation_when_already_home() {
        let mut event = BotMoveRoomEvent::new();
        let bot = BotTickState::new(
            3,
            Position::with_rotation(1, 2, 0.0, 2),
            Position::with_rotation(1, 2, 0.0, 4),
        )
        .nearby_player_count(1);

        assert_eq!(
            event.tick(&[bot], 0),
            vec![
                SchedulerEffect::SetRotation {
                    entity_id: 3,
                    rotation: 4
                },
                SchedulerEffect::MarkNeedsUpdate { entity_id: 3 }
            ]
        );
    }
}
