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
