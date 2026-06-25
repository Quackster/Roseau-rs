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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryRoomDao;
    use crate::dao::RoomDao;
    use crate::game::room::settings::{RoomState, RoomType};

    fn owner() -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(7, "alice", "hello", "hd=100");
        details
    }

    fn room(id: i32) -> RoomData {
        RoomData::new(
            id,
            false,
            RoomType::Private,
            7,
            "alice",
            "Old room",
            0,
            "",
            25,
            "old desc",
            "model_a",
            "default",
            "wall",
            "floor",
            false,
            true,
        )
    }

    #[test]
    fn creates_room_and_maps_to_flat_created_outcome() {
        let dao = InMemoryRoomDao::new();

        let execution = RoomCommandExecutor::create_flat(
            &dao,
            &owner(),
            CreateFlatRequest::new("Tea Room", "model_b", 1, false),
        )
        .unwrap();

        let RoomCommandExecution::Created(room) = &execution else {
            panic!("expected created room");
        };
        assert_eq!(room.id(), 1);
        assert_eq!(room.name(), "Tea Room");
        assert_eq!(room.model_name(), "model_b");
        assert_eq!(room.state(), RoomState::Doorbell);
        assert!(!room.show_owner_name());
        assert!(execution.command_outcome().flat_created().is_some());
    }

    #[test]
    fn returns_flat_info_for_existing_room_only() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(room(42));

        let execution = RoomCommandExecutor::get_flat_info(&dao, 42).unwrap();
        let missing = RoomCommandExecutor::get_flat_info(&dao, 99).unwrap();

        assert!(matches!(execution, RoomCommandExecution::FlatInfo(_)));
        assert!(execution.command_outcome().flat_info_packet().is_some());
        assert_eq!(missing, RoomCommandExecution::Ignored);
    }

    #[test]
    fn deletes_room_only_with_owner_rights() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(room(42));

        let ignored = RoomCommandExecutor::delete_flat(&dao, 42, false).unwrap();
        let deleted = RoomCommandExecutor::delete_flat(&dao, 42, true).unwrap();

        assert_eq!(ignored, RoomCommandExecution::Ignored);
        assert!(dao.room(42, false).unwrap().is_none());
        assert_eq!(deleted, RoomCommandExecution::Deleted { room_id: 42 });
    }

    #[test]
    fn updates_flat_metadata_and_preserves_short_name() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(room(42));

        let updated = RoomCommandExecutor::update_flat(
            &dao,
            UpdateFlatRequest::new(42, "Renamed", 2, false, true),
        )
        .unwrap();
        let preserved =
            RoomCommandExecutor::update_flat(&dao, UpdateFlatRequest::new(42, "x", 1, true, true))
                .unwrap();

        let RoomCommandExecution::Updated(updated) = updated else {
            panic!("expected update");
        };
        let RoomCommandExecution::Updated(preserved) = preserved else {
            panic!("expected second update");
        };
        assert_eq!(updated.name(), "Renamed");
        assert_eq!(updated.state(), RoomState::Password);
        assert!(!updated.show_owner_name());
        assert_eq!(preserved.name(), "Renamed");
        assert_eq!(preserved.state(), RoomState::Doorbell);
        assert!(preserved.show_owner_name());
    }

    #[test]
    fn set_flat_info_updates_description_password_and_all_super_user() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(room(42));

        let updated = RoomCommandExecutor::set_flat_info(
            &dao,
            SetFlatInfoRequest::new(42, "new desc", "secret", true, true),
        )
        .unwrap();
        let preserved = RoomCommandExecutor::set_flat_info(
            &dao,
            SetFlatInfoRequest::new(42, "x", "door", false, true),
        )
        .unwrap();

        let RoomCommandExecution::Updated(updated) = updated else {
            panic!("expected info update");
        };
        let RoomCommandExecution::Updated(preserved) = preserved else {
            panic!("expected second info update");
        };
        assert_eq!(updated.description(), "new desc");
        assert_eq!(updated.password(), "secret");
        assert!(updated.has_all_super_user());
        assert_eq!(preserved.description(), "new desc");
        assert_eq!(preserved.password(), "door");
        assert!(!preserved.has_all_super_user());
    }

    #[test]
    fn ignores_mutations_without_rights_or_missing_room() {
        let dao = InMemoryRoomDao::new();
        dao.insert_room(room(42));

        assert_eq!(
            RoomCommandExecutor::update_flat(
                &dao,
                UpdateFlatRequest::new(42, "Renamed", 1, false, false),
            )
            .unwrap(),
            RoomCommandExecution::Ignored
        );
        assert_eq!(
            RoomCommandExecutor::set_flat_info(
                &dao,
                SetFlatInfoRequest::new(99, "desc", "secret", false, true),
            )
            .unwrap(),
            RoomCommandExecution::Ignored
        );
        assert_eq!(dao.room(42, false).unwrap().unwrap().name(), "Old room");
    }
}
