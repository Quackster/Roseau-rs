use crate::dao::{DaoError, ItemDao, RoomDao};
use crate::game::room::RoomDecorationOutcome;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomDecorationIncomingPlan;

impl RoomDecorationIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        item_dao: &dyn ItemDao,
        room_dao: &dyn RoomDao,
        room_id: i32,
        has_owner_rights: bool,
    ) -> Result<Vec<RoomDecorationOutcome>, DaoError> {
        let IncomingExecutionEffect::ApplyDecoration {
            decoration,
            item_id,
        } = effect
        else {
            return Ok(Vec::new());
        };

        if !has_owner_rights {
            return Ok(vec![RoomDecorationOutcome::Ignored]);
        }

        let Some(item) = item_dao.item(*item_id)? else {
            return Ok(vec![RoomDecorationOutcome::Ignored]);
        };
        if !item.definition().behaviour().is_decoration() {
            return Ok(vec![RoomDecorationOutcome::Ignored]);
        }

        let Some(mut room) = room_dao.room(room_id, false)? else {
            return Ok(vec![RoomDecorationOutcome::Ignored]);
        };
        let data = item.custom_data().unwrap_or_default().to_owned();

        match decoration.as_str() {
            "wallpaper" => room.set_wall(&data),
            "floor" => room.set_floor(&data),
            _ => return Ok(vec![RoomDecorationOutcome::Ignored]),
        }

        item_dao.delete_item(i64::from(*item_id))?;
        room_dao.update_room(&room)?;
        Ok(vec![RoomDecorationOutcome::applied(decoration, data)])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        item_dao: &dyn ItemDao,
        room_dao: &dyn RoomDao,
        room_id: i32,
        has_owner_rights: bool,
    ) -> Result<Vec<RoomDecorationOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(
                effect,
                item_dao,
                room_dao,
                room_id,
                has_owner_rights,
            )?);
        }

        Ok(outcomes)
    }
}
