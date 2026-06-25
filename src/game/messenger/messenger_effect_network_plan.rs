use crate::game::messenger::MessengerEffect;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerEffectNetworkPlan;

impl MessengerEffectNetworkPlan {
    pub fn plan(effect: &MessengerEffect, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        match effect {
            MessengerEffect::SendRequests(packet) => {
                vec![Self::write(connection_id, packet.compose().get())]
            }
            MessengerEffect::SendFriends(packet) => {
                vec![Self::write(connection_id, packet.compose().get())]
            }
            MessengerEffect::RefreshFriendList { .. } => Vec::new(),
        }
    }

    pub fn plan_all(effects: &[MessengerEffect], connection_id: i32) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, connection_id))
            .collect()
    }

    fn write(connection_id: i32, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::outgoing::{BuddyAddRequests, BuddyList, BuddyListFriend};

    #[test]
    fn plans_direct_messenger_packets_for_connection() {
        let effects = MessengerEffectNetworkPlan::plan_all(
            &[
                MessengerEffect::SendRequests(BuddyAddRequests::new(["alice"])),
                MessengerEffect::SendFriends(BuddyList::new(
                    [BuddyListFriend::new(
                        7,
                        "bob",
                        "mission",
                        Some("Room"),
                        "50",
                    )],
                    None,
                )),
            ],
            42,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#BUDDYADDREQUESTS\r/alice##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#BUDDYLIST\r7\tbob\tmission\rRoom\t50##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn leaves_friend_refresh_for_runtime_messenger_lookup() {
        assert!(MessengerEffectNetworkPlan::plan(
            &MessengerEffect::RefreshFriendList {
                user_id: 7,
                offline_user_id: Some(5),
            },
            42,
        )
        .is_empty());
    }
}
