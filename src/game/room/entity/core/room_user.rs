use std::collections::{HashMap, VecDeque};

use crate::game::room::entity::{RoomUserEffect, RoomUserStatus};
use crate::game::room::model::Position;
use crate::messages::outgoing::{
    RoomUserStatus as OutgoingRoomUserStatus, StatusEntity, UserEntry,
};
use crate::settings::{CARRY_DRINK_INTERVAL_TICKS, DEFAULT_AFK_ROOM_KICK_TICKS};

#[derive(Debug, Clone, PartialEq)]
pub struct RoomUser {
    pub(crate) entity_id: i32,
    pub(crate) username: String,
    figure: String,
    mission: String,
    pool_figure: Option<String>,
    dance_id: i32,
    time_until_next_drink: i64,
    pub(crate) position: Position,
    pub(crate) goal: Option<Position>,
    pub(crate) next: Option<Position>,
    pub(crate) statuses: HashMap<String, RoomUserStatus>,
    pub(crate) path: VecDeque<Position>,
    pub(crate) room_id: i32,
    pub(crate) walking: bool,
    pub(crate) needs_update: bool,
    pub(crate) can_walk: bool,
    look_reset_time: i64,
    pub(crate) current_item_id: Option<i32>,
    afk_timer: i32,
    afk_room_kick_ticks: i32,
    pub(crate) kick_when_stop: bool,
}

impl RoomUser {
    pub fn new(
        entity_id: i32,
        username: impl Into<String>,
        figure: impl Into<String>,
        mission: impl Into<String>,
        pool_figure: Option<impl Into<String>>,
    ) -> Self {
        let mut user = Self {
            entity_id,
            username: username.into(),
            figure: figure.into(),
            mission: mission.into(),
            pool_figure: pool_figure.map(Into::into),
            dance_id: 0,
            time_until_next_drink: -1,
            position: Position::new(0, 0, 0.0),
            goal: None,
            next: None,
            statuses: HashMap::new(),
            path: VecDeque::new(),
            room_id: 0,
            walking: false,
            needs_update: false,
            can_walk: true,
            look_reset_time: -1,
            current_item_id: None,
            afk_timer: DEFAULT_AFK_ROOM_KICK_TICKS,
            afk_room_kick_ticks: DEFAULT_AFK_ROOM_KICK_TICKS,
            kick_when_stop: false,
        };
        user.dispose();
        user
    }

    pub fn dispose(&mut self) {
        self.statuses.clear();
        self.path.clear();
        self.position = Position::new(0, 0, 0.0);
        self.goal = Some(Position::new(0, 0, 0.0));
        self.next = None;
        self.current_item_id = None;
        self.needs_update = false;
        self.walking = false;
        self.can_walk = true;
        self.dance_id = 0;
        self.time_until_next_drink = -1;
        self.look_reset_time = -1;
        self.kick_when_stop = false;
        self.reset_afk_timer();
    }

    pub fn set_status(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        infinite: bool,
        duration: i64,
    ) {
        let key = key.into();
        if key == "carryd" {
            self.time_until_next_drink = CARRY_DRINK_INTERVAL_TICKS;
        }

        self.statuses.insert(
            key.clone(),
            RoomUserStatus::new(key, value, infinite, duration),
        );
    }

    pub fn set_status_with_update(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        infinite: bool,
        duration: i64,
        send_update: bool,
    ) {
        self.set_status(key, value, infinite, duration);
        if send_update {
            self.needs_update = true;
        }
    }

    pub fn remove_status(&mut self, key: &str) -> Option<RoomUserStatus> {
        self.statuses.remove(key)
    }

    pub fn tick_status(&mut self, key: &str) {
        if let Some(status) = self.statuses.get_mut(key) {
            status.tick();
        }
    }

    pub fn contains_status(&self, key: &str) -> bool {
        self.statuses.contains_key(key)
    }

    pub fn status(&self, key: &str) -> Option<&RoomUserStatus> {
        self.statuses.get(key)
    }

