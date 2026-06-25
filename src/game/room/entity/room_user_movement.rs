use std::collections::VecDeque;

use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::{calculate_direction, Position};

impl RoomUser {
    pub fn walk_to(&mut self, x: i32, y: i32, path: impl Into<VecDeque<Position>>) -> bool {
        if self.room_id == 0 || !self.can_walk || self.position.is_match(Position::new(x, y, 0.0)) {
            return false;
        }

        if let Some(next) = self.next.take() {
            self.position =
                Position::with_rotation(next.x(), next.y(), self.position.z(), next.rotation());
        }

        self.kick_when_stop = false;
        self.reset_afk_timer();

        let path = path.into();
        if path.is_empty() {
            return false;
        }

        self.goal = Some(Position::new(x, y, 0.0));
        self.path = path;
        self.walking = true;
        true
    }

    pub fn go_away(
        &mut self,
        door_position: Position,
        path: impl Into<VecDeque<Position>>,
    ) -> Vec<RoomUserEffect> {
        if !self.position.is_match(door_position)
            && self.walk_to(door_position.x(), door_position.y(), path)
        {
            self.kick_when_stop = true;
            Vec::new()
        } else {
            vec![RoomUserEffect::Kick]
        }
    }

    pub fn splash_from_pool_lift(
        &mut self,
        landing_position: Position,
        exit_path: impl Into<VecDeque<Position>>,
    ) -> Vec<RoomUserEffect> {
        let landing_text = landing_position.to_string();
        self.position = landing_position;
        self.set_status_with_update("swim", "", true, -1, true);
        self.can_walk = true;
        self.walk_to(18, 19, exit_path);

        vec![RoomUserEffect::ShowProgram(vec![
            "BIGSPLASH".to_owned(),
            "POSITION".to_owned(),
            landing_text,
        ])]
    }

    pub fn stop_walking(&mut self) -> Vec<RoomUserEffect> {
        self.remove_status("mv");
        self.walking = false;
        self.goal = None;
        self.next = None;

        if self.kick_when_stop {
            return vec![RoomUserEffect::Kick];
        }

        self.needs_update = true;
        vec![RoomUserEffect::TriggerCurrentItem {
            item_id: self.current_item_id,
        }]
    }

    pub fn force_stop_walking(&mut self) {
        self.remove_status("mv");
        self.path.clear();
        self.walking = false;
    }

    pub fn look_towards(&mut self, look: Position) {
        if self.walking {
            return;
        }

        let direction =
            calculate_direction(self.position.x(), self.position.y(), look.x(), look.y()) as i32;
        let diff = self.position.rotation() - direction;

        if self.position.rotation() % 2 == 0 {
            let mut position = self.position;
            if diff > 0 {
                position.set_head_rotation(self.position.rotation() - 1);
            } else if diff < 0 {
                position.set_head_rotation(self.position.rotation() + 1);
            } else {
                position.set_head_rotation(self.position.rotation());
            }
            self.position = position;
        }

        self.needs_update = true;
    }

    pub fn update_new_height(&mut self, height: f64) {
        if (height - self.position.z()).abs() > f64::EPSILON {
            self.position = Position::with_rotation(
                self.position.x(),
                self.position.y(),
                height,
                self.position.rotation(),
            );
            self.needs_update = true;
        }
    }
}
