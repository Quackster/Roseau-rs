use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};

use crate::dao::{CreateRoom, DaoError, RoomChatlog, RoomDao};
use crate::game::player::{Bot, PlayerDetails};
use crate::game::room::model::RoomModel;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomConnection, RoomData};

#[derive(Debug, Default)]
pub struct InMemoryRoomDao {
    rooms: RefCell<HashMap<i32, RoomData>>,
    room_models: RefCell<HashMap<String, RoomModel>>,
    rights: RefCell<HashMap<i32, Vec<i32>>>,
    connections: RefCell<HashMap<i32, Vec<RoomConnection>>>,
    bots: RefCell<HashMap<i32, Vec<Bot>>>,
    chatlogs: RefCell<Vec<RoomChatlog>>,
    next_room_id: Cell<i32>,
}

impl InMemoryRoomDao {
    pub fn new() -> Self {
        Self {
            rooms: RefCell::new(HashMap::new()),
            room_models: RefCell::new(HashMap::new()),
            rights: RefCell::new(HashMap::new()),
            connections: RefCell::new(HashMap::new()),
            bots: RefCell::new(HashMap::new()),
            chatlogs: RefCell::new(Vec::new()),
            next_room_id: Cell::new(1),
        }
    }

    pub fn insert_room(&self, room: RoomData) {
        let id = room.id();
        self.next_room_id.set(self.next_room_id.get().max(id + 1));
        self.rooms.borrow_mut().insert(id, room);
    }

    pub fn insert_model(&self, model: RoomModel) {
        self.room_models
            .borrow_mut()
            .insert(model.name().to_owned(), model);
    }

    pub fn insert_connections(&self, room_id: i32, connections: Vec<RoomConnection>) {
        self.connections.borrow_mut().insert(room_id, connections);
    }

    pub fn insert_bots(&self, room_id: i32, bots: Vec<Bot>) {
        self.bots.borrow_mut().insert(room_id, bots);
    }

    pub fn chatlogs(&self) -> Vec<RoomChatlog> {
        self.chatlogs.borrow().clone()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.borrow().is_empty()
            && self.room_models.borrow().is_empty()
            && self.rights.borrow().is_empty()
            && self.connections.borrow().is_empty()
            && self.bots.borrow().is_empty()
            && self.chatlogs.borrow().is_empty()
    }

    fn next_room_id(&self) -> i32 {
        let id = self.next_room_id.get();
        self.next_room_id.set(id + 1);
        id
    }
}

impl RoomDao for InMemoryRoomDao {
    fn public_rooms(&self, _store_in_memory: bool) -> Result<Vec<RoomData>, DaoError> {
        let mut rooms = self
            .rooms
            .borrow()
            .values()
            .filter(|room| room.room_type() == RoomType::Public)
            .cloned()
            .collect::<Vec<_>>();
        rooms.sort_by_key(RoomData::id);
        Ok(rooms)
    }

    fn player_rooms(
        &self,
        details: &PlayerDetails,
        _store_in_memory: bool,
    ) -> Result<Vec<RoomData>, DaoError> {
        let mut rooms = self
            .rooms
            .borrow()
            .values()
            .filter(|room| room.owner_id() == details.id())
            .cloned()
            .collect::<Vec<_>>();
        rooms.sort_by_key(RoomData::id);
        Ok(rooms)
    }

    fn room(&self, room_id: i32, _store_in_memory: bool) -> Result<Option<RoomData>, DaoError> {
        Ok(self.rooms.borrow().get(&room_id).cloned())
    }

    fn room_rights(&self, room_id: i32) -> Result<Vec<i32>, DaoError> {
        Ok(self
            .rights
            .borrow()
            .get(&room_id)
            .cloned()
            .unwrap_or_default())
    }

    fn update_room(&self, room: &RoomData) -> Result<(), DaoError> {
        if self.rooms.borrow().contains_key(&room.id()) {
            self.rooms.borrow_mut().insert(room.id(), room.clone());
        }
        Ok(())
    }

    fn model(&self, model: &str) -> Result<Option<RoomModel>, DaoError> {
        Ok(self.room_models.borrow().get(model).cloned())
    }

    fn delete_room(&self, room: &RoomData) -> Result<(), DaoError> {
        self.rooms.borrow_mut().remove(&room.id());
        self.rights.borrow_mut().remove(&room.id());
        self.connections.borrow_mut().remove(&room.id());
        self.bots.borrow_mut().remove(&room.id());
        Ok(())
    }

    fn create_room(&self, room: &CreateRoom) -> Result<RoomData, DaoError> {
        let data = RoomData::new(
            self.next_room_id(),
            false,
            RoomType::Private,
            room.owner_id,
            &room.owner_name,
            &room.name,
            room.state,
            "",
            25,
            &room.description,
            &room.model,
            "default",
            "",
            "",
            false,
            room.show_owner_name,
        );
        self.insert_room(data.clone());
        Ok(data)
    }

