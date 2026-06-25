use crate::game::room::settings::{RoomState, RoomType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomData {
    id: i32,
    room_type: RoomType,
    owner_id: i32,
    owner_name: String,
    name: String,
    state: RoomState,
    password: String,
    users_max: i32,
    description: String,
    model: String,
    class_name: String,
    wall: String,
    floor: String,
    all_super_user: bool,
    show_owner_name: bool,
    hidden: bool,
}

impl RoomData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        hidden: bool,
        room_type: RoomType,
        owner_id: i32,
        owner_name: impl Into<String>,
        name: impl Into<String>,
        state: i32,
        password: impl Into<String>,
        users_max: i32,
        description: impl Into<String>,
        model: impl Into<String>,
        class_name: impl Into<String>,
        wall: impl Into<String>,
        floor: impl Into<String>,
        all_super_user: bool,
        show_owner_name: bool,
    ) -> Self {
        Self {
            id,
            hidden,
            room_type,
            owner_id,
            owner_name: owner_name.into(),
            name: name.into(),
            state: RoomState::from_code(state),
            password: password.into(),
            users_max,
            description: description.into(),
            model: model.into(),
            class_name: class_name.into(),
            wall: wall.into(),
            floor: floor.into(),
            all_super_user,
            show_owner_name,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn room_type(&self) -> RoomType {
        self.room_type
    }

    pub fn owner_id(&self) -> i32 {
        self.owner_id
    }

    pub fn owner_name(&self) -> &str {
        &self.owner_name
    }

    pub fn set_owner_name(&mut self, owner_name: impl Into<String>) {
        self.owner_name = owner_name.into();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn state(&self) -> RoomState {
        self.state
    }

    pub fn set_state(&mut self, state: i32) {
        self.state = RoomState::from_code(state);
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn set_password(&mut self, password: impl Into<String>) {
        self.password = password.into();
    }

    pub fn users_max(&self) -> i32 {
        self.users_max
    }

    pub fn set_users_max(&mut self, users_max: i32) {
        self.users_max = users_max;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }

    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn wall(&self) -> &str {
        &self.wall
    }

    pub fn set_wall(&mut self, wall: impl Into<String>) {
        self.wall = wall.into();
    }

    pub fn floor(&self) -> &str {
        &self.floor
    }

    pub fn set_floor(&mut self, floor: impl Into<String>) {
        self.floor = floor.into();
    }

    pub fn wall_height(&self) -> i32 {
        -1
    }

    pub fn server_port(&self, base_port: i32) -> i32 {
        self.id + base_port
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn has_all_super_user(&self) -> bool {
        self.all_super_user
    }

    pub fn set_all_super_user(&mut self, all_super_user: bool) {
        self.all_super_user = all_super_user;
    }

    pub fn show_owner_name(&self) -> bool {
        self.show_owner_name
    }

    pub fn set_show_owner_name(&mut self, show_owner_name: bool) {
        self.show_owner_name = show_owner_name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room_data() -> RoomData {
        RoomData::new(
            42,
            false,
            RoomType::Private,
            7,
            "alice",
            "My room",
            2,
            "secret",
            25,
            "desc",
            "model_a",
            "default",
            "wallpaper",
            "floor",
            false,
            true,
        )
    }

    #[test]
    fn stores_room_data_fields() {
        let data = room_data();

        assert_eq!(data.id(), 42);
        assert_eq!(data.room_type(), RoomType::Private);
        assert_eq!(data.owner_id(), 7);
        assert_eq!(data.owner_name(), "alice");
        assert_eq!(data.name(), "My room");
        assert_eq!(data.state(), RoomState::Password);
        assert_eq!(data.password(), "secret");
        assert_eq!(data.users_max(), 25);
        assert_eq!(data.model_name(), "model_a");
        assert_eq!(data.class_name(), "default");
        assert_eq!(data.wall_height(), -1);
        assert_eq!(data.server_port(30001), 30043);
        assert!(data.show_owner_name());
    }

    #[test]
    fn mutates_room_data_fields() {
        let mut data = room_data();
        data.set_name("Other");
        data.set_state(1);
        data.set_password("door");
        data.set_description("changed");
        data.set_wall("new-wall");
        data.set_floor("new-floor");
        data.set_all_super_user(true);
        data.set_show_owner_name(false);

        assert_eq!(data.name(), "Other");
        assert_eq!(data.state(), RoomState::Doorbell);
        assert_eq!(data.password(), "door");
        assert_eq!(data.description(), "changed");
        assert_eq!(data.wall(), "new-wall");
        assert_eq!(data.floor(), "new-floor");
        assert!(data.has_all_super_user());
        assert!(!data.show_owner_name());
    }
}
