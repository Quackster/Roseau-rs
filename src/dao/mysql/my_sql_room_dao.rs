use std::collections::{HashMap, HashSet};

use crate::dao::mysql::{
    RoomQueries, RoomResultMapper, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::{CreateRoom, DaoError, RoomChatlog, RoomDao};
use crate::game::player::{Bot, PlayerDetails};
use crate::game::room::model::RoomModel;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomConnection, RoomData};

#[derive(Debug)]
pub struct MySqlRoomDao<E> {
    executor: E,
    owner_name: String,
    models: HashMap<String, RoomModel>,
    now: i64,
}

impl<E> MySqlRoomDao<E> {
    pub fn new(
        executor: E,
        owner_name: impl Into<String>,
        models: HashMap<String, RoomModel>,
        now: i64,
    ) -> Self {
        Self {
            executor,
            owner_name: owner_name.into(),
            models,
            now,
        }
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn owner_name(&self) -> &str {
        &self.owner_name
    }

    pub fn models(&self) -> &HashMap<String, RoomModel> {
        &self.models
    }

    pub fn set_now(&mut self, now: i64) {
        self.now = now;
    }
}

impl<E: SqlExecutor> MySqlRoomDao<E> {
    fn execute_plan(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let result = self.executor.execute(plan.clone())?;
        plan.validate_result(result)
    }

    fn execute_mutation(&self, plan: SqlExecutionPlan) -> Result<(), DaoError> {
        self.execute_plan(plan)?.require_mutation()
    }
}

impl<E: SqlExecutor> RoomDao for MySqlRoomDao<E> {
    fn public_rooms(&self, _store_in_memory: bool) -> Result<Vec<RoomData>, DaoError> {
        let result = self.execute_plan(RoomQueries::public_rooms().read_plan())?;
        RoomResultMapper::rooms(result, &self.owner_name)
    }

    fn player_rooms(
        &self,
        details: &PlayerDetails,
        _store_in_memory: bool,
    ) -> Result<Vec<RoomData>, DaoError> {
        let result = self.execute_plan(RoomQueries::player_rooms(details.id()).read_plan())?;
        RoomResultMapper::rooms(result, details.username())
    }

    fn room(&self, room_id: i32, _store_in_memory: bool) -> Result<Option<RoomData>, DaoError> {
        let result = self.execute_plan(RoomQueries::room(room_id).read_plan())?;
        RoomResultMapper::optional_room(result, &self.owner_name)
    }

    fn room_rights(&self, room_id: i32) -> Result<Vec<i32>, DaoError> {
        let result = self.execute_plan(RoomQueries::room_rights(room_id).read_plan())?;
        RoomResultMapper::room_rights(result)
    }

    fn update_room(&self, room: &RoomData) -> Result<(), DaoError> {
        self.execute_mutation(RoomQueries::update_room(room).execute_plan())
    }

    fn model(&self, model: &str) -> Result<Option<RoomModel>, DaoError> {
        Ok(self.models.get(model).cloned())
    }

    fn delete_room(&self, room: &RoomData) -> Result<(), DaoError> {
        self.execute_mutation(RoomQueries::delete_room(room.id()).execute_plan())
    }

    fn create_room(&self, room: &CreateRoom) -> Result<RoomData, DaoError> {
        let result =
            self.execute_plan(RoomQueries::create_room(room).insert_returning_id_plan())?;
        let room_id = RoomResultMapper::created_room_id(result)?;
        Ok(RoomData::new(
            room_id,
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
        ))
    }

    fn room_connections(&self, room_id: i32) -> Result<Vec<RoomConnection>, DaoError> {
        let result = self.execute_plan(RoomQueries::room_connections(room_id).read_plan())?;
        RoomResultMapper::room_connections(result)
    }

    fn bots(&self, room_id: i32) -> Result<Vec<Bot>, DaoError> {
        let result = self.execute_plan(RoomQueries::bots(room_id).read_plan())?;
        RoomResultMapper::bots(result)
    }

    fn save_room_rights(&self, room_id: i32, rights: &[i32]) -> Result<(), DaoError> {
        self.execute_mutation(RoomQueries::delete_room_rights(room_id).execute_plan())?;
        let mut deduped = rights
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        deduped.sort_unstable();
        for user_id in deduped {
            self.execute_mutation(RoomQueries::insert_room_right(room_id, user_id).execute_plan())?;
        }
        Ok(())
    }

    fn save_chatlog(&self, chatlog: &RoomChatlog) -> Result<(), DaoError> {
        self.execute_mutation(RoomQueries::save_chatlog(chatlog, self.now).execute_plan())
    }

    fn public_room_ids(&self) -> Result<Vec<i32>, DaoError> {
        let result = self.execute_plan(RoomQueries::public_room_ids().read_plan())?;
        RoomResultMapper::public_room_ids(result)
    }

    fn latest_player_rooms(
        &self,
        blacklist: &[i32],
        multiplier: i32,
    ) -> Result<Vec<RoomData>, DaoError> {
        let result = self.execute_plan(RoomQueries::latest_player_rooms(multiplier).read_plan())?;
        let blacklist = blacklist.iter().copied().collect::<HashSet<_>>();
        Ok(RoomResultMapper::rooms(result, &self.owner_name)?
            .into_iter()
            .filter(|room| !blacklist.contains(&room.id()))
            .collect())
    }
}
