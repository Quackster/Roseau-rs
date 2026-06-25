use crate::game::player::{Permission, PlayerDetails, PlayerManager};
use crate::game::room::settings::{RoomState, RoomType};
use crate::game::room::{Room, RoomData, RoomEffect, RoomEntryOutcome};

fn details(id: i32, name: &str, rank: i32) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        id, name, "mission", "figure", "", "email", rank, 0, "M", "GB", "", "", 0, "", 0,
    );
    details
}

fn room_data(room_type: RoomType, owner_id: i32, model: &str) -> RoomData {
    room_data_with_state(room_type, owner_id, model, 0, "")
}

fn room_data_with_state(
    room_type: RoomType,
    owner_id: i32,
    model: &str,
    state: i32,
    password: &str,
) -> RoomData {
    RoomData::new(
        12,
        false,
        room_type,
        owner_id,
        "owner",
        "room",
        state,
        password,
        25,
        "description",
        model,
        "class",
        "201",
        "0",
        false,
        true,
    )
}

#[test]
fn loads_private_rights_and_public_server_effects() {
    let mut private_room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    assert_eq!(private_room.load(vec![8]), Vec::new());
    assert_eq!(private_room.rights(), &[8]);

    let mut public_room = Room::new(room_data(RoomType::Public, 7, "pool_b"));
    assert_eq!(
        public_room.load(vec![8]),
        vec![RoomEffect::StartPublicServer {
            room_name: "room".to_owned(),
            port: 12,
        }]
    );
    assert!(public_room.rights().is_empty());
}

#[test]
fn first_player_entry_schedules_events_for_model_and_bots() {
    let mut room = Room::new(room_data(RoomType::Public, 7, "pool_b"));

    let effects = room.first_player_entry([100, 101]);

    assert!(effects.contains(&RoomEffect::ScheduleWalkTicks));
    assert!(effects.contains(&RoomEffect::RegenerateCollisionMaps));
    assert!(effects.contains(&RoomEffect::RegisterEvent {
        event_name: "habbo_lido".to_owned(),
    }));
    assert!(effects.contains(&RoomEffect::RegisterEvent {
        event_name: "bot_move_room".to_owned(),
    }));
    assert!(effects.contains(&RoomEffect::RegisterEvent {
        event_name: "user_status".to_owned(),
    }));
    assert_eq!(room.bot_ids(), &[100, 101]);
}

#[test]
fn manages_rights_and_doorbell_recipients() {
    let owner = details(7, "owner", 1);
    let controller = details(8, "controller", 1);
    let visitor = details(9, "visitor", 1);
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    room.load(vec![8]);

    assert!(room.has_rights(&owner, false, false));
    assert!(room.has_rights(&controller, false, false));
    assert!(!room.has_rights(&visitor, false, false));
    assert_eq!(
        room.ring_doorbell(
            &visitor,
            &[owner.clone(), controller.clone(), visitor.clone()]
        ),
        vec![
            RoomEffect::SendDoorbell {
                user_id: 7,
                username: "visitor".to_owned(),
            },
            RoomEffect::SendDoorbell {
                user_id: 8,
                username: "visitor".to_owned(),
            },
        ]
    );

    assert!(room
        .remove_user_rights(&controller)
        .contains(&RoomEffect::SaveRights {
            room_id: 12,
            rights: Vec::new(),
        }));
    assert!(room
        .give_user_rights(&visitor)
        .contains(&RoomEffect::SaveRights {
            room_id: 12,
            rights: vec![9],
        }));
}

#[test]
fn assigns_and_revokes_rights_only_from_owner_or_all_rights_sender() {
    let owner = details(7, "owner", 1);
    let guest = details(8, "guest", 1);
    let target = details(9, "target", 1);
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    room.load(Vec::new());

    assert!(room
        .assign_user_rights(&guest, Some(&target), false)
        .is_empty());
    assert!(room.rights().is_empty());

    assert!(room
        .assign_user_rights(&owner, Some(&target), false)
        .contains(&RoomEffect::SaveRights {
            room_id: 12,
            rights: vec![9],
        }));
    assert_eq!(room.rights(), &[9]);

    assert!(room
        .revoke_user_rights(&guest, Some(&target), false)
        .is_empty());
    assert_eq!(room.rights(), &[9]);

    assert!(room
        .revoke_user_rights(&guest, Some(&target), true)
        .contains(&RoomEffect::SaveRights {
            room_id: 12,
            rights: Vec::new(),
        }));
    assert!(room.rights().is_empty());
    assert!(room.assign_user_rights(&owner, None, false).is_empty());
    assert!(room.revoke_user_rights(&owner, None, false).is_empty());
}

