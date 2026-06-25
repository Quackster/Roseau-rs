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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::PlayerDetails;

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn manager() -> PlayerManager {
        let mut manager = PlayerManager::new(vec![]);
        manager.insert(PlayerSession::new(70, 30000, details(7, "Alice")));
        manager.insert(PlayerSession::new(80, 30000, details(8, "Bob")));
        manager
    }

    fn room_user(entity_id: i32, name: &str) -> RoomUser {
        let mut user = RoomUser::new(entity_id, name, "hd-100", "hello", None::<String>);
        user.set_room_id(42);
        user
    }

    #[test]
    fn broadcasts_batched_status_for_matching_room_users() {
        let mut alice = room_user(7, "Alice");
        alice.set_status("mv", " 1,2,0", true, -1);
        let bob = room_user(8, "Bob");

        let effects = SchedulerEffectNetworkPlan::plan(
            &SchedulerEffect::SendStatus(vec![7, 8, 9]),
            &[7, 8],
            &[alice, bob],
            &manager(),
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#STATUS \rAlice 0,0,0,0,0/mv 1,2,0/\rBob 0,0,0,0,0/##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 80,
                    packet: "#STATUS \rAlice 0,0,0,0,0/mv 1,2,0/\rBob 0,0,0,0,0/##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn broadcasts_show_program_and_lido_camera_effects() {
        let effects = SchedulerEffectNetworkPlan::plan_all(
            &[
                SchedulerEffect::ShowProgram(vec![
                    "lamp".to_owned(),
                    "setlamp".to_owned(),
                    "2".to_owned(),
                ]),
                SchedulerEffect::TargetCamera {
                    player_id: 7,
                    username: "Alice".to_owned(),
                },
                SchedulerEffect::SetCamera(1),
            ],
            &[7],
            &[],
            &manager(),
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#SHOWPROGRAM\rlamp setlamp 2##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#SHOWPROGRAM\rcam1 targetcamera Alice##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 70,
                    packet: "#SHOWPROGRAM\rcam1 setcamera 1##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn ignores_non_network_scheduler_effects() {
        let effects = SchedulerEffectNetworkPlan::plan(
            &SchedulerEffect::MarkNeedsUpdate { entity_id: 7 },
            &[7],
            &[room_user(7, "Alice")],
            &manager(),
        );

        assert!(effects.is_empty());
    }
}
