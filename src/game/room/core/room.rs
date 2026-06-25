use crate::game::player::PlayerManager;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomData, RoomEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    order_id: i32,
    pub(super) disposed: bool,
    pub(super) data: RoomData,
    pub(super) rights: Vec<i32>,
    pub(super) player_ids: Vec<i32>,
    pub(super) bot_ids: Vec<i32>,
    pub(super) event_names: Vec<String>,
    pub(super) walk_ticks_scheduled: bool,
    pub(super) event_ticks_scheduled: bool,
}

impl Room {
    pub fn new(data: RoomData) -> Self {
        Self {
            order_id: -1,
            disposed: false,
            data,
            rights: Vec::new(),
            player_ids: Vec::new(),
            bot_ids: Vec::new(),
            event_names: Vec::new(),
            walk_ticks_scheduled: false,
            event_ticks_scheduled: false,
        }
    }

    pub fn load(&mut self, rights: Vec<i32>) -> Vec<RoomEffect> {
        let mut effects = Vec::new();

        if self.data.room_type() == RoomType::Public && !self.data.is_hidden() {
            effects.push(RoomEffect::StartPublicServer {
                room_name: self.data.name().to_owned(),
                port: self.data.server_port(0),
            });
        }

        self.rights = if self.data.room_type() == RoomType::Private {
            rights
        } else {
            Vec::new()
        };

        effects
    }

    pub fn first_player_entry(&mut self, bots: impl IntoIterator<Item = i32>) -> Vec<RoomEffect> {
        self.disposed = false;

        let mut effects = Vec::new();
        if !self.walk_ticks_scheduled {
            self.walk_ticks_scheduled = true;
            effects.push(RoomEffect::ScheduleWalkTicks);
        }

        effects.push(RoomEffect::LoadPassiveObjects {
            model_name: self.data.model_name().to_owned(),
            room_id: self.data.id(),
        });
        effects.push(RoomEffect::LoadBots {
            room_id: self.data.id(),
        });

        self.bot_ids = bots.into_iter().collect();
        effects.push(RoomEffect::RegenerateCollisionMaps);

        if self.data.model_name() == "bar_b" {
            effects.extend(self.register_event("club_massiva_disco"));
        }
        if self.data.model_name() == "pool_b" {
            effects.extend(self.register_event("habbo_lido"));
        }
        if !self.bot_ids.is_empty() {
            effects.extend(self.register_event("bot_move_room"));
        }
        effects.extend(self.register_event("user_status"));

        effects
    }

    pub fn register_event(&mut self, event_name: impl Into<String>) -> Vec<RoomEffect> {
        let event_name = event_name.into();
        let mut effects = Vec::new();

        if !self.event_ticks_scheduled {
            self.event_ticks_scheduled = true;
            effects.push(RoomEffect::ScheduleEventTicks);
        }

        self.event_names.push(event_name.clone());
        effects.push(RoomEffect::RegisterEvent { event_name });
        effects
    }

    pub fn add_player(&mut self, user_id: i32) {
        if !self.player_ids.contains(&user_id) {
            self.player_ids.push(user_id);
        }
    }

    pub fn remove_player(&mut self, user_id: i32) {
        self.player_ids.retain(|id| *id != user_id);
    }

    pub fn dispose(
        &mut self,
        force_disposal: bool,
        player_manager: &PlayerManager,
    ) -> Vec<RoomEffect> {
        if force_disposal {
            let mut effects = self
                .player_ids
                .iter()
                .map(|user_id| RoomEffect::LeaveRoom {
                    user_id: *user_id,
                    hotel_view: true,
                })
                .collect::<Vec<_>>();
            effects.extend(self.clear_runtime_data());
            self.disposed = true;
            effects.push(RoomEffect::RemoveLoadedRoom {
                room_id: self.data.id(),
            });
            return effects;
        }

        if self.disposed || !self.player_ids.is_empty() {
            return Vec::new();
        }

        let mut effects = self.clear_runtime_data();
        if self.data.room_type() == RoomType::Private
            && player_manager.get_by_id(self.data.owner_id()).is_none()
        {
            self.disposed = true;
            effects.push(RoomEffect::RemoveLoadedRoom {
                room_id: self.data.id(),
            });
        }

        effects
    }

    fn clear_runtime_data(&mut self) -> Vec<RoomEffect> {
        self.player_ids.clear();
        self.bot_ids.clear();
        self.event_names.clear();
        self.walk_ticks_scheduled = false;
        self.event_ticks_scheduled = false;
        vec![RoomEffect::ClearRuntimeData]
    }

    pub fn player_by_id(&self, user_id: i32) -> Option<i32> {
        self.player_ids.iter().copied().find(|id| *id == user_id)
    }

    pub fn data(&self) -> &RoomData {
        &self.data
    }

    pub fn rights(&self) -> &[i32] {
        &self.rights
    }

    pub fn player_ids(&self) -> &[i32] {
        &self.player_ids
    }

    pub fn bot_ids(&self) -> &[i32] {
        &self.bot_ids
    }

    pub fn event_names(&self) -> &[String] {
        &self.event_names
    }

    pub fn is_disposed(&self) -> bool {
        self.disposed
    }

    pub fn set_disposed(&mut self, disposed: bool) {
        self.disposed = disposed;
    }

    pub fn order_id(&self) -> i32 {
        self.order_id
    }

    pub fn set_order_id(&mut self, order_id: i32) {
        self.order_id = order_id;
    }
}
