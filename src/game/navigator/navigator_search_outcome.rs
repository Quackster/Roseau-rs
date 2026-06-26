use crate::game::navigator::NavigatorRequest;
use crate::game::room::{RoomNavigatorEntry, RoomSummary};
use crate::messages::outgoing::BusyFlatResults;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigatorSearchOutcome {
    request: NavigatorRequest,
    rooms: Vec<RoomNavigatorEntry>,
}

impl NavigatorSearchOutcome {
    pub fn new(
        request: NavigatorRequest,
        rooms: impl IntoIterator<Item = RoomSummary>,
        server_ip: impl AsRef<str>,
        private_server_port: u16,
    ) -> Self {
        let server_ip = server_ip.as_ref();
        Self {
            request,
            rooms: rooms
                .into_iter()
                .map(|room| {
                    RoomNavigatorEntry::new(
                        room.data().clone(),
                        request,
                        server_ip,
                        private_server_port,
                        users_now(room.player_count()),
                    )
                })
                .collect(),
        }
    }

    pub fn empty(request: NavigatorRequest) -> Self {
        Self {
            request,
            rooms: Vec::new(),
        }
    }

    pub fn busy_flat_results(&self) -> BusyFlatResults<RoomNavigatorEntry> {
        BusyFlatResults::new(self.rooms.clone())
    }

    pub fn request(&self) -> NavigatorRequest {
        self.request
    }

    pub fn rooms(&self) -> &[RoomNavigatorEntry] {
        &self.rooms
    }
}

fn users_now(player_count: usize) -> i32 {
    i32::try_from(player_count).unwrap_or(i32::MAX)
}

#[cfg(test)]
#[path = "navigator_search_outcome_tests.rs"]
mod tests;