#[test]
fn refreshes_owner_controller_and_guest_privileges() {
    let owner = details(7, "owner", 5);
    let controller = details(8, "controller", 3);
    let guest = details(9, "guest", 1);
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    room.load(vec![8]);

    assert!(room
        .refresh_flat_privileges(&owner, false, true)
        .contains(&RoomEffect::SendOwnerPrivileges { user_id: 7 }));
    assert!(room
        .refresh_flat_privileges(&controller, false, true)
        .contains(&RoomEffect::SendControllerPrivileges { user_id: 8 }));
    assert!(room
        .refresh_flat_privileges(&guest, false, false)
        .contains(&RoomEffect::MarkRoomUserForUpdate { user_id: 9 }));
}

#[test]
fn tries_flat_with_password_doorbell_and_rights_rules() {
    let owner = details(7, "owner", 1);
    let controller = details(8, "controller", 1);
    let visitor = details(9, "visitor", 1);
    let mut password_room = Room::new(room_data_with_state(
        RoomType::Private,
        7,
        "model_a",
        RoomState::Password.state_code(),
        "secret",
    ));
    password_room.load(vec![8]);

    assert_eq!(
        password_room.try_flat(&visitor, &[], "wrong", false),
        RoomEntryOutcome::IncorrectPassword
    );
    assert_eq!(
        password_room.try_flat(&visitor, &[], "secret", false),
        RoomEntryOutcome::LetIn
    );
    assert_eq!(
        password_room.try_flat(&controller, &[], "wrong", false),
        RoomEntryOutcome::LetIn
    );

    let mut doorbell_room = Room::new(room_data_with_state(
        RoomType::Private,
        7,
        "model_a",
        RoomState::Doorbell.state_code(),
        "",
    ));
    doorbell_room.load(vec![8]);

    assert_eq!(
        doorbell_room.try_flat(&visitor, &[visitor.clone()], "", false),
        RoomEntryOutcome::IncorrectPassword
    );
    assert_eq!(
        doorbell_room.try_flat(&visitor, &[owner, controller], "", false),
        RoomEntryOutcome::Doorbell(vec![
            RoomEffect::SendDoorbell {
                user_id: 7,
                username: "visitor".to_owned(),
            },
            RoomEffect::SendDoorbell {
                user_id: 8,
                username: "visitor".to_owned(),
            },
        ])
    );
}

#[test]
fn lets_waiting_user_in_only_when_sender_has_rights() {
    let owner = details(7, "owner", 1);
    let guest = details(8, "guest", 1);
    let waiting = details(9, "waiting", 1);
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    room.load(Vec::new());

    assert_eq!(
        room.let_user_in(&owner, Some(&waiting), false),
        vec![RoomEffect::LetUserIn {
            user_id: 9,
            room_id: 12,
        }]
    );
    assert!(room.let_user_in(&guest, Some(&waiting), false).is_empty());
    assert!(room.let_user_in(&owner, None, false).is_empty());
}

#[test]
fn kicks_room_user_only_with_rights_and_protected_target_permission() {
    let owner = details(7, "owner", 4);
    let guest = details(8, "guest", 1);
    let moderator = details(9, "moderator", 6);
    let protected = details(10, "protected", 5);
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    room.load(Vec::new());
    let player_manager = PlayerManager::new(vec![Permission::new("room_kick_any_user", true, 5)]);

    assert_eq!(
        room.kick_user(&player_manager, &owner, Some(&guest), false),
        vec![RoomEffect::KickUser { user_id: 8 }]
    );
    assert!(room
        .kick_user(&player_manager, &guest, Some(&owner), false)
        .is_empty());
    assert!(room
        .kick_user(&player_manager, &owner, Some(&protected), false)
        .is_empty());
    assert_eq!(
        room.kick_user(&player_manager, &moderator, Some(&protected), true),
        vec![RoomEffect::KickUser { user_id: 10 }]
    );
    assert!(room
        .kick_user(&player_manager, &owner, None, false)
        .is_empty());
}

#[test]
fn disposes_empty_private_room_when_owner_is_offline() {
    let mut room = Room::new(room_data(RoomType::Private, 7, "model_a"));
    let manager = PlayerManager::new(Vec::new());

    assert_eq!(
        room.dispose(false, &manager),
        vec![
            RoomEffect::ClearRuntimeData,
            RoomEffect::RemoveLoadedRoom { room_id: 12 },
        ]
    );
    assert!(room.is_disposed());
}
