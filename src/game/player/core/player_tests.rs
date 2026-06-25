use crate::game::entity::EntityType;
use crate::game::messenger::{MessengerEffect, MessengerFriend};
use crate::game::player::{
    Permission, Player, PlayerDetails, PlayerEffect, PlayerManager, PlayerSession,
};
use crate::messages::OutgoingMessage;

fn details(id: i32, username: &str, rank: i32) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        id, username, "mission", "figure", "pool", "email", rank, 10, "M", "GB", "badge",
        "birthday", 0, "hello", 3,
    );
    details
}

#[test]
fn stores_player_state_and_emits_alert_and_kick_effects() {
    let mut player = Player::with_details(42, 30000, details(7, "alice", 5));
    player.set_machine_id("machine");
    player.set_current_room_id(Some(99));
    player.set_last_created_room_id(Some(12));
    player.set_order_info_protection(123);
    player.set_send_hotel_alert(false);

    assert_eq!(player.entity_type(), EntityType::Player);
    assert_eq!(player.machine_id(), Some("machine"));
    assert_eq!(player.current_room_id(), Some(99));
    assert_eq!(player.last_created_room_id(), Some(12));
    assert_eq!(player.order_info_protection(), 123);
    assert!(!player.can_send_hotel_alert());
    assert_eq!(
        player.kick(),
        PlayerEffect::CloseConnection { connection_id: 42 }
    );

    let PlayerEffect::SendAlert(packet) = player.send_alert("maintenance") else {
        panic!("expected alert packet");
    };
    let mut response = packet.compose();
    assert_eq!(response.get(), "#SYSTEMBROADCAST\rmaintenance##");
}

#[test]
fn login_plans_last_login_update_like_java_player() {
    let player = Player::with_details(42, 30000, details(7, "alice", 5));

    assert_eq!(
        player.login(true),
        vec![PlayerEffect::UpdateLastLogin { user_id: 7 }]
    );
    assert_eq!(
        player.login(false),
        vec![PlayerEffect::UpdateLastLogin { user_id: 7 }]
    );
}

#[test]
fn checks_permissions_and_finds_matching_sessions() {
    let player = Player::with_details(42, 30000, details(7, "alice", 5));
    let mut manager = PlayerManager::new(vec![Permission::new("room_admin", true, 4)]);
    manager.insert(PlayerSession::new(1, 30000, details(7, "alice-main", 5)));
    manager.insert(PlayerSession::new(2, 30001, details(7, "alice-private", 5)));
    manager.insert(PlayerSession::new(3, 40000, details(7, "alice-public", 5)));

    assert!(player.has_permission(&manager, "room_admin"));
    assert_eq!(
        player
            .main_server_session(&manager, 30000)
            .unwrap()
            .connection_id(),
        1
    );
    assert_eq!(
        player
            .private_room_session(&manager, 30001)
            .unwrap()
            .connection_id(),
        2
    );
    assert_eq!(
        player
            .public_room_session(&manager, 30000, 30001)
            .unwrap()
            .connection_id(),
        3
    );
}

#[test]
fn disposal_clears_main_server_inventory_and_messenger_state() {
    let mut player = Player::with_details(42, 30000, details(7, "alice", 5));
    player.messenger_mut().load(
        vec![MessengerFriend::new(
            8,
            "bob",
            "hello",
            Some("room".to_owned()),
            10,
            true,
            true,
        )],
        Vec::new(),
    );

    let effects = player.dispose(30000);

    assert_eq!(
        effects,
        vec![
            PlayerEffect::DisposeOwnedRooms { user_id: 7 },
            PlayerEffect::DisposeInventory { user_id: 7 },
            PlayerEffect::Messenger(MessengerEffect::RefreshFriendList {
                user_id: 8,
                offline_user_id: Some(7),
            }),
        ]
    );
    assert!(player.inventory().items().is_empty());
    assert!(player.messenger().friends().is_empty());
}

#[test]
fn disposal_leaves_current_room_on_room_server() {
    let mut player = Player::with_details(42, 40000, details(7, "alice", 5));
    player.set_current_room_id(Some(99));

    assert_eq!(
        player.dispose(30000),
        vec![PlayerEffect::LeaveCurrentRoom { connection_id: 42 }]
    );
    assert_eq!(player.current_room_id(), None);
}
