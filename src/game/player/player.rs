use crate::game::entity::{Entity, EntityType};
use crate::game::inventory::Inventory;
use crate::game::messenger::Messenger;
use crate::game::player::{PlayerDetails, PlayerEffect, PlayerManager, PlayerSession};
use crate::game::room::entity::RoomUser;
use crate::messages::outgoing::SystemBroadcast;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    machine_id: Option<String>,
    details: PlayerDetails,
    connection_id: i32,
    server_port: u16,
    inventory: Inventory,
    messenger: Messenger,
    current_room_id: Option<i32>,
    last_created_room_id: Option<i32>,
    send_hotel_alert: bool,
    order_info_protection: i64,
}

impl Player {
    pub fn new(connection_id: i32, server_port: u16) -> Self {
        Self::with_details(connection_id, server_port, PlayerDetails::new())
    }

    pub fn with_details(connection_id: i32, server_port: u16, details: PlayerDetails) -> Self {
        let user_id = details.id();
        Self {
            machine_id: None,
            details,
            connection_id,
            server_port,
            inventory: Inventory::new(),
            messenger: Messenger::new(user_id),
            current_room_id: None,
            last_created_room_id: None,
            send_hotel_alert: true,
            order_info_protection: 0,
        }
    }

    pub fn login(&self, _initial_authentication: bool) -> Vec<PlayerEffect> {
        vec![PlayerEffect::UpdateLastLogin {
            user_id: self.details.id(),
        }]
    }

    pub fn has_permission(&self, manager: &PlayerManager, permission: &str) -> bool {
        manager.has_permission(self.details.rank(), permission)
    }

    pub fn send_alert(&self, message: impl Into<String>) -> PlayerEffect {
        PlayerEffect::SendAlert(SystemBroadcast::new(message))
    }

    pub fn kick(&self) -> PlayerEffect {
        PlayerEffect::CloseConnection {
            connection_id: self.connection_id,
        }
    }

    pub fn kick_all_connections(&self) -> PlayerEffect {
        PlayerEffect::CloseUserConnections {
            user_id: self.details.id(),
        }
    }

    pub fn dispose(&mut self, main_server_port: u16) -> Vec<PlayerEffect> {
        if self.server_port == main_server_port {
            let mut effects = vec![
                PlayerEffect::DisposeOwnedRooms {
                    user_id: self.details.id(),
                },
                PlayerEffect::DisposeInventory {
                    user_id: self.details.id(),
                },
            ];
            effects.extend(
                self.messenger
                    .dispose()
                    .into_iter()
                    .map(PlayerEffect::Messenger),
            );
            self.inventory.dispose();
            effects
        } else if self.current_room_id.is_some() {
            self.current_room_id = None;
            vec![PlayerEffect::LeaveCurrentRoom {
                connection_id: self.connection_id,
            }]
        } else {
            Vec::new()
        }
    }

    pub fn main_server_session<'a>(
        &self,
        manager: &'a PlayerManager,
        main_server_port: i32,
    ) -> Option<&'a PlayerSession> {
        manager.get_by_id_on_port(self.details.id(), main_server_port)
    }

    pub fn private_room_session<'a>(
        &self,
        manager: &'a PlayerManager,
        private_server_port: i32,
    ) -> Option<&'a PlayerSession> {
        manager.get_private_room_player(self.details.id(), private_server_port)
    }

    pub fn public_room_session<'a>(
        &self,
        manager: &'a PlayerManager,
        main_server_port: i32,
        private_server_port: i32,
    ) -> Option<&'a PlayerSession> {
        manager.players().values().find(|session| {
            session.details().id() == self.details.id()
                && session.server_port() != main_server_port
                && session.server_port() != private_server_port
        })
    }

    pub fn entity_type(&self) -> EntityType {
        EntityType::Player
    }

    pub fn connection_id(&self) -> i32 {
        self.connection_id
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }

    pub fn machine_id(&self) -> Option<&str> {
        self.machine_id.as_deref()
    }

    pub fn set_machine_id(&mut self, machine_id: impl Into<String>) {
        self.machine_id = Some(machine_id.into());
    }

    pub fn details(&self) -> &PlayerDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.details
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn messenger(&self) -> &Messenger {
        &self.messenger
    }

    pub fn messenger_mut(&mut self) -> &mut Messenger {
        &mut self.messenger
    }

    pub fn current_room_id(&self) -> Option<i32> {
        self.current_room_id
    }

    pub fn set_current_room_id(&mut self, room_id: Option<i32>) {
        self.current_room_id = room_id;
    }

    pub fn last_created_room_id(&self) -> Option<i32> {
        self.last_created_room_id
    }

    pub fn set_last_created_room_id(&mut self, room_id: Option<i32>) {
        self.last_created_room_id = room_id;
    }

    pub fn can_send_hotel_alert(&self) -> bool {
        self.send_hotel_alert
    }

    pub fn set_send_hotel_alert(&mut self, send_hotel_alert: bool) {
        self.send_hotel_alert = send_hotel_alert;
    }

    pub fn order_info_protection(&self) -> i64 {
        self.order_info_protection
    }

    pub fn set_order_info_protection(&mut self, order_info_protection: i64) {
        self.order_info_protection = order_info_protection;
    }
}

impl Entity for Player {
    fn details(&self) -> &PlayerDetails {
        self.details()
    }

    fn room_user(&self) -> Option<&RoomUser> {
        None
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::messenger::{MessengerEffect, MessengerFriend};
    use crate::game::player::Permission;
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
}
