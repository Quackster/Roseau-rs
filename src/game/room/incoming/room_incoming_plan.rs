use crate::dao::{DaoError, RoomDao};
use crate::game::player::PlayerDetails;
use crate::game::room::{
    CreateFlatRequest, RoomCommandExecution, RoomCommandExecutor, SetFlatInfoRequest,
    UpdateFlatRequest,
};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomIncomingPlan;

impl RoomIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        room_dao: &dyn RoomDao,
        owner: &PlayerDetails,
        has_owner_rights: bool,
    ) -> Result<Vec<RoomCommandExecution>, DaoError> {
        let execution = match effect {
            IncomingExecutionEffect::CreateFlat {
                room_name,
                room_model,
                state,
                show_owner_name,
                ..
            } => RoomCommandExecutor::create_flat(
                room_dao,
                owner,
                CreateFlatRequest::new(room_name, room_model, *state, *show_owner_name),
            )?,
            IncomingExecutionEffect::DeleteFlat { room_id } => {
                RoomCommandExecutor::delete_flat(room_dao, *room_id, has_owner_rights)?
            }
            IncomingExecutionEffect::GetFlatInfo { room_id } => {
                RoomCommandExecutor::get_flat_info(room_dao, *room_id)?
            }
            IncomingExecutionEffect::SetFlatInfo {
                room_id,
                description,
                password,
                all_super_user,
            } => RoomCommandExecutor::set_flat_info(
                room_dao,
                SetFlatInfoRequest::new(
                    *room_id,
                    description,
                    password,
                    *all_super_user,
                    has_owner_rights,
                ),
            )?,
            IncomingExecutionEffect::UpdateFlat {
                room_id,
                room_name,
                state,
                show_owner_name,
            } => RoomCommandExecutor::update_flat(
                room_dao,
                UpdateFlatRequest::new(
                    *room_id,
                    room_name,
                    *state,
                    *show_owner_name,
                    has_owner_rights,
                ),
            )?,
            _ => return Ok(Vec::new()),
        };

        Ok(vec![execution])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        room_dao: &dyn RoomDao,
        owner: &PlayerDetails,
        has_owner_rights: bool,
    ) -> Result<Vec<RoomCommandExecution>, DaoError> {
        let mut executions = Vec::new();

        for effect in effects {
            executions.extend(Self::plan(effect, room_dao, owner, has_owner_rights)?);
        }

        Ok(executions)
    }
}

#[cfg(test)]
#[path = "room_incoming_plan_tests.rs"]
mod tests;
