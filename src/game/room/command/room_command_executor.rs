use crate::dao::{CreateRoom, DaoError, RoomDao};
use crate::game::player::PlayerDetails;
use crate::game::room::{RoomCommandOutcome, RoomData};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomCommandExecutor;

impl RoomCommandExecutor {
    pub fn create_flat(
        room_dao: &dyn RoomDao,
        owner: &PlayerDetails,
        request: CreateFlatRequest<'_>,
    ) -> Result<RoomCommandExecution, DaoError> {
        let room = room_dao.create_room(&CreateRoom::new(
            owner,
            request.room_name,
            "",
            request.room_model,
            request.state,
            request.show_owner_name,
        ))?;

        Ok(RoomCommandExecution::Created(room.clone()))
    }

    pub fn get_flat_info(
        room_dao: &dyn RoomDao,
        room_id: i32,
    ) -> Result<RoomCommandExecution, DaoError> {
        let Some(room) = room_dao.room(room_id, false)? else {
            return Ok(RoomCommandExecution::Ignored);
        };

        Ok(RoomCommandExecution::FlatInfo(room))
    }

    pub fn delete_flat(
        room_dao: &dyn RoomDao,
        room_id: i32,
        has_owner_rights: bool,
    ) -> Result<RoomCommandExecution, DaoError> {
        if !has_owner_rights {
            return Ok(RoomCommandExecution::Ignored);
        }

        let Some(room) = room_dao.room(room_id, false)? else {
            return Ok(RoomCommandExecution::Ignored);
        };

        room_dao.delete_room(&room)?;
        Ok(RoomCommandExecution::Deleted { room_id })
    }

    pub fn update_flat(
        room_dao: &dyn RoomDao,
        request: UpdateFlatRequest<'_>,
    ) -> Result<RoomCommandExecution, DaoError> {
        if !request.has_owner_rights {
            return Ok(RoomCommandExecution::Ignored);
        }

        let Some(mut room) = room_dao.room(request.room_id, false)? else {
            return Ok(RoomCommandExecution::Ignored);
        };

        let room_name = if request.room_name.chars().count() > 2 {
            request.room_name
        } else {
            room.name()
        }
        .to_owned();

        room.set_name(room_name);
        room.set_state(request.state);
        room.set_show_owner_name(request.show_owner_name);
        room_dao.update_room(&room)?;
        Ok(RoomCommandExecution::Updated(room))
    }

    pub fn set_flat_info(
        room_dao: &dyn RoomDao,
        request: SetFlatInfoRequest<'_>,
    ) -> Result<RoomCommandExecution, DaoError> {
        if !request.has_owner_rights {
            return Ok(RoomCommandExecution::Ignored);
        }

        let Some(mut room) = room_dao.room(request.room_id, false)? else {
            return Ok(RoomCommandExecution::Ignored);
        };

        let description = if request.description.chars().count() > 2 {
            request.description
        } else {
            room.description()
        }
        .to_owned();

        room.set_description(description);
        room.set_password(request.password);
        room.set_all_super_user(request.all_super_user);
        room_dao.update_room(&room)?;
        Ok(RoomCommandExecution::Updated(room))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateFlatRequest<'a> {
    pub room_name: &'a str,
    pub room_model: &'a str,
    pub state: i32,
    pub show_owner_name: bool,
}

impl<'a> CreateFlatRequest<'a> {
    pub fn new(room_name: &'a str, room_model: &'a str, state: i32, show_owner_name: bool) -> Self {
        Self {
            room_name,
            room_model,
            state,
            show_owner_name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFlatRequest<'a> {
    pub room_id: i32,
    pub room_name: &'a str,
    pub state: i32,
    pub show_owner_name: bool,
    pub has_owner_rights: bool,
}

impl<'a> UpdateFlatRequest<'a> {
    pub fn new(
        room_id: i32,
        room_name: &'a str,
        state: i32,
        show_owner_name: bool,
        has_owner_rights: bool,
    ) -> Self {
        Self {
            room_id,
            room_name,
            state,
            show_owner_name,
            has_owner_rights,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetFlatInfoRequest<'a> {
    pub room_id: i32,
    pub description: &'a str,
    pub password: &'a str,
    pub all_super_user: bool,
    pub has_owner_rights: bool,
}

impl<'a> SetFlatInfoRequest<'a> {
    pub fn new(
        room_id: i32,
        description: &'a str,
        password: &'a str,
        all_super_user: bool,
        has_owner_rights: bool,
    ) -> Self {
        Self {
            room_id,
            description,
            password,
            all_super_user,
            has_owner_rights,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomCommandExecution {
    Created(RoomData),
    FlatInfo(RoomData),
    Updated(RoomData),
    Deleted { room_id: i32 },
    Ignored,
}

impl RoomCommandExecution {
    pub fn command_outcome(&self) -> RoomCommandOutcome {
        match self {
            Self::Created(room) => RoomCommandOutcome::created(room),
            Self::FlatInfo(room) => RoomCommandOutcome::flat_info(room),
            Self::Updated(_) | Self::Deleted { .. } | Self::Ignored => RoomCommandOutcome::Ignored,
        }
    }
}
