use std::collections::VecDeque;

use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct RoomWalkEntity {
    entity_id: i32,
    walking: bool,
    needs_update: bool,
    position: Position,
    goal: Option<Position>,
    next: Option<Position>,
    path: VecDeque<Position>,
    current_item_id: Option<i32>,
}

impl RoomWalkEntity {
    pub fn new(entity_id: i32, position: Position) -> Self {
        Self {
            entity_id,
            walking: false,
            needs_update: false,
            position,
            goal: None,
            next: None,
            path: VecDeque::new(),
            current_item_id: None,
        }
    }

    pub fn walking(mut self, walking: bool) -> Self {
        self.walking = walking;
        self
    }

    pub fn needs_update(mut self, needs_update: bool) -> Self {
        self.needs_update = needs_update;
        self
    }

    pub fn with_goal(mut self, goal: Option<Position>) -> Self {
        self.goal = goal;
        self
    }

    pub fn with_next(mut self, next: Option<Position>) -> Self {
        self.next = next;
        self
    }

    pub fn path(mut self, path: impl Into<VecDeque<Position>>) -> Self {
        self.path = path.into();
        self
    }

    pub fn current_item_id(mut self, current_item_id: Option<i32>) -> Self {
        self.current_item_id = current_item_id;
        self
    }

    pub fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub fn is_walking(&self) -> bool {
        self.walking
    }

    pub fn needs_update_value(&self) -> bool {
        self.needs_update
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn goal(&self) -> Option<Position> {
        self.goal
    }

    pub fn next(&self) -> Option<Position> {
        self.next
    }

    pub fn path_values(&self) -> &VecDeque<Position> {
        &self.path
    }

    pub fn current_item_id_value(&self) -> Option<i32> {
        self.current_item_id
    }
}
