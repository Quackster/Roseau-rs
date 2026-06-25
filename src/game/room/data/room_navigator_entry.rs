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
mod tests {
    use super::*;
    use crate::game::room::settings::RoomType;

    fn room_data(show_owner_name: bool) -> RoomData {
        RoomData::new(
            12,
            false,
            RoomType::Private,
            7,
            "alice",
            "Tea Room",
            1,
            "secret",
            25,
            "A quiet room",
            "model_a",
            "class",
            "201",
            "0",
            false,
            show_owner_name,
        )
    }

    #[test]
    fn serialises_private_room_navigator_entry_with_owner_name() {
        let entry = RoomNavigatorEntry::new(
            room_data(false),
            NavigatorRequest::PrivateRooms,
            "127.0.0.1",
            37119,
            3,
        );
        let mut response = NettyResponse::with_header("ROOMS");
        response.append_object(&entry);

        assert_eq!(
            response.get(),
            "#ROOMS\r12/Tea Room/alice/closed//floor1/127.0.0.1/127.0.0.1/37119/3/null/A quiet room##"
        );
    }

    #[test]
    fn hides_owner_name_for_non_private_room_request_when_configured() {
        let entry = RoomNavigatorEntry::new(
            room_data(false),
            NavigatorRequest::PopularRooms,
            "10.0.0.1",
            37119,
            0,
        );
        let mut response = NettyResponse::with_header("ROOMS");
        response.append_object(&entry);

        assert_eq!(
            response.get(),
            "#ROOMS\r12/Tea Room/-/closed//floor1/10.0.0.1/10.0.0.1/37119/0/null/A quiet room##"
        );
    }
}
