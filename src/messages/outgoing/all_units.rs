use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllUnits {
    server_ip: String,
    rooms: Vec<PublicUnit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicUnit {
    name: String,
    users_now: i32,
    users_max: i32,
    server_port: i32,
    cct: String,
    model_name: String,
}

impl AllUnits {
    pub fn new(server_ip: impl Into<String>, rooms: impl IntoIterator<Item = PublicUnit>) -> Self {
        Self {
            server_ip: server_ip.into(),
            rooms: rooms.into_iter().collect(),
        }
    }
}

impl PublicUnit {
    pub fn new(
        name: impl Into<String>,
        users_now: i32,
        users_max: i32,
        server_port: i32,
        cct: impl Into<String>,
        model_name: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            users_now,
            users_max,
            server_port,
            cct: cct.into(),
            model_name: model_name.into(),
        }
    }
}

impl OutgoingMessage for AllUnits {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ALLUNITS");

        for room in &self.rooms {
            response.append_new_argument(&room.name);
            response.append_argument_with(room.users_now, ',');
            response.append_argument_with(room.users_max, ',');
            response.append_argument_with(&self.server_ip, ',');
            response.append_argument_with(&self.server_ip, '/');
            response.append_argument_with(room.server_port, ',');
            response.append_argument_with(&room.name, ',');
            response.append_tab_argument(&room.cct);
            response.append_argument_with(room.users_now, ',');
            response.append_argument_with(room.users_max, ',');
            response.append_argument_with(&room.model_name, ',');
        }
    }
}
