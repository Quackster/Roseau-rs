use crate::game::navigator::NavigatorRequest;
use crate::game::room::RoomData;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomNavigatorEntry {
    data: RoomData,
    request: NavigatorRequest,
    server_ip: String,
    private_server_port: u16,
    users_now: i32,
}

impl RoomNavigatorEntry {
    pub fn new(
        data: RoomData,
        request: NavigatorRequest,
        server_ip: impl Into<String>,
        private_server_port: u16,
        users_now: i32,
    ) -> Self {
        Self {
            data,
            request,
            server_ip: server_ip.into(),
            private_server_port,
            users_now,
        }
    }

    pub fn data(&self) -> &RoomData {
        &self.data
    }

    pub fn request(&self) -> NavigatorRequest {
        self.request
    }

    pub fn server_ip(&self) -> &str {
        &self.server_ip
    }

    pub fn private_server_port(&self) -> u16 {
        self.private_server_port
    }

    pub fn users_now(&self) -> i32 {
        self.users_now
    }

    fn visible_owner_name(&self) -> &str {
        if self.request == NavigatorRequest::PrivateRooms || self.data.show_owner_name() {
            self.data.owner_name()
        } else {
            "-"
        }
    }
}

impl SerializableObject for RoomNavigatorEntry {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_new_argument(self.data.id());
        response.append_part_argument(self.data.name());
        response.append_part_argument(self.visible_owner_name());
        response.append_part_argument(self.data.state());
        response.append_part_argument("");
        response.append_part_argument("floor1");
        response.append_part_argument(&self.server_ip);
        response.append_part_argument(&self.server_ip);
        response.append_part_argument(self.private_server_port);
        response.append_part_argument(self.users_now);
        response.append_part_argument("null");
        response.append_part_argument(self.data.description());
    }
}

#[cfg(test)]
#[path = "room_navigator_entry_tests.rs"]
mod tests;
