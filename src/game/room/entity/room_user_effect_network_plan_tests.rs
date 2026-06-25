use crate::game::player::{PlayerDetails, PlayerManager, PlayerSession};
use crate::game::room::entity::{RoomUser, RoomUserEffect, RoomUserEffectNetworkPlan};
use crate::game::room::model::Position;
use crate::server::PlayerNetworkEffect;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn manager() -> PlayerManager {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(70, 30000, details(7, "Alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "Bob")));
    manager.insert(PlayerSession::new(90, 30000, details(9, "Carol")));
    manager
}

fn room_user(entity_id: i32, name: &str) -> RoomUser {
    let mut user = RoomUser::new(entity_id, name, "hd-100", "hello", None::<String>);
    user.set_room_id(42);
    user
}

#[test]
fn sends_chat_to_speaker_and_nearby_room_players() {
    let mut alice = room_user(7, "Alice");
    alice.set_position(Position::new(0, 0, 0.0));
    let mut bob = room_user(8, "Bob");
    bob.set_position(Position::new(2, 0, 0.0));
    let mut carol = room_user(9, "Carol");
    carol.set_position(Position::new(8, 0, 0.0));

    let effects = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::Chat {
            header: "CHAT".to_owned(),
            username: "Alice".to_owned(),
            message: "hello".to_owned(),
        },
        7,
        &[7, 8, 9],
        &[alice, bob, carol],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#CHAT\rAlice hello##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#CHAT\rAlice hello##".to_owned(),
            },
        ]
    );
}

#[test]
fn broadcasts_shout_to_all_room_players() {
    let effects = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::Chat {
            header: "SHOUT".to_owned(),
            username: "Alice".to_owned(),
            message: "hello".to_owned(),
        },
        7,
        &[7, 8, 9],
        &[],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#SHOUT\rAlice hello##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#SHOUT\rAlice hello##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 90,
                packet: "#SHOUT\rAlice hello##".to_owned(),
            },
        ]
    );
}

#[test]
fn sends_whispers_to_sender_and_target_without_room_broadcast() {
    let effects = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::Whisper {
            username: "Alice".to_owned(),
            target_username: Some("Bob".to_owned()),
            message: "secret".to_owned(),
        },
        7,
        &[7, 8, 9],
        &[],
        &manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#WHISPER\rAlice secret##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#WHISPER\rAlice secret##".to_owned(),
            },
        ]
    );
}

#[test]
fn echoes_whisper_only_to_sender_when_target_is_missing_or_same_player() {
    let manager = manager();

    let missing_target = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::Whisper {
            username: "Alice".to_owned(),
            target_username: Some("Nobody".to_owned()),
            message: "secret".to_owned(),
        },
        7,
        &[7, 8, 9],
        &[],
        &manager,
    );
    let same_target = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::Whisper {
            username: "Alice".to_owned(),
            target_username: Some("Alice".to_owned()),
            message: "secret".to_owned(),
        },
        7,
        &[7, 8, 9],
        &[],
        &manager,
    );

    assert_eq!(
        missing_target,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 70,
            packet: "#WHISPER\rAlice secret##".to_owned(),
        }]
    );
    assert_eq!(same_target, missing_target);
}

#[test]
fn broadcasts_single_user_status_and_entry_packets() {
    let mut user = room_user(7, "Alice");
    user.set_status("sit", " 1", true, -1);
    let manager = manager();

    let effects = RoomUserEffectNetworkPlan::plan_all(
        &[
            RoomUserEffect::SendStatus { entity_id: 7 },
            RoomUserEffect::SendUsers { entity_id: 7 },
        ],
        7,
        &[7],
        &[user],
        &manager,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#STATUS \rAlice 0,0,0,0,0/sit 1/##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#USERS\r  Alice hd-100 0 0 0 hello##".to_owned(),
            },
        ]
    );
}

#[test]
fn sends_direct_ticket_and_kick_effects_to_acting_user() {
    let manager = manager();

    let effects = RoomUserEffectNetworkPlan::plan_all(
        &[RoomUserEffect::NotEnoughTickets, RoomUserEffect::Kick],
        8,
        &[7, 8],
        &[],
        &manager,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#PH_NOTICKETS##".to_owned(),
            },
            PlayerNetworkEffect::CloseConnection { connection_id: 80 },
        ]
    );
}

#[test]
fn ignores_effects_that_need_other_runtime_boundaries() {
    let effects = RoomUserEffectNetworkPlan::plan(
        &RoomUserEffect::TriggerCurrentItem { item_id: Some(3) },
        7,
        &[7],
        &[room_user(7, "Alice")],
        &manager(),
    );

    assert!(effects.is_empty());
}
