use crate::game::commands::CommandEffectNetworkPlan;
use crate::messages::IncomingExecutionEffect;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IncomingExecutionEffectNetworkPlan;

impl IncomingExecutionEffectNetworkPlan {
    pub fn plan(effect: &IncomingExecutionEffect, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        match effect {
            IncomingExecutionEffect::Command(command) => {
                CommandEffectNetworkPlan::plan(command, connection_id)
            }
            IncomingExecutionEffect::SetRoomStatus { .. }
            | IncomingExecutionEffect::RemoveRoomStatus { .. }
            | IncomingExecutionEffect::MarkRoomNeedsUpdate
            | IncomingExecutionEffect::ResetAfkTimer
            | IncomingExecutionEffect::RefreshInventory { .. }
            | IncomingExecutionEffect::MarkMessengerMessageRead { .. }
            | IncomingExecutionEffect::SendTickets
            | IncomingExecutionEffect::AssignPersonalMessage { .. }
            | IncomingExecutionEffect::GoAway
            | IncomingExecutionEffect::RequestBuddy { .. }
            | IncomingExecutionEffect::AcceptBuddy { .. }
            | IncomingExecutionEffect::DeclineBuddy { .. }
            | IncomingExecutionEffect::RemoveBuddy { .. }
            | IncomingExecutionEffect::SendMessengerMessage { .. }
            | IncomingExecutionEffect::InitMessenger
            | IncomingExecutionEffect::MoveStuff { .. }
            | IncomingExecutionEffect::WalkTo { .. }
            | IncomingExecutionEffect::LookTo { .. }
            | IncomingExecutionEffect::EnterDoor { .. }
            | IncomingExecutionEffect::LetUserIn { .. }
            | IncomingExecutionEffect::RemoveItem { .. }
            | IncomingExecutionEffect::AssignRights { .. }
            | IncomingExecutionEffect::RemoveRights { .. }
            | IncomingExecutionEffect::KickUser { .. }
            | IncomingExecutionEffect::AddWallItem { .. }
            | IncomingExecutionEffect::ReturnItemToInventory { .. }
            | IncomingExecutionEffect::CreateFlat { .. }
            | IncomingExecutionEffect::DeleteFlat { .. }
            | IncomingExecutionEffect::GetFlatInfo { .. }
            | IncomingExecutionEffect::GetOrderInfo { .. }
            | IncomingExecutionEffect::GetUnitUsers { .. }
            | IncomingExecutionEffect::GoToFlat
            | IncomingExecutionEffect::InitUnitListener
            | IncomingExecutionEffect::JumpPerformance { .. }
            | IncomingExecutionEffect::ClosePoolChangeBooth
            | IncomingExecutionEffect::RetrieveUserInfo
            | IncomingExecutionEffect::FindUser { .. }
            | IncomingExecutionEffect::ApplyDecoration { .. }
            | IncomingExecutionEffect::SetItemData { .. }
            | IncomingExecutionEffect::SetStuffData { .. }
            | IncomingExecutionEffect::UseStripItem { .. }
            | IncomingExecutionEffect::SplashPosition { .. }
            | IncomingExecutionEffect::SearchBusyFlats { .. }
            | IncomingExecutionEffect::EmptySearchBusyFlats
            | IncomingExecutionEffect::SearchFlat { .. }
            | IncomingExecutionEffect::SearchFlatForUser { .. }
            | IncomingExecutionEffect::TryFlat { .. }
            | IncomingExecutionEffect::PlaceWallItemFromInventory { .. }
            | IncomingExecutionEffect::PlaceFloorItemFromInventory { .. }
            | IncomingExecutionEffect::Purchase { .. }
            | IncomingExecutionEffect::SetFlatInfo { .. }
            | IncomingExecutionEffect::UpdatePoolFigure { .. }
            | IncomingExecutionEffect::UpdateFlat { .. }
            | IncomingExecutionEffect::CryForHelp { .. }
            | IncomingExecutionEffect::Talk { .. }
            | IncomingExecutionEffect::Password(_) => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, connection_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::commands::CommandEffect;

    #[test]
    fn plans_command_alerts_for_current_connection() {
        let effects = IncomingExecutionEffectNetworkPlan::plan(
            &IncomingExecutionEffect::Command(CommandEffect::SendAlert("hello".to_owned())),
            42,
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#SYSTEMBROADCAST\rhello##".to_owned(),
            }]
        );
    }

    #[test]
    fn leaves_non_network_incoming_effects_to_domain_executors() {
        let effects = IncomingExecutionEffectNetworkPlan::plan_all(
            &[
                IncomingExecutionEffect::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                },
                IncomingExecutionEffect::ResetAfkTimer,
                IncomingExecutionEffect::Command(CommandEffect::MarkRoomNeedsUpdate),
            ],
            42,
        );

        assert!(effects.is_empty());
    }
}