    fn room_connections(&self, room_id: i32) -> Result<Vec<RoomConnection>, DaoError> {
        Ok(self
            .connections
            .borrow()
            .get(&room_id)
            .cloned()
            .unwrap_or_default())
    }

    fn bots(&self, room_id: i32) -> Result<Vec<Bot>, DaoError> {
        Ok(self
            .bots
            .borrow()
            .get(&room_id)
            .cloned()
            .unwrap_or_default())
    }

    fn save_room_rights(&self, room_id: i32, rights: &[i32]) -> Result<(), DaoError> {
        let mut deduped = rights
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        deduped.sort_unstable();
        self.rights.borrow_mut().insert(room_id, deduped);
        Ok(())
    }

    fn save_chatlog(&self, chatlog: &RoomChatlog) -> Result<(), DaoError> {
        self.chatlogs.borrow_mut().push(chatlog.clone());
        Ok(())
    }

    fn public_room_ids(&self) -> Result<Vec<i32>, DaoError> {
        let mut ids = self
            .rooms
            .borrow()
            .values()
            .filter(|room| room.room_type() == RoomType::Public && !room.is_hidden())
            .map(RoomData::id)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        Ok(ids)
    }

    fn latest_player_rooms(
        &self,
        blacklist: &[i32],
        multiplier: i32,
    ) -> Result<Vec<RoomData>, DaoError> {
        let blacklist = blacklist.iter().copied().collect::<HashSet<_>>();
        let offset = (multiplier.max(0) as usize) * 11;
        let mut rooms = self
            .rooms
            .borrow()
            .values()
            .filter(|room| room.room_type() == RoomType::Private && !blacklist.contains(&room.id()))
            .cloned()
            .collect::<Vec<_>>();
        rooms.sort_by_key(|room| std::cmp::Reverse(room.id()));
        Ok(rooms.into_iter().skip(offset).take(11).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::model::Position;

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
            "model_a",
            "default",
            "wall",
            "floor",
            false,
            true,
        )
    }

    fn public_room(id: i32, hidden: bool, name: &str) -> RoomData {
        RoomData::new(
            id,
            hidden,
            RoomType::Public,
            0,
            "",
            name,
            0,
            "",
            100,
            "desc",
            "model_a",
            "default",
            "",
            "",
            false,
            false,
        )
    }

    #[test]
    fn stores_and_queries_rooms_by_type_owner_and_latest() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(private_room(1, 7, "Old"));
        dao.insert_room(public_room(2, false, "Lobby"));
        dao.insert_room(private_room(3, 7, "New"));

        let mut details = PlayerDetails::new();
        details.fill_basic(7, "alice", "hello", "hd-100");

        assert_eq!(dao.public_rooms(false).unwrap()[0].id(), 2);
        assert_eq!(dao.public_room_ids().unwrap(), vec![2]);
        assert_eq!(dao.player_rooms(&details, false).unwrap().len(), 2);
        assert_eq!(dao.latest_player_rooms(&[1], 0).unwrap()[0].id(), 3);
    }

    #[test]
    fn creates_updates_and_deletes_rooms() {
        let dao = InMemoryRoomDao::new();
        let mut owner = PlayerDetails::new();
        owner.fill_basic(7, "alice", "hello", "hd-100");
        let created = dao
            .create_room(&CreateRoom::new(&owner, "Room", "Desc", "model_a", 1, true))
            .unwrap();

        assert_eq!(created.id(), 1);
        assert_eq!(created.owner_id(), 7);

        let mut changed = created.clone();
        changed.set_name("Changed");
        dao.update_room(&changed).unwrap();
        assert_eq!(
            dao.room(created.id(), false).unwrap().unwrap().name(),
            "Changed"
        );

        dao.delete_room(&changed).unwrap();
        assert!(dao.room(created.id(), false).unwrap().is_none());
    }

    #[test]
    fn stores_rights_connections_models_bots_and_chatlogs() {
        let dao = InMemoryRoomDao::new();
        let model = RoomModel::new("model_a", "00 00", 0, 0, 0, 2, false, false).unwrap();
        dao.insert_model(model);
        dao.insert_connections(1, vec![RoomConnection::new(1, 2, Position::new(3, 4, 0.0))]);
        dao.insert_bots(
            1,
            vec![Bot::new(Position::new(1, 1, 0.0), vec![], vec![], vec![])],
        );
        dao.save_room_rights(1, &[7, 7, 8]).unwrap();
        dao.save_chatlog(&RoomChatlog::new("alice", 1, "CHAT", "hello"))
            .unwrap();

        assert!(dao.model("model_a").unwrap().is_some());
        assert_eq!(dao.room_connections(1).unwrap()[0].to_id(), 2);
        assert_eq!(dao.bots(1).unwrap().len(), 1);
        assert_eq!(dao.room_rights(1).unwrap(), vec![7, 8]);
        assert_eq!(dao.chatlogs()[0].message, "hello");
    }
}
