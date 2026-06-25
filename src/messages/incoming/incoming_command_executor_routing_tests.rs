use crate::game::commands::{CommandContext, CommandManager};
use crate::messages::{IncomingCommand, IncomingCommandExecutor, IncomingExecutionEffect};

#[test]
fn translates_room_lookup_catalogue_and_misc_commands_to_execution_effects() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(
            &manager,
            &context,
            &[
                IncomingCommand::CreateFlat {
                    floor: "model_a".to_owned(),
                    room_name: "den".to_owned(),
                    room_model: "a".to_owned(),
                    state: 1,
                    show_owner_name: true,
                },
                IncomingCommand::DeleteFlat { room_id: 5 },
                IncomingCommand::GetFlatInfo { room_id: 6 },
                IncomingCommand::GetOrderInfo {
                    call_id: "chair".to_owned(),
                },
                IncomingCommand::GetUnitUsers {
                    room_name: "lobby".to_owned(),
                },
                IncomingCommand::GoToFlat,
                IncomingCommand::InitUnitListener,
                IncomingCommand::MoveStuff {
                    item_id: 3,
                    x: 4,
                    y: 5,
                    rotation: Some(2),
                },
                IncomingCommand::JumpPerformance {
                    data: "jump".to_owned(),
                },
                IncomingCommand::ClosePoolChangeBooth,
                IncomingCommand::RetrieveUserInfo,
                IncomingCommand::FindUser {
                    username: "alice".to_owned(),
                },
                IncomingCommand::ApplyDecoration {
                    decoration: "wallpaper".to_owned(),
                    item_id: 7,
                },
                IncomingCommand::SetItemData {
                    item_id: 8,
                    data: "ON".to_owned(),
                },
                IncomingCommand::SetStuffData {
                    item_id: 9,
                    data_class: "score".to_owned(),
                    custom_data: "10".to_owned(),
                },
                IncomingCommand::UseStripItem { item_id: 10 },
                IncomingCommand::SplashPosition {
                    position: "1,2,3".to_owned(),
                },
            ],
        ),
        vec![
            IncomingExecutionEffect::CreateFlat {
                floor: "model_a".to_owned(),
                room_name: "den".to_owned(),
                room_model: "a".to_owned(),
                state: 1,
                show_owner_name: true,
            },
            IncomingExecutionEffect::DeleteFlat { room_id: 5 },
            IncomingExecutionEffect::GetFlatInfo { room_id: 6 },
            IncomingExecutionEffect::GetOrderInfo {
                call_id: "chair".to_owned(),
            },
            IncomingExecutionEffect::GetUnitUsers {
                room_name: "lobby".to_owned(),
            },
            IncomingExecutionEffect::GoToFlat,
            IncomingExecutionEffect::InitUnitListener,
            IncomingExecutionEffect::MoveStuff {
                item_id: 3,
                x: 4,
                y: 5,
                rotation: Some(2),
            },
            IncomingExecutionEffect::JumpPerformance {
                data: "jump".to_owned(),
            },
            IncomingExecutionEffect::ClosePoolChangeBooth,
            IncomingExecutionEffect::RetrieveUserInfo,
            IncomingExecutionEffect::FindUser {
                username: "alice".to_owned(),
            },
            IncomingExecutionEffect::ApplyDecoration {
                decoration: "wallpaper".to_owned(),
                item_id: 7,
            },
            IncomingExecutionEffect::SetItemData {
                item_id: 8,
                data: "ON".to_owned(),
            },
            IncomingExecutionEffect::SetStuffData {
                item_id: 9,
                data_class: "score".to_owned(),
                custom_data: "10".to_owned(),
            },
            IncomingExecutionEffect::UseStripItem { item_id: 10 },
            IncomingExecutionEffect::SplashPosition {
                position: "1,2,3".to_owned(),
            },
        ]
    );
}

#[test]
fn translates_navigation_purchase_and_profile_room_update_commands() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(
            &manager,
            &context,
            &[
                IncomingCommand::SearchBusyFlats { multiplier: 2 },
                IncomingCommand::EmptySearchBusyFlats,
                IncomingCommand::SearchFlat {
                    query: "pool".to_owned(),
                },
                IncomingCommand::SearchFlatForUser {
                    username: "alice".to_owned(),
                },
                IncomingCommand::TryFlat {
                    room_id: 3,
                    password: "door".to_owned(),
                },
                IncomingCommand::PlaceWallItemFromInventory {
                    item_id: 4,
                    wall_position: ":w=1,2".to_owned(),
                },
                IncomingCommand::PlaceFloorItemFromInventory {
                    item_id: 5,
                    x: 6,
                    y: 7,
                    rotation: 2,
                },
                IncomingCommand::Purchase {
                    call_id: "chair".to_owned(),
                },
                IncomingCommand::SetFlatInfo {
                    room_id: 8,
                    description: "desc".to_owned(),
                    password: "secret".to_owned(),
                    all_super_user: true,
                },
                IncomingCommand::UpdatePoolFigure {
                    pool_figure: "pool".to_owned(),
                },
                IncomingCommand::UpdateFlat {
                    room_id: 9,
                    room_name: "new".to_owned(),
                    state: 1,
                    show_owner_name: false,
                },
            ],
        ),
        vec![
            IncomingExecutionEffect::SearchBusyFlats { multiplier: 2 },
            IncomingExecutionEffect::EmptySearchBusyFlats,
            IncomingExecutionEffect::SearchFlat {
                query: "pool".to_owned(),
            },
            IncomingExecutionEffect::SearchFlatForUser {
                username: "alice".to_owned(),
            },
            IncomingExecutionEffect::TryFlat {
                room_id: 3,
                password: "door".to_owned(),
            },
            IncomingExecutionEffect::PlaceWallItemFromInventory {
                item_id: 4,
                wall_position: ":w=1,2".to_owned(),
            },
            IncomingExecutionEffect::PlaceFloorItemFromInventory {
                item_id: 5,
                x: 6,
                y: 7,
                rotation: 2,
            },
            IncomingExecutionEffect::Purchase {
                call_id: "chair".to_owned(),
            },
            IncomingExecutionEffect::SetFlatInfo {
                room_id: 8,
                description: "desc".to_owned(),
                password: "secret".to_owned(),
                all_super_user: true,
            },
            IncomingExecutionEffect::UpdatePoolFigure {
                pool_figure: "pool".to_owned(),
            },
            IncomingExecutionEffect::UpdateFlat {
                room_id: 9,
                room_name: "new".to_owned(),
                state: 1,
                show_owner_name: false,
            },
        ]
    );
}
