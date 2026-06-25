use crate::dao::{DaoError, ItemDao};
use crate::game::item::Item;
use crate::game::room::{RoomEffect, RoomMapping};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectItemExecutor;

impl RoomEffectItemExecutor {
    pub fn apply(
        items: &mut Vec<Item>,
        mapping: &mut RoomMapping,
        item_dao: &impl ItemDao,
        effect: &RoomEffect,
    ) -> Result<Vec<Item>, DaoError> {
        match effect {
            RoomEffect::LoadPassiveObjects {
                model_name,
                room_id,
            } => {
                let loaded = item_dao
                    .public_room_items(model_name, *room_id)?
                    .into_values()
                    .collect::<Vec<_>>();
                let loaded_ids = loaded.iter().map(Item::id).collect::<Vec<_>>();

                items.retain(|item| !loaded_ids.contains(&item.id()));
                items.extend(loaded.iter().cloned());

                Ok(loaded)
            }
            RoomEffect::RegenerateCollisionMaps => {
                mapping.regenerate_collision_maps(items.clone());
                Ok(Vec::new())
            }
            RoomEffect::StartPublicServer { .. }
            | RoomEffect::ScheduleWalkTicks
            | RoomEffect::ScheduleEventTicks
            | RoomEffect::LoadBots { .. }
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
        items: &mut Vec<Item>,
        mapping: &mut RoomMapping,
        item_dao: &impl ItemDao,
        effects: &[RoomEffect],
    ) -> Result<Vec<Item>, DaoError> {
        let mut loaded = Vec::new();

        for effect in effects {
            loaded.extend(Self::apply(items, mapping, item_dao, effect)?);
        }

        Ok(loaded)
    }
}
