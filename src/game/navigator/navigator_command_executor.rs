use crate::dao::{DaoError, NavigatorDao, RoomDao};
use crate::game::navigator::{NavigatorRequest, NavigatorSearchOutcome};
use crate::game::player::PlayerManager;
use crate::game::room::{RoomManager, RoomSummary};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigatorCommandExecutor;

impl NavigatorCommandExecutor {
    pub fn execute(
        effect: &IncomingExecutionEffect,
        navigator_dao: &dyn NavigatorDao,
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        server_ip: &str,
        private_server_port: u16,
    ) -> Result<Option<NavigatorSearchOutcome>, DaoError> {
        match effect {
            IncomingExecutionEffect::SearchBusyFlats { multiplier } => {
                let rooms = Self::busy_rooms(room_dao, room_manager, *multiplier)?;
                Ok(Some(NavigatorSearchOutcome::new(
                    NavigatorRequest::PopularRooms,
                    rooms,
                    server_ip,
                    private_server_port,
                )))
            }
            IncomingExecutionEffect::EmptySearchBusyFlats => Ok(Some(
                NavigatorSearchOutcome::empty(NavigatorRequest::PopularRooms),
            )),
            IncomingExecutionEffect::SearchFlat { query } => {
                let rooms = navigator_dao
                    .rooms_by_like_name(query)?
                    .into_iter()
                    .map(RoomSummary::new)
                    .collect::<Vec<_>>();
                Ok(Some(NavigatorSearchOutcome::new(
                    NavigatorRequest::SearchRooms,
                    rooms,
                    server_ip,
                    private_server_port,
                )))
            }
            IncomingExecutionEffect::SearchFlatForUser { username } => {
                let Some(session) = player_manager.get_by_name(username) else {
                    return Ok(None);
                };

                let rooms = room_dao
                    .player_rooms(session.details(), true)?
                    .into_iter()
                    .map(RoomSummary::new)
                    .collect::<Vec<_>>();
                Ok(Some(NavigatorSearchOutcome::new(
                    NavigatorRequest::PrivateRooms,
                    rooms,
                    server_ip,
                    private_server_port,
                )))
            }
            _ => Ok(None),
        }
    }

    fn busy_rooms(
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        multiplier: i32,
    ) -> Result<Vec<RoomSummary>, DaoError> {
        let mut rooms = room_manager
            .loaded_rooms()
            .values()
            .filter(|room| {
                room.data().room_type() == crate::game::room::settings::RoomType::Private
                    && !room.data().is_hidden()
                    && room.player_count() > 0
            })
            .cloned()
            .collect::<Vec<_>>();
        let loaded_ids = rooms
            .iter()
            .map(|room| room.data().id())
            .collect::<Vec<_>>();
        let range = if multiplier > 0 { multiplier / 11 } else { 0 };

        rooms.extend(
            room_dao
                .latest_player_rooms(&loaded_ids, range)?
                .into_iter()
                .map(RoomSummary::new),
        );
        rooms.sort_by(|left, right| right.player_count().cmp(&left.player_count()));

        let skip = usize::try_from(range).unwrap_or(0).min(rooms.len());
        Ok(rooms.into_iter().skip(skip).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::{InMemoryNavigatorDao, InMemoryRoomDao};
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::game::room::settings::RoomType;
    use crate::game::room::RoomData;

    fn private_room(id: i32, owner_id: i32, name: &str) -> RoomData {
        RoomData::new(
            id,
            false,
            RoomType::Private,
            owner_id,
            "alice",
            name,
            0,
            "",
            25,
            "desc",
            "model",
            "class",
            "wall",
            "floor",
            false,
            true,
        )
    }

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn outcome_packets(outcome: &NavigatorSearchOutcome) -> String {
        use crate::messages::OutgoingMessage;

        let mut packet = outcome.busy_flat_results().compose();
        packet.get()
    }

    #[test]
    fn executes_flat_name_search_through_navigator_dao() {
        let navigator_dao = InMemoryNavigatorDao::new(vec![private_room(10, 7, "Cafe")]);
        let room_dao = InMemoryRoomDao::new();
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        let outcome = NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::SearchFlat {
                query: "caf".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .unwrap();

        assert_eq!(outcome.request(), NavigatorRequest::SearchRooms);
        assert_eq!(outcome.rooms().len(), 1);
        assert!(outcome_packets(&outcome).contains("Cafe"));
    }

    #[test]
    fn executes_busy_flat_search_with_loaded_and_latest_rooms() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        room_dao.insert_room(private_room(20, 8, "Latest"));
        let mut room_manager = RoomManager::new();
        let mut loaded = RoomSummary::new(private_room(10, 7, "Loaded"));
        loaded.set_player_count(3);
        room_manager.add(loaded);
        let player_manager = PlayerManager::new(vec![]);

        let outcome = NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::SearchBusyFlats { multiplier: 0 },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .unwrap();

        assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
        assert_eq!(
            outcome
                .rooms()
                .iter()
                .map(|room| room.data().id())
                .collect::<Vec<_>>(),
            vec![10, 20]
        );
    }

    #[test]
    fn executes_empty_busy_flat_search_as_java_fallback_packet() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        let outcome = NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::EmptySearchBusyFlats,
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .unwrap();

        assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
        assert!(outcome.rooms().is_empty());
        assert_eq!(outcome_packets(&outcome), "#BUSY_FLAT_RESULTS 1##");
    }

    #[test]
    fn executes_online_user_room_search_through_room_dao() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        room_dao.insert_room(private_room(30, 9, "Owned"));
        let room_manager = RoomManager::new();
        let mut player_manager = PlayerManager::new(vec![]);
        player_manager.insert(PlayerSession::new(70, 30000, details(9, "Alice")));

        let outcome = NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::SearchFlatForUser {
                username: "alice".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .unwrap();

        assert_eq!(outcome.request(), NavigatorRequest::PrivateRooms);
        assert_eq!(outcome.rooms()[0].data().id(), 30);
    }

    #[test]
    fn ignores_missing_user_and_non_navigator_effects() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        assert!(NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::SearchFlatForUser {
                username: "alice".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .is_none());
        assert!(NavigatorCommandExecutor::execute(
            &IncomingExecutionEffect::GoAway,
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .is_none());
    }
}
