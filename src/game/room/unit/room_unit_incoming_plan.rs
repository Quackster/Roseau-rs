use crate::game::player::PlayerManager;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomManager, RoomSummary, RoomUnitOutcome};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUnitIncomingPlan;

impl RoomUnitIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        base_port: i32,
    ) -> Vec<RoomUnitOutcome> {
        match effect {
            IncomingExecutionEffect::InitUnitListener => {
                vec![RoomUnitOutcome::listener(public_rooms(room_manager))]
            }
            IncomingExecutionEffect::GetUnitUsers { room_name } => {
                vec![unit_members(
                    room_manager,
                    player_manager,
                    room_name,
                    base_port,
                )]
            }
            _ => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        base_port: i32,
    ) -> Vec<RoomUnitOutcome> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room_manager, player_manager, base_port))
            .collect()
    }
}

fn public_rooms(room_manager: &RoomManager) -> Vec<RoomSummary> {
    room_manager
        .get_public_rooms()
        .into_iter()
        .cloned()
        .collect()
}

fn unit_members(
    room_manager: &RoomManager,
    player_manager: &PlayerManager,
    room_name: &str,
    base_port: i32,
) -> RoomUnitOutcome {
    let Some(room) = room_manager.get_public_rooms().into_iter().find(|room| {
        room.data().name() == room_name
            && room.data().room_type() == RoomType::Public
            && !room.data().is_hidden()
    }) else {
        return RoomUnitOutcome::missing_room();
    };

    let room_port = room.data().server_port(base_port);
    let mut sessions = player_manager
        .players()
        .values()
        .filter(|session| session.server_port() == room_port)
        .collect::<Vec<_>>();
    sessions.sort_by_key(|session| session.connection_id());

    RoomUnitOutcome::unit_members(
        [room.clone()],
        sessions
            .into_iter()
            .map(|session| session.details().username().to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::game::room::RoomData;
    use crate::messages::OutgoingMessage;

    fn public_room(id: i32, name: &str, class_name: &str, order_id: i32) -> RoomSummary {
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
        room.set_order_id(order_id);
        room
    }

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    #[test]
    fn plans_unit_listener_from_loaded_public_rooms() {
        let mut room_manager = RoomManager::new();
        room_manager.add(public_room(5, "Habbo Lido", "lido", 2));
        room_manager.add(public_room(4, "Club Massiva", "club", 1));
        let player_manager = PlayerManager::new(vec![]);

        let outcomes = RoomUnitIncomingPlan::plan(
            &IncomingExecutionEffect::InitUnitListener,
            &room_manager,
            &player_manager,
            22004,
        );
        let mut all_units = outcomes[0].all_units("127.0.0.1", 22004).unwrap().compose();

        assert_eq!(outcomes.len(), 1);
        assert!(all_units.get().contains("Club Massiva"));
        assert!(all_units.get().contains("Habbo Lido"));
        assert!(outcomes[0].unit_members_packet().is_none());
    }

    #[test]
    fn plans_unit_members_for_named_public_room() {
        let mut room_manager = RoomManager::new();
        room_manager.add(public_room(5, "Habbo Lido", "lido", 1));
        let mut player_manager = PlayerManager::new(vec![]);
        player_manager.insert(PlayerSession::new(70, 22009, details(7, "alice")));
        player_manager.insert(PlayerSession::new(80, 22009, details(8, "bob")));
        player_manager.insert(PlayerSession::new(90, 22010, details(9, "carol")));

        let outcomes = RoomUnitIncomingPlan::plan(
            &IncomingExecutionEffect::GetUnitUsers {
                room_name: "Habbo Lido".to_owned(),
            },
            &room_manager,
            &player_manager,
            22004,
        );
        let mut all_units = outcomes[0].all_units("10.0.0.1", 22004).unwrap().compose();
        let mut unit_members = outcomes[0].unit_members_packet().unwrap().compose();

        assert_eq!(outcomes.len(), 1);
        assert!(all_units.get().contains("Habbo Lido"));
        assert_eq!(unit_members.get(), "#UNITMEMBERS\ralice\rbob##");
    }

    #[test]
    fn plans_missing_room_for_unknown_unit_members_request() {
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        let outcomes = RoomUnitIncomingPlan::plan(
            &IncomingExecutionEffect::GetUnitUsers {
                room_name: "Missing".to_owned(),
            },
            &room_manager,
            &player_manager,
            22004,
        );

        assert_eq!(outcomes, vec![RoomUnitOutcome::missing_room()]);
        assert!(outcomes[0].all_units("127.0.0.1", 22004).is_none());
    }

    #[test]
    fn ignores_unrelated_unit_effects() {
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        assert!(RoomUnitIncomingPlan::plan(
            &IncomingExecutionEffect::GoAway,
            &room_manager,
            &player_manager,
            22004,
        )
        .is_empty());
    }
}
