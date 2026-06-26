use crate::game::player::{PlayerManager, PlayerSession};
use crate::game::room::entity::{RoomUser, RoomUserChatNetworkPlan, RoomUserEffect};
use crate::messages::outgoing::{PhNoTickets, ShowProgram, Status, Users};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;
use crate::settings::TALK_DISTANCE;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUserEffectNetworkPlan;

impl RoomUserEffectNetworkPlan {
    pub fn plan(
        effect: &RoomUserEffect,
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        Self::plan_with_chat_distance(
            effect,
            acting_user_id,
            room_player_ids,
            room_users,
            player_manager,
            TALK_DISTANCE,
        )
    }

    fn plan_with_chat_distance(
        effect: &RoomUserEffect,
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        chat_distance: i32,
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            RoomUserEffect::Chat {
                header,
                username,
                message,
            } if header == "CHAT" => RoomUserChatNetworkPlan::chat(
                acting_user_id,
                room_player_ids,
                room_users,
                player_manager,
                header,
                username,
                message,
                chat_distance,
            ),
            RoomUserEffect::Chat {
                header,
                username,
                message,
            } => RoomUserChatNetworkPlan::broadcast_chat(
                room_player_ids,
                room_users,
                player_manager,
                header,
                username,
                message,
            ),
            RoomUserEffect::Whisper {
                username,
                target_username,
                message,
            } => RoomUserChatNetworkPlan::whisper(
                acting_user_id,
                target_username.as_deref(),
                room_users,
                player_manager,
                username,
                message,
            ),
            RoomUserEffect::SendStatus { entity_id } => room_users
                .iter()
                .find(|user| user.entity_id() == *entity_id)
                .map(|user| {
                    Self::broadcast(
                        room_player_ids,
                        room_users,
                        player_manager,
                        Status::new([user.status_entity()]).compose().get(),
                    )
                })
                .unwrap_or_default(),
            RoomUserEffect::SendUsers { entity_id } => room_users
                .iter()
                .find(|user| user.entity_id() == *entity_id)
                .map(|user| {
                    Self::broadcast(
                        room_player_ids,
                        room_users,
                        player_manager,
                        Users::new([user.user_entry()]).compose().get(),
                    )
                })
                .unwrap_or_default(),
            RoomUserEffect::ShowProgram(parameters) => Self::broadcast(
                room_player_ids,
                room_users,
                player_manager,
                ShowProgram::new(parameters).compose().get(),
            ),
            RoomUserEffect::NotEnoughTickets => Self::send_to_user(
                acting_user_id,
                room_users,
                player_manager,
                PhNoTickets.compose().get(),
            )
            .into_iter()
            .collect(),
            RoomUserEffect::Kick => {
                Self::room_session_by_user_id(player_manager, room_users, acting_user_id)
                    .map(|session| PlayerNetworkEffect::CloseConnection {
                        connection_id: session.connection_id(),
                    })
                    .into_iter()
                    .collect()
            }
            RoomUserEffect::DelayedChat { .. }
            | RoomUserEffect::TransferRoom { .. }
            | RoomUserEffect::TriggerCurrentItem { .. }
            | RoomUserEffect::WalkStarted { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[RoomUserEffect],
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| {
                Self::plan(
                    effect,
                    acting_user_id,
                    room_player_ids,
                    room_users,
                    player_manager,
                )
            })
            .collect()
    }

    fn broadcast(
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        Self::send_to_room_player_ids(room_player_ids, room_users, player_manager, packet)
    }

    fn send_to_room_player_ids(
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_player_ids
            .iter()
            .filter_map(|user_id| {
                Self::room_session_by_user_id(player_manager, room_users, *user_id)
            })
            .map(|session| Self::write(session, packet.clone()))
            .collect()
    }

    fn send_to_user(
        user_id: i32,
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Option<PlayerNetworkEffect> {
        Self::room_session_by_user_id(player_manager, room_users, user_id)
            .map(|session| Self::write(session, packet))
    }

    fn room_session_by_user_id<'a>(
        player_manager: &'a PlayerManager,
        room_users: &[RoomUser],
        user_id: i32,
    ) -> Option<&'a PlayerSession> {
        let Some(room_id) = room_users
            .iter()
            .find(|user| user.entity_id() == user_id)
            .map(RoomUser::room_id)
        else {
            return player_manager.get_by_id(user_id);
        };

        player_manager
            .players()
            .values()
            .find(|session| {
                session.details().id() == user_id
                    && session
                        .room_user()
                        .is_some_and(|room_user| room_user.room_id() == room_id)
            })
            .or_else(|| player_manager.get_by_id(user_id))
    }

    fn write(session: &PlayerSession, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: session.connection_id(),
            packet,
        }
    }
}
