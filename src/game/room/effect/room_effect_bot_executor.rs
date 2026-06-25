use crate::dao::{DaoError, RoomDao};
use crate::game::player::Bot;
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectBotExecutor;

impl RoomEffectBotExecutor {
    pub fn apply(
        bots: &mut Vec<Bot>,
        room_dao: &impl RoomDao,
        effect: &RoomEffect,
    ) -> Result<Vec<Bot>, DaoError> {
        match effect {
            RoomEffect::LoadBots { room_id } => {
                let loaded = room_dao.bots(*room_id)?;
                bots.clear();
                bots.extend(loaded.iter().cloned());
                Ok(loaded)
            }
            RoomEffect::StartPublicServer { .. }
            | RoomEffect::ScheduleWalkTicks
            | RoomEffect::ScheduleEventTicks
            | RoomEffect::LoadPassiveObjects { .. }
            | RoomEffect::RegenerateCollisionMaps
            | RoomEffect::RegisterEvent { .. }
            | RoomEffect::SendDoorbell { .. }
            | RoomEffect::SendOwnerPrivileges { .. }
            | RoomEffect::SendControllerPrivileges { .. }
            | RoomEffect::SendNoControllerPrivileges { .. }
            | RoomEffect::SetRoomUserStatus { .. }
            | RoomEffect::RemoveRoomUserStatus { .. }
            | RoomEffect::MarkRoomUserForUpdate { .. }
            | RoomEffect::LetUserIn { .. }
            | RoomEffect::LeaveRoom { .. }
            | RoomEffect::KickUser { .. }
            | RoomEffect::ClearRuntimeData
            | RoomEffect::RemoveLoadedRoom { .. }
            | RoomEffect::SaveRights { .. } => Ok(Vec::new()),
        }
    }

    pub fn apply_all(
        bots: &mut Vec<Bot>,
        room_dao: &impl RoomDao,
        effects: &[RoomEffect],
    ) -> Result<Vec<Bot>, DaoError> {
        let mut loaded = Vec::new();

        for effect in effects {
            loaded.extend(Self::apply(bots, room_dao, effect)?);
        }

        Ok(loaded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryRoomDao;
    use crate::game::room::model::Position;

    fn bot(x: i32, y: i32) -> Bot {
        Bot::new(Position::new(x, y, 0.0), vec![], vec![], vec![])
    }

    #[test]
    fn loads_room_bots_from_room_dao() {
        let dao = InMemoryRoomDao::new();
        dao.insert_bots(7, vec![bot(1, 2), bot(3, 4)]);
        dao.insert_bots(8, vec![bot(9, 9)]);
        let mut bots = vec![bot(0, 0)];

        let loaded =
            RoomEffectBotExecutor::apply(&mut bots, &dao, &RoomEffect::LoadBots { room_id: 7 })
                .unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(bots.len(), 2);
        assert_eq!(bots[0].start_position(), Position::new(1, 2, 0.0));
        assert_eq!(bots[1].start_position(), Position::new(3, 4, 0.0));
    }

    #[test]
    fn ignores_non_bot_room_effects() {
        let dao = InMemoryRoomDao::new();
        dao.insert_bots(7, vec![bot(1, 2)]);
        let mut bots = vec![bot(0, 0)];

        let loaded = RoomEffectBotExecutor::apply_all(
            &mut bots,
            &dao,
            &[
                RoomEffect::SendOwnerPrivileges { user_id: 7 },
                RoomEffect::RegenerateCollisionMaps,
            ],
        )
        .unwrap();

        assert!(loaded.is_empty());
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].start_position(), Position::new(0, 0, 0.0));
    }
}
