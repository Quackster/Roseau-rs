use crate::protocol::NettyResponse;

use super::IncomingCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingContext {
    authenticated: bool,
    main_server_connection: bool,
    in_room: bool,
    room_model_name: Option<String>,
    current_room_name: Option<String>,
    enterable_door_item_ids: Vec<i32>,
    carry_drink_time: i32,
    credits: i32,
    username_chars: String,
    sent: Vec<NettyResponse>,
    commands: Vec<IncomingCommand>,
}

impl IncomingContext {
    pub fn new() -> Self {
        Self {
            authenticated: false,
            main_server_connection: true,
            in_room: false,
            room_model_name: None,
            current_room_name: None,
            enterable_door_item_ids: Vec::new(),
            carry_drink_time: 0,
            credits: 0,
            username_chars: "*".to_owned(),
            sent: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn authenticated(mut self, authenticated: bool) -> Self {
        self.authenticated = authenticated;
        self
    }

    pub fn main_server_connection(mut self, main_server_connection: bool) -> Self {
        self.main_server_connection = main_server_connection;
        self
    }

    pub fn in_room(mut self, in_room: bool) -> Self {
        self.in_room = in_room;
        self
    }

    pub fn room_model_name(mut self, room_model_name: impl Into<String>) -> Self {
        self.in_room = true;
        self.room_model_name = Some(room_model_name.into());
        self
    }

    pub fn current_room_name(mut self, current_room_name: impl Into<String>) -> Self {
        self.in_room = true;
        self.current_room_name = Some(current_room_name.into());
        self
    }

    pub fn enterable_door_item(mut self, item_id: i32) -> Self {
        self.in_room = true;
        self.enterable_door_item_ids.push(item_id);
        self
    }

    pub fn carry_drink_time(mut self, carry_drink_time: i32) -> Self {
        self.carry_drink_time = carry_drink_time;
        self
    }

    pub fn credits(mut self, credits: i32) -> Self {
        self.credits = credits;
        self
    }

    pub fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    pub fn set_main_server_connection(&mut self, main_server_connection: bool) {
        self.main_server_connection = main_server_connection;
    }

    pub fn set_in_room(&mut self, in_room: bool) {
        self.in_room = in_room;
    }

    pub fn set_room_model_name(&mut self, room_model_name: impl Into<String>) {
        self.in_room = true;
        self.room_model_name = Some(room_model_name.into());
    }

    pub fn set_current_room_name(&mut self, current_room_name: impl Into<String>) {
        self.in_room = true;
        self.current_room_name = Some(current_room_name.into());
    }

    pub fn set_credits(&mut self, credits: i32) {
        self.credits = credits;
    }

    pub fn username_chars(mut self, username_chars: impl Into<String>) -> Self {
        self.username_chars = username_chars.into();
        self
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn is_main_server_connection(&self) -> bool {
        self.main_server_connection
    }

    pub fn is_in_room(&self) -> bool {
        self.in_room
    }

    pub fn is_in_room_model(&self, room_model_name: &str) -> bool {
        self.room_model_name.as_deref() == Some(room_model_name)
    }

    pub fn current_room_name_value(&self) -> Option<&str> {
        self.current_room_name.as_deref()
    }

    pub fn can_enter_door_item(&self, item_id: i32) -> bool {
        self.in_room && self.enterable_door_item_ids.contains(&item_id)
    }

    pub fn carry_drink_time_value(&self) -> i32 {
        self.carry_drink_time
    }

    pub fn credits_value(&self) -> i32 {
        self.credits
    }

    pub fn username_chars_value(&self) -> &str {
        &self.username_chars
    }

    pub fn send(&mut self, response: NettyResponse) {
        self.sent.push(response);
    }

    pub fn record(&mut self, command: IncomingCommand) {
        self.commands.push(command);
    }

    pub fn sent(&self) -> &[NettyResponse] {
        &self.sent
    }

    pub fn take_sent(&mut self) -> Vec<NettyResponse> {
        std::mem::take(&mut self.sent)
    }

    pub fn commands(&self) -> &[IncomingCommand] {
        &self.commands
    }

    pub fn take_commands(&mut self) -> Vec<IncomingCommand> {
        std::mem::take(&mut self.commands)
    }
}

impl Default for IncomingContext {
    fn default() -> Self {
        Self::new()
    }
}
