use crate::game::room::model::Position;
use crate::game::room::schedulers::{RoomEvent, SchedulerEffect};

#[derive(Debug, Clone, PartialEq)]
pub struct LidoPlayerState {
    id: i32,
    username: String,
    position: Position,
    walking: bool,
}

impl LidoPlayerState {
    pub fn new(id: i32, username: impl Into<String>, position: Position, walking: bool) -> Self {
        Self {
            id,
            username: username.into(),
            position,
            walking,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PoolQueueTile {
    position: Position,
    target: Position,
}

impl PoolQueueTile {
    pub fn new(position: Position, target: Position) -> Self {
        Self { position, target }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HabboLidoEvent {
    event: RoomEvent,
    following_id: i32,
    camera_type: i32,
}

impl HabboLidoEvent {
    pub fn new() -> Self {
        Self {
            event: RoomEvent::new(),
            following_id: -1,
            camera_type: -1,
        }
    }

    pub fn tick(
        &mut self,
        players: &[LidoPlayerState],
        pool_queue_tiles: &[PoolQueueTile],
        camera_effect: i32,
        target_index: usize,
    ) -> Vec<SchedulerEffect> {
        let mut effects = Vec::new();

        if !players.iter().any(|player| player.id == self.following_id) {
            effects.extend(self.find_new_target(players, target_index));
        }

        if self.event.can_tick(9) {
            match camera_effect {
                0 => effects.extend(self.find_new_target(players, target_index)),
                1 | 2 if self.camera_type != camera_effect => {
                    self.camera_type = camera_effect;
                    effects.push(SchedulerEffect::SetCamera(camera_effect));
                }
                _ => {}
            }
        }

        for player in players {
            if player.walking {
                continue;
            }

            if let Some(tile) = pool_queue_tiles
                .iter()
                .find(|tile| tile.position.is_match(player.position))
            {
                effects.push(SchedulerEffect::WalkTo {
                    entity_id: player.id,
                    x: tile.target.x(),
                    y: tile.target.y(),
                });
            }
        }

        self.event.increase_ticked();
        effects
    }

    pub fn following_id(&self) -> i32 {
        self.following_id
    }

    pub fn camera_type(&self) -> i32 {
        self.camera_type
    }

    fn find_new_target(
        &mut self,
        players: &[LidoPlayerState],
        target_index: usize,
    ) -> Vec<SchedulerEffect> {
        let Some(player) = players.get(target_index % players.len().max(1)) else {
            return Vec::new();
        };

        self.following_id = player.id;
        vec![SchedulerEffect::TargetCamera {
            player_id: player.id,
            username: player.username.clone(),
        }]
    }
}

impl Default for HabboLidoEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "habbo_lido_event_tests.rs"]
mod tests;
