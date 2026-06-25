use super::room_unit_network_plan::*;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomData, RoomSummary};

fn public_room(id: i32, name: &str, class_name: &str, player_count: usize) -> RoomSummary {
    let mut room = RoomSummary::new(RoomData::new(
        id,
        false,
        RoomType::Public,
        -1,
        "",
        name,
        0,
        "",
        25,
        "description",
        "pool_b",
        class_name,
        "wall",
        "floor",
        false,
        true,
    ));
    room.set_player_count(player_count);
    room
}

#[test]
fn maps_unit_listener_to_all_units_packet() {
    let effects = RoomUnitNetworkPlan::plan(
        &RoomUnitOutcome::listener([public_room(5, "Habbo Lido", "lido", 2)]),
        42,
        "127.0.0.1",
        22004,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#ALLUNITS\rHabbo Lido,2,25,127.0.0.1/127.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##".to_owned(),
        }]
    );
}

#[test]
fn maps_unit_members_to_all_units_and_member_packets() {
    let effects = RoomUnitNetworkPlan::plan(
        &RoomUnitOutcome::unit_members([public_room(5, "Habbo Lido", "lido", 2)], ["alice", "bob"]),
        42,
        "10.0.0.1",
        22004,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ALLUNITS\rHabbo Lido,2,25,10.0.0.1/10.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#UNITMEMBERS\ralice\rbob##".to_owned(),
            },
        ]
    );
}

#[test]
fn missing_unit_room_has_no_network_effect() {
    assert!(
        RoomUnitNetworkPlan::plan(&RoomUnitOutcome::missing_room(), 42, "127.0.0.1", 22004)
            .is_empty()
    );
}