    pub fn send_status_effect(&self) -> RoomUserEffect {
        RoomUserEffect::SendStatus {
            entity_id: self.entity_id,
        }
    }

    pub fn users_effect(&self) -> RoomUserEffect {
        RoomUserEffect::SendUsers {
            entity_id: self.entity_id,
        }
    }

    pub fn status_entity(&self) -> StatusEntity {
        let mut statuses = self.statuses.values().collect::<Vec<_>>();
        statuses.sort_by(|left, right| left.key().cmp(right.key()));

        StatusEntity::new(
            &self.username,
            self.position.x(),
            self.position.y(),
            self.position.z().to_string(),
            self.position.head_rotation(),
            self.position.rotation(),
            statuses
                .into_iter()
                .map(|status| OutgoingRoomUserStatus::new(status.key(), status.value())),
        )
    }

    pub fn user_entry(&self) -> UserEntry {
        UserEntry::new(
            &self.username,
            &self.figure,
            self.position.x(),
            self.position.y(),
            self.position.z(),
            &self.mission,
            self.pool_figure.as_deref(),
        )
    }

    pub fn reset_afk_timer(&mut self) {
        self.afk_timer = self.afk_room_kick_ticks;
    }

    pub fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn goal(&self) -> Option<Position> {
        self.goal
    }

    pub fn set_goal(&mut self, goal: Option<Position>) {
        self.goal = goal;
    }

    pub fn next(&self) -> Option<Position> {
        self.next
    }

    pub fn set_next(&mut self, next: Option<Position>) {
        self.next = next;
    }

    pub fn path(&self) -> &VecDeque<Position> {
        &self.path
    }

    pub fn set_path(&mut self, path: impl Into<VecDeque<Position>>) {
        self.path.clear();
        self.path = path.into();
    }

    pub fn room_id(&self) -> i32 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    pub fn is_walking(&self) -> bool {
        self.walking
    }

    pub fn set_walking(&mut self, walking: bool) {
        self.walking = walking;
    }

    pub fn needs_update(&self) -> bool {
        self.needs_update
    }

    pub fn set_needs_update(&mut self, needs_update: bool) {
        self.needs_update = needs_update;
    }

    pub fn can_walk(&self) -> bool {
        self.can_walk
    }

    pub fn set_can_walk(&mut self, can_walk: bool) {
        self.can_walk = can_walk;
    }

    pub fn dance_id(&self) -> i32 {
        self.dance_id
    }

    pub fn set_dance_id(&mut self, dance_id: i32) {
        self.dance_id = dance_id;
    }

    pub fn is_dancing(&self) -> bool {
        self.dance_id != 0
    }

    pub fn time_until_next_drink(&self) -> i64 {
        self.time_until_next_drink
    }

    pub fn set_time_until_next_drink(&mut self, time_until_next_drink: i64) {
        self.time_until_next_drink = time_until_next_drink;
    }

    pub fn look_reset_time(&self) -> i64 {
        self.look_reset_time
    }

    pub fn set_look_reset_time(&mut self, look_reset_time: i64) {
        self.look_reset_time = look_reset_time;
    }

    pub fn current_item_id(&self) -> Option<i32> {
        self.current_item_id
    }

    pub fn set_current_item_id(&mut self, current_item_id: Option<i32>) {
        self.current_item_id = current_item_id;
    }

    pub fn kick_when_stop(&self) -> bool {
        self.kick_when_stop
    }

    pub fn set_kick_when_stop(&mut self, kick_when_stop: bool) {
        self.kick_when_stop = kick_when_stop;
    }

    pub fn afk_timer(&self) -> i32 {
        self.afk_timer
    }

    pub fn set_afk_timer(&mut self, afk_timer: i32) {
        self.afk_timer = afk_timer;
    }

    pub fn statuses(&self) -> &HashMap<String, RoomUserStatus> {
        &self.statuses
    }
}
