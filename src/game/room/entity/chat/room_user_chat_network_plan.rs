use crate::game::player::{PlayerManager, PlayerSession};
use crate::game::room::entity::RoomUser;
use crate::messages::outgoing::Chat;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUserChatNetworkPlan;

impl RoomUserChatNetworkPlan {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn chat(
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        header: &str,
        username: &str,
        message: &str,
        chat_distance: i32,
    ) -> Vec<PlayerNetworkEffect> {
        Self::send_to_room_player_ids(
            &Self::chat_recipient_ids(acting_user_id, room_player_ids, room_users, chat_distance),
            room_users,
            player_manager,
            Chat::new(header, username, message).compose().get(),
        )
    }

    pub(crate) fn broadcast_chat(
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        header: &str,
        username: &str,
        message: &str,
    ) -> Vec<PlayerNetworkEffect> {
        Self::send_to_room_player_ids(
            room_player_ids,
            room_users,
            player_manager,
            Chat::new(header, username, message).compose().get(),
        )
    }

    pub(crate) fn whisper(
        acting_user_id: i32,
        target_username: Option<&str>,
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        username: &str,
        message: &str,
    ) -> Vec<PlayerNetworkEffect> {
        let Some(acting_session) =
            room_session_by_user_id(player_manager, room_users, acting_user_id)
        else {
            return Vec::new();
        };

        let packet = Chat::new("WHISPER", username, message).compose().get();
        let mut effects = vec![Self::write(acting_session, packet.clone())];

        let target_session = target_username.and_then(|username| {
            if room_users.is_empty() {
                player_manager.get_by_name(username)
            } else {
                room_sessions(player_manager, room_users)
                    .into_iter()
                    .find(|session| session.details().username().eq_ignore_ascii_case(username))
            }
        });

        if let Some(target_session) = target_session {
            if target_session.details().id() != acting_user_id {
                effects.push(Self::write(target_session, packet));
            }
        }

        effects
    }

    fn send_to_room_player_ids(
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        player_manager: &PlayerManager,
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_player_ids
            .iter()
            .filter_map(|user_id| room_session_by_user_id(player_manager, room_users, *user_id))
            .map(|session| Self::write(session, packet.clone()))
            .collect()
    }

    fn chat_recipient_ids(
        acting_user_id: i32,
        room_player_ids: &[i32],
        room_users: &[RoomUser],
        chat_distance: i32,
    ) -> Vec<i32> {
        let Some(speaker) = room_users
            .iter()
            .find(|user| user.entity_id() == acting_user_id)
        else {
            return vec![acting_user_id];
        };

        room_player_ids
            .iter()
            .copied()
            .filter(|user_id| {
                *user_id == acting_user_id
                    || room_users
                        .iter()
                        .find(|user| user.entity_id() == *user_id)
                        .is_some_and(|user| {
                            speaker.position().distance(user.position()) <= chat_distance
                        })
            })
            .collect()
    }

    fn write(session: &PlayerSession, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: session.connection_id(),
            packet,
        }
    }
}

fn room_sessions<'a>(
    player_manager: &'a PlayerManager,
    room_users: &[RoomUser],
) -> Vec<&'a PlayerSession> {
    player_manager
        .players()
        .values()
        .filter(|session| {
            session.room_user().is_some_and(|session_room_user| {
                room_users.iter().any(|room_user| {
                    room_user.entity_id() == session.details().id()
                        && room_user.room_id() == session_room_user.room_id()
                })
            })
        })
        .collect()
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
