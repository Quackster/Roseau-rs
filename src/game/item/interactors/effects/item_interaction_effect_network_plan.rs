use crate::game::item::{interactors::ItemInteractionEffect, Item};
use crate::game::player::{PlayerManager, PlayerSession};
use crate::messages::outgoing::{
    ActiveObjectUpdate, DoorIn, DoorOut, JumpData, JumpingPlaceOk, OpenUimakoppi, PhTickets,
    ShowProgram, UpdateWallItem,
};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionEffectNetworkPlan;

impl ItemInteractionEffectNetworkPlan {
    pub fn plan(
        effect: &ItemInteractionEffect,
        acting_user_id: i32,
        acting_username: &str,
        acting_tickets: i32,
        room_player_ids: &[i32],
        items: &[Item],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            ItemInteractionEffect::ShowProgram { item_id, program } => items
                .iter()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    Self::broadcast(
                        room_player_ids,
                        player_manager,
                        ShowProgram::new([item.item_data(), program])
                            .compose()
                            .get(),
                    )
                })
                .unwrap_or_default(),
            ItemInteractionEffect::OpenPoolChangeBooth => Self::send_to_user(
                acting_user_id,
                player_manager,
                OpenUimakoppi.compose().get(),
            )
            .into_iter()
            .collect(),
            ItemInteractionEffect::SendJumpingPlaceOk => Self::send_to_user(
                acting_user_id,
                player_manager,
                JumpingPlaceOk.compose().get(),
            )
            .into_iter()
            .collect(),
            ItemInteractionEffect::SendJumpData { username, data } => Self::broadcast(
                room_player_ids,
                player_manager,
                JumpData::new(username, data).compose().get(),
            ),
            ItemInteractionEffect::SendTickets => Self::send_to_user(
                acting_user_id,
                player_manager,
                PhTickets::new(acting_tickets).compose().get(),
            )
            .into_iter()
            .collect(),
            ItemInteractionEffect::SendDoorOut { item_id } => Self::plan_door_out(
                *item_id,
                acting_username,
                room_player_ids,
                items,
                player_manager,
            ),
            ItemInteractionEffect::SendDoorIn { item_id } => Self::plan_door_in(
                *item_id,
                acting_username,
                room_player_ids,
                items,
                player_manager,
            ),
            ItemInteractionEffect::UpdateItemStatus { item_id } => items
                .iter()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    let packet = if item.definition().behaviour().is_on_floor() {
                        ActiveObjectUpdate::new(Some(item.clone())).compose().get()
                    } else {
                        UpdateWallItem::new(item.clone()).compose().get()
                    };

                    Self::broadcast(room_player_ids, player_manager, packet)
                })
                .unwrap_or_default(),
            ItemInteractionEffect::RemoveStatus { .. }
            | ItemInteractionEffect::SetStatus { .. }
            | ItemInteractionEffect::SetBodyRotation { .. }
            | ItemInteractionEffect::SetPosition { .. }
            | ItemInteractionEffect::SetCanWalk { .. }
            | ItemInteractionEffect::SetWalking { .. }
            | ItemInteractionEffect::ClearNextStep
            | ItemInteractionEffect::ForceStopWalking
            | ItemInteractionEffect::MarkNeedsUpdate
            | ItemInteractionEffect::SetGoal { .. }
            | ItemInteractionEffect::BuildPathToGoal
            | ItemInteractionEffect::TriggerCurrentItem
            | ItemInteractionEffect::WalkTo { .. }
            | ItemInteractionEffect::LockTiles { .. }
            | ItemInteractionEffect::UnlockTiles { .. }
            | ItemInteractionEffect::DecrementTickets { .. }
            | ItemInteractionEffect::SavePlayer
            | ItemInteractionEffect::LoadRoom { .. }
            | ItemInteractionEffect::LeaveRoom { .. }
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[ItemInteractionEffect],
        acting_user_id: i32,
        acting_username: &str,
        acting_tickets: i32,
        room_player_ids: &[i32],
        items: &[Item],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| {
                Self::plan(
                    effect,
                    acting_user_id,
                    acting_username,
                    acting_tickets,
                    room_player_ids,
                    items,
                    player_manager,
                )
            })
            .collect()
    }

    pub fn plan_for_connection_ids(
        effect: &ItemInteractionEffect,
        acting_connection_id: i32,
        acting_username: &str,
        acting_tickets: i32,
        room_connection_ids: &[i32],
        items: &[Item],
    ) -> Vec<PlayerNetworkEffect> {
        match effect {
            ItemInteractionEffect::ShowProgram { item_id, program } => items
                .iter()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    Self::broadcast_to_connection_ids(
                        room_connection_ids,
                        ShowProgram::new([item.item_data(), program])
                            .compose()
                            .get(),
                    )
                })
                .unwrap_or_default(),
            ItemInteractionEffect::OpenPoolChangeBooth => {
                vec![Self::write_to_connection(
                    acting_connection_id,
                    OpenUimakoppi.compose().get(),
                )]
            }
            ItemInteractionEffect::SendJumpingPlaceOk => {
                vec![Self::write_to_connection(
                    acting_connection_id,
                    JumpingPlaceOk.compose().get(),
                )]
            }
            ItemInteractionEffect::SendJumpData { username, data } => {
                Self::broadcast_to_connection_ids(
                    room_connection_ids,
                    JumpData::new(username, data).compose().get(),
                )
            }
            ItemInteractionEffect::SendTickets => {
                vec![Self::write_to_connection(
                    acting_connection_id,
                    PhTickets::new(acting_tickets).compose().get(),
                )]
            }
            ItemInteractionEffect::SendDoorOut { item_id } => {
                Self::plan_door_out_for_connection_ids(
                    *item_id,
                    acting_username,
                    room_connection_ids,
                    items,
                )
            }
            ItemInteractionEffect::SendDoorIn { item_id } => Self::plan_door_in_for_connection_ids(
                *item_id,
                acting_username,
                room_connection_ids,
                items,
            ),
            ItemInteractionEffect::UpdateItemStatus { item_id } => items
                .iter()
                .find(|item| item.id() == *item_id)
                .map(|item| {
                    let packet = if item.definition().behaviour().is_on_floor() {
                        ActiveObjectUpdate::new(Some(item.clone())).compose().get()
                    } else {
                        UpdateWallItem::new(item.clone()).compose().get()
                    };

                    Self::broadcast_to_connection_ids(room_connection_ids, packet)
                })
                .unwrap_or_default(),
            ItemInteractionEffect::RemoveStatus { .. }
            | ItemInteractionEffect::SetStatus { .. }
            | ItemInteractionEffect::SetBodyRotation { .. }
            | ItemInteractionEffect::SetPosition { .. }
            | ItemInteractionEffect::SetCanWalk { .. }
            | ItemInteractionEffect::SetWalking { .. }
            | ItemInteractionEffect::ClearNextStep
            | ItemInteractionEffect::ForceStopWalking
            | ItemInteractionEffect::MarkNeedsUpdate
            | ItemInteractionEffect::SetGoal { .. }
            | ItemInteractionEffect::BuildPathToGoal
            | ItemInteractionEffect::TriggerCurrentItem
            | ItemInteractionEffect::WalkTo { .. }
            | ItemInteractionEffect::LockTiles { .. }
            | ItemInteractionEffect::UnlockTiles { .. }
            | ItemInteractionEffect::DecrementTickets { .. }
            | ItemInteractionEffect::SavePlayer
            | ItemInteractionEffect::LoadRoom { .. }
            | ItemInteractionEffect::LeaveRoom { .. }
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn plan_all_for_connection_ids(
        effects: &[ItemInteractionEffect],
        acting_connection_id: i32,
        acting_username: &str,
        acting_tickets: i32,
        room_connection_ids: &[i32],
        items: &[Item],
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| {
                Self::plan_for_connection_ids(
                    effect,
                    acting_connection_id,
                    acting_username,
                    acting_tickets,
                    room_connection_ids,
                    items,
                )
            })
            .collect()
    }

    fn plan_door_out(
        item_id: i32,
        acting_username: &str,
        room_player_ids: &[i32],
        items: &[Item],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        items
            .iter()
            .find(|item| item.id() == item_id)
            .map(|item| {
                Self::broadcast(
                    room_player_ids,
                    player_manager,
                    DoorOut::new(item.padding(), item.id(), acting_username)
                        .compose()
                        .get(),
                )
            })
            .unwrap_or_default()
    }

    fn plan_door_out_for_connection_ids(
        item_id: i32,
        acting_username: &str,
        room_connection_ids: &[i32],
        items: &[Item],
    ) -> Vec<PlayerNetworkEffect> {
        items
            .iter()
            .find(|item| item.id() == item_id)
            .map(|item| {
                Self::broadcast_to_connection_ids(
                    room_connection_ids,
                    DoorOut::new(item.padding(), item.id(), acting_username)
                        .compose()
                        .get(),
                )
            })
            .unwrap_or_default()
    }

    fn plan_door_in(
        item_id: i32,
        acting_username: &str,
        room_player_ids: &[i32],
        items: &[Item],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        items
            .iter()
            .find(|item| item.id() == item_id)
            .map(|item| {
                Self::broadcast(
                    room_player_ids,
                    player_manager,
                    DoorIn::new(item.padding(), item.id(), acting_username)
                        .compose()
                        .get(),
                )
            })
            .unwrap_or_default()
    }

    fn plan_door_in_for_connection_ids(
        item_id: i32,
        acting_username: &str,
        room_connection_ids: &[i32],
        items: &[Item],
    ) -> Vec<PlayerNetworkEffect> {
        items
            .iter()
            .find(|item| item.id() == item_id)
            .map(|item| {
                Self::broadcast_to_connection_ids(
                    room_connection_ids,
                    DoorIn::new(item.padding(), item.id(), acting_username)
                        .compose()
                        .get(),
                )
            })
            .unwrap_or_default()
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

    fn broadcast_to_connection_ids(
        room_connection_ids: &[i32],
        packet: String,
    ) -> Vec<PlayerNetworkEffect> {
        room_connection_ids
            .iter()
            .map(|connection_id| Self::write_to_connection(*connection_id, packet.clone()))
            .collect()
    }

    fn send_to_user(
        user_id: i32,
        player_manager: &PlayerManager,
        packet: String,
    ) -> Option<PlayerNetworkEffect> {
        player_manager
            .get_by_id(user_id)
            .map(|session| Self::write(session, packet))
    }

    fn write(session: &PlayerSession, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id: session.connection_id(),
            packet,
        }
    }

    fn write_to_connection(connection_id: i32, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet,
        }
    }
}
