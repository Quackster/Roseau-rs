use crate::game::player::{PlayerManager, PlayerSession};
use crate::game::room::entity::RoomUser;
use crate::game::room::schedulers::SchedulerEffect;
use crate::messages::outgoing::{ShowProgram, Status};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SchedulerEffectNetworkPlan;

impl SchedulerEffectNetworkPlan {
    pub fn plan(
        effect: &SchedulerEffect,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            SchedulerEffect::SendStatus(entity_ids) => {
                let status_entities = entity_ids
                    .iter()
                    .filter_map(|entity_id| {
                        room_users
                            .iter()
                            .find(|user| user.entity_id() == *entity_id)
                            .map(RoomUser::status_entity)
                    })
                    .collect::<Vec<_>>();

                if status_entities.is_empty() {
                    Vec::new()
                } else {
                    Self::broadcast(
                        room_player_ids,
                        player_manager,
                        Status::new(status_entities).compose().get(),
                    )
                }
            }
            SchedulerEffect::ShowProgram(parameters) => Self::broadcast(
                room_player_ids,
                player_manager,
                ShowProgram::new(parameters).compose().get(),
            ),
            SchedulerEffect::TargetCamera { username, .. } => Self::broadcast(
                room_player_ids,
                player_manager,
                ShowProgram::new(["cam1", "targetcamera", username])
                    .compose()
                    .get(),
            ),
            SchedulerEffect::SetCamera(camera_type) => Self::broadcast(
                room_player_ids,
                player_manager,
                ShowProgram::new(["cam1", "setcamera", &camera_type.to_string()])
                    .compose()
                    .get(),
            ),
            SchedulerEffect::SetHeadRotation { .. }
            | SchedulerEffect::RemoveStatus { .. }
            | SchedulerEffect::TickStatus { .. }
            | SchedulerEffect::SetStatus { .. }
            | SchedulerEffect::MarkNeedsUpdate { .. }
            | SchedulerEffect::SetLookResetTime { .. }
            | SchedulerEffect::SetTimeUntilNextDrink { .. }
            | SchedulerEffect::WalkTo { .. }
            | SchedulerEffect::SetRotation { .. }
            | SchedulerEffect::MoveTo { .. }
            | SchedulerEffect::UpdateHeight { .. }
            | SchedulerEffect::SetNext { .. }
            | SchedulerEffect::PopPath { .. }
            | SchedulerEffect::ClearPath { .. }
            | SchedulerEffect::StopWalking { .. }
            | SchedulerEffect::TriggerCurrentItem { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[SchedulerEffect],
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room_player_ids, room_users, player_manager))
            .collect()
    }

    fn broadcast(
        room_player_ids: &[i32],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_player_ids
            .iter()
            .filter_map(|user_id| player_manager.get_by_id(*user_id))
            .map(|session| Self::write(session, packet.clone()))
            .collect()
    }

    fn write(session: &PlayerSession, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: session.connection_id(),
            packet,
        }
    }
}
