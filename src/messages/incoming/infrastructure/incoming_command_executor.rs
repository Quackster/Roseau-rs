use crate::game::commands::{CommandContext, CommandManager};
use crate::messages::incoming::incoming_messenger_command_plan::IncomingMessengerCommandPlan;
use crate::messages::incoming::incoming_password_command_plan::IncomingPasswordCommandPlan;
use crate::messages::{IncomingCommand, IncomingExecutionEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IncomingCommandExecutor;

impl IncomingCommandExecutor {
    pub fn plan(
        command_manager: &CommandManager,
        command_context: &CommandContext,
        commands: &[IncomingCommand],
    ) -> Vec<IncomingExecutionEffect> {
        commands
            .iter()
            .flat_map(|command| Self::plan_one(command_manager, command_context, command))
            .collect()
    }

    fn plan_one(
        command_manager: &CommandManager,
        command_context: &CommandContext,
        command: &IncomingCommand,
    ) -> Vec<IncomingExecutionEffect> {
        if let Some(effect) = IncomingPasswordCommandPlan::plan(command) {
            return vec![effect];
        }
        if let Some(effects) = IncomingMessengerCommandPlan::plan(command) {
            return effects;
        }

        match command {
            IncomingCommand::SetRoomStatus {
                key,
                value,
                visible,
                timeout,
            } => vec![IncomingExecutionEffect::SetRoomStatus {
                key: key.clone(),
                value: value.clone(),
                visible: *visible,
                timeout: *timeout,
            }],
            IncomingCommand::RemoveRoomStatus { key } => {
                vec![IncomingExecutionEffect::RemoveRoomStatus { key: key.clone() }]
            }
            IncomingCommand::MarkRoomNeedsUpdate => {
                vec![IncomingExecutionEffect::MarkRoomNeedsUpdate]
            }
            IncomingCommand::ResetAfkTimer => vec![IncomingExecutionEffect::ResetAfkTimer],
            IncomingCommand::SendAlert { message } => {
                vec![IncomingExecutionEffect::Command(
                    crate::game::commands::CommandEffect::SendAlert(message.clone()),
                )]
            }
            IncomingCommand::CloseUserConnections => {
                vec![IncomingExecutionEffect::CloseUserConnections]
            }
            IncomingCommand::ClosePublicRoomConnections => {
                vec![IncomingExecutionEffect::ClosePublicRoomConnections]
            }
            IncomingCommand::RefreshInventory { category } => {
                vec![IncomingExecutionEffect::RefreshInventory {
                    category: category.clone(),
                }]
            }
            IncomingCommand::SendTickets => vec![IncomingExecutionEffect::SendTickets],
            IncomingCommand::GoAway => vec![IncomingExecutionEffect::GoAway],
            IncomingCommand::MoveStuff {
                item_id,
                x,
                y,
                rotation,
            } => vec![IncomingExecutionEffect::MoveStuff {
                item_id: *item_id,
                x: *x,
                y: *y,
                rotation: *rotation,
            }],
            IncomingCommand::WalkTo { x, y } => {
                vec![IncomingExecutionEffect::WalkTo { x: *x, y: *y }]
            }
            IncomingCommand::LookTo { x, y } => {
                vec![IncomingExecutionEffect::LookTo { x: *x, y: *y }]
            }
            IncomingCommand::EnterDoor { item_id } => {
                vec![IncomingExecutionEffect::EnterDoor { item_id: *item_id }]
            }
            IncomingCommand::LetUserIn { username } => {
                vec![IncomingExecutionEffect::LetUserIn {
                    username: username.clone(),
                }]
            }
            IncomingCommand::RemoveItem { item_id } => {
                vec![IncomingExecutionEffect::RemoveItem { item_id: *item_id }]
            }
            IncomingCommand::AssignRights { username } => {
                vec![IncomingExecutionEffect::AssignRights {
                    username: username.clone(),
                }]
            }
            IncomingCommand::RemoveRights { username } => {
                vec![IncomingExecutionEffect::RemoveRights {
                    username: username.clone(),
                }]
            }
            IncomingCommand::KickUser { username } => {
                vec![IncomingExecutionEffect::KickUser {
                    username: username.clone(),
                }]
            }
            IncomingCommand::AddWallItem {
                sprite,
                wall_position,
                extra_data,
            } => vec![IncomingExecutionEffect::AddWallItem {
                sprite: sprite.clone(),
                wall_position: wall_position.clone(),
                extra_data: extra_data.clone(),
            }],
            IncomingCommand::ReturnItemToInventory { item_id } => {
                vec![
                    IncomingExecutionEffect::ReturnItemToInventory { item_id: *item_id },
                    IncomingExecutionEffect::RefreshInventory {
                        category: "last".to_owned(),
                    },
                ]
            }
            IncomingCommand::CreateFlat {
                floor,
                room_name,
                room_model,
                state,
                show_owner_name,
            } => vec![IncomingExecutionEffect::CreateFlat {
                floor: floor.clone(),
                room_name: room_name.clone(),
                room_model: room_model.clone(),
                state: *state,
                show_owner_name: *show_owner_name,
            }],
            IncomingCommand::DeleteFlat { room_id } => {
                vec![IncomingExecutionEffect::DeleteFlat { room_id: *room_id }]
            }
            IncomingCommand::GetFlatInfo { room_id } => {
                vec![IncomingExecutionEffect::GetFlatInfo { room_id: *room_id }]
            }
            IncomingCommand::GetOrderInfo { call_id } => {
                vec![IncomingExecutionEffect::GetOrderInfo {
                    call_id: call_id.clone(),
                }]
            }
            IncomingCommand::GetUnitUsers { room_name } => {
                vec![IncomingExecutionEffect::GetUnitUsers {
                    room_name: room_name.clone(),
                }]
            }
            IncomingCommand::GoToFlat => vec![IncomingExecutionEffect::GoToFlat],
            IncomingCommand::InitUnitListener => vec![IncomingExecutionEffect::InitUnitListener],
            IncomingCommand::JumpPerformance { data } => {
                vec![IncomingExecutionEffect::JumpPerformance { data: data.clone() }]
            }
            IncomingCommand::ClosePoolChangeBooth => {
                vec![IncomingExecutionEffect::ClosePoolChangeBooth]
            }
            IncomingCommand::RetrieveUserInfo => vec![IncomingExecutionEffect::RetrieveUserInfo],
            IncomingCommand::GetCredits => vec![IncomingExecutionEffect::GetCredits],
            IncomingCommand::FindUser { username } => {
                vec![IncomingExecutionEffect::FindUser {
                    username: username.clone(),
                }]
            }
            IncomingCommand::ApplyDecoration {
                decoration,
                item_id,
            } => vec![IncomingExecutionEffect::ApplyDecoration {
                decoration: decoration.clone(),
                item_id: *item_id,
            }],
            IncomingCommand::SetItemData { item_id, data } => {
                vec![IncomingExecutionEffect::SetItemData {
                    item_id: *item_id,
                    data: data.clone(),
                }]
            }
            IncomingCommand::SetStuffData {
                item_id,
                data_class,
                custom_data,
            } => vec![IncomingExecutionEffect::SetStuffData {
                item_id: *item_id,
                data_class: data_class.clone(),
                custom_data: custom_data.clone(),
            }],
            IncomingCommand::UseStripItem { item_id } => {
                vec![IncomingExecutionEffect::UseStripItem { item_id: *item_id }]
            }
            IncomingCommand::SplashPosition { position } => {
                vec![IncomingExecutionEffect::SplashPosition {
                    position: position.clone(),
                }]
            }
            IncomingCommand::SearchBusyFlats { multiplier } => {
                vec![IncomingExecutionEffect::SearchBusyFlats {
                    multiplier: *multiplier,
                }]
            }
            IncomingCommand::EmptySearchBusyFlats => {
                vec![IncomingExecutionEffect::EmptySearchBusyFlats]
            }
            IncomingCommand::SearchFlat { query } => {
                vec![IncomingExecutionEffect::SearchFlat {
                    query: query.clone(),
                }]
            }
            IncomingCommand::SearchFlatForUser { username } => {
                vec![IncomingExecutionEffect::SearchFlatForUser {
                    username: username.clone(),
                }]
            }
            IncomingCommand::TryFlat { room_id, password } => {
                vec![IncomingExecutionEffect::TryFlat {
                    room_id: *room_id,
                    password: password.clone(),
                }]
            }
            IncomingCommand::PlaceWallItemFromInventory {
                item_id,
                wall_position,
            } => vec![IncomingExecutionEffect::PlaceWallItemFromInventory {
                item_id: *item_id,
                wall_position: wall_position.clone(),
            }],
            IncomingCommand::PlaceFloorItemFromInventory {
                item_id,
                x,
                y,
                rotation,
            } => vec![IncomingExecutionEffect::PlaceFloorItemFromInventory {
                item_id: *item_id,
                x: *x,
                y: *y,
                rotation: *rotation,
            }],
            IncomingCommand::Purchase { call_id } => {
                vec![IncomingExecutionEffect::Purchase {
                    call_id: call_id.clone(),
                }]
            }
            IncomingCommand::SetFlatInfo {
                room_id,
                description,
                password,
                all_super_user,
            } => vec![IncomingExecutionEffect::SetFlatInfo {
                room_id: *room_id,
                description: description.clone(),
                password: password.clone(),
                all_super_user: *all_super_user,
            }],
            IncomingCommand::UpdatePoolFigure { pool_figure } => {
                vec![IncomingExecutionEffect::UpdatePoolFigure {
                    pool_figure: pool_figure.clone(),
                }]
            }
            IncomingCommand::UpdateFlat {
                room_id,
                room_name,
                state,
                show_owner_name,
            } => vec![IncomingExecutionEffect::UpdateFlat {
                room_id: *room_id,
                room_name: room_name.clone(),
                state: *state,
                show_owner_name: *show_owner_name,
            }],
            IncomingCommand::CryForHelp { message } => {
                vec![IncomingExecutionEffect::CryForHelp {
                    message: message.clone(),
                }]
            }
            IncomingCommand::Login { .. }
            | IncomingCommand::RegisterPlayer { .. }
            | IncomingCommand::UpdateProfile { .. }
            | IncomingCommand::MarkMessengerMessageRead { .. }
            | IncomingCommand::AssignPersonalMessage { .. }
            | IncomingCommand::RequestBuddy { .. }
            | IncomingCommand::AcceptBuddy { .. }
            | IncomingCommand::DeclineBuddy { .. }
            | IncomingCommand::RemoveBuddy { .. }
            | IncomingCommand::SendMessengerMessage { .. }
            | IncomingCommand::InitMessenger => {
                unreachable!("pre-dispatch command planners handle this command")
            }
            IncomingCommand::Talk { mode, message }
                if mode != "WHISPER" && message.starts_with(':') =>
            {
                let effects = command_manager.invoke_command(command_context, message);
                if effects.is_empty() {
                    vec![IncomingExecutionEffect::Talk {
                        mode: mode.clone(),
                        message: message.clone(),
                    }]
                } else {
                    effects
                        .into_iter()
                        .map(IncomingExecutionEffect::Command)
                        .collect()
                }
            }
            IncomingCommand::Talk { mode, message } => {
                vec![IncomingExecutionEffect::Talk {
                    mode: mode.clone(),
                    message: message.clone(),
                }]
            }
        }
    }
}
