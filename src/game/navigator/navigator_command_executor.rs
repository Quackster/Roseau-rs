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
#[path = "navigator_command_executor_tests.rs"]
mod tests;
