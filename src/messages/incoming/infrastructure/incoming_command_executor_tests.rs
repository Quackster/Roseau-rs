use crate::game::commands::{CommandContext, CommandEffect, CommandManager, RoomUserCommandState};
use crate::game::player::PasswordAction;
use crate::messages::{IncomingCommand, IncomingCommandExecutor, IncomingExecutionEffect};

#[test]
fn translates_supported_commands_to_domain_effects() {
    let manager = CommandManager::new();
    let context = CommandContext::new();
    let effects = IncomingCommandExecutor::plan(
        &manager,
        &context,
        &[
            IncomingCommand::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1".to_owned(),
                visible: true,
                timeout: -1,
            },
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::RefreshInventory {
                category: "floor".to_owned(),
            },
            IncomingCommand::WalkTo { x: 3, y: 4 },
        ],
    );

    assert_eq!(
        effects,
        vec![
            IncomingExecutionEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1".to_owned(),
                visible: true,
                timeout: -1,
            },
            IncomingExecutionEffect::ResetAfkTimer,
            IncomingExecutionEffect::RefreshInventory {
                category: "floor".to_owned(),
            },
            IncomingExecutionEffect::WalkTo { x: 3, y: 4 },
        ]
    );
}

#[test]
fn invokes_colon_chat_through_command_manager() {
    let mut manager = CommandManager::new();
    manager.load();
    let context = CommandContext::with_room_user(RoomUserCommandState::new(true, false, 2, 1.0));

    let effects = IncomingCommandExecutor::plan(
        &manager,
        &context,
        &[IncomingCommand::Talk {
            mode: "CHAT".to_owned(),
            message: ":sit".to_owned(),
        }],
    );

    assert_eq!(
        effects,
        vec![
            IncomingExecutionEffect::Command(CommandEffect::RemoveRoomStatus {
                key: "dance".to_owned(),
            }),
            IncomingExecutionEffect::Command(CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1".to_owned(),
                infinite: true,
                duration: -1,
            }),
            IncomingExecutionEffect::Command(CommandEffect::MarkRoomNeedsUpdate),
        ]
    );
}

#[test]
fn preserves_colon_whispers_as_chat_text() {
    let mut manager = CommandManager::new();
    manager.load();
    let context = CommandContext::with_room_user(RoomUserCommandState::new(true, false, 2, 1.0));

    let effects = IncomingCommandExecutor::plan(
        &manager,
        &context,
        &[IncomingCommand::Talk {
            mode: "WHISPER".to_owned(),
            message: ":sit".to_owned(),
        }],
    );

    assert_eq!(
        effects,
        vec![IncomingExecutionEffect::Talk {
            mode: "WHISPER".to_owned(),
            message: ":sit".to_owned(),
        }]
    );
}

#[test]
fn preserves_unmapped_commands_for_later_domain_wiring() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(&manager, &context, &[IncomingCommand::MarkRoomNeedsUpdate],),
        vec![IncomingExecutionEffect::MarkRoomNeedsUpdate]
    );
}

#[test]
fn plans_password_operations_for_authentication_commands() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(
            &manager,
            &context,
            &[
                IncomingCommand::Login {
                    username: "alice".to_owned(),
                    password: "secret".to_owned(),
                    room_login: false,
                },
                IncomingCommand::RegisterPlayer {
                    name: "bob".to_owned(),
                    password: "door".to_owned(),
                    email: "bob@example.test".to_owned(),
                    mission: "hello".to_owned(),
                    figure: "hd-100".to_owned(),
                    sex: "M".to_owned(),
                    birthday: "1990-01-01".to_owned(),
                },
                IncomingCommand::UpdateProfile {
                    password: "changed".to_owned(),
                    email: "alice@example.test".to_owned(),
                    figure: "hd-200".to_owned(),
                    mission: "new".to_owned(),
                    sex: "F".to_owned(),
                },
            ],
        ),
        vec![
            IncomingExecutionEffect::Password(PasswordAction::VerifyLogin {
                username: "alice".to_owned(),
                password: "secret".to_owned(),
                room_login: false,
            }),
            IncomingExecutionEffect::Password(PasswordAction::HashRegistration {
                username: "bob".to_owned(),
                password: "door".to_owned(),
                email: "bob@example.test".to_owned(),
                mission: "hello".to_owned(),
                figure: "hd-100".to_owned(),
                sex: "M".to_owned(),
                birthday: "1990-01-01".to_owned(),
            }),
            IncomingExecutionEffect::Password(PasswordAction::HashProfileUpdate {
                user_id: None,
                password: "changed".to_owned(),
                email: "alice@example.test".to_owned(),
                figure: "hd-200".to_owned(),
                mission: "new".to_owned(),
                sex: "F".to_owned(),
            }),
        ]
    );
}

#[test]
fn translates_messenger_commands_to_execution_effects() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(
            &manager,
            &context,
            &[
                IncomingCommand::GoAway,
                IncomingCommand::RequestBuddy {
                    username: "alice".to_owned(),
                },
                IncomingCommand::AcceptBuddy {
                    username: "bob".to_owned(),
                },
                IncomingCommand::DeclineBuddy {
                    username: "cora".to_owned(),
                },
                IncomingCommand::RemoveBuddy {
                    username: "drew".to_owned(),
                },
                IncomingCommand::SendMessengerMessage {
                    receiver_ids: vec![1, 2],
                    message: "hello".to_owned(),
                },
                IncomingCommand::InitMessenger,
            ],
        ),
        vec![
            IncomingExecutionEffect::GoAway,
            IncomingExecutionEffect::RequestBuddy {
                username: "alice".to_owned(),
            },
            IncomingExecutionEffect::AcceptBuddy {
                username: "bob".to_owned(),
            },
            IncomingExecutionEffect::DeclineBuddy {
                username: "cora".to_owned(),
            },
            IncomingExecutionEffect::RemoveBuddy {
                username: "drew".to_owned(),
            },
            IncomingExecutionEffect::SendMessengerMessage {
                receiver_ids: vec![1, 2],
                message: "hello".to_owned(),
            },
            IncomingExecutionEffect::InitMessenger,
        ]
    );
}

#[test]
fn translates_room_access_and_item_commands_to_execution_effects() {
    let manager = CommandManager::new();
    let context = CommandContext::new();

    assert_eq!(
        IncomingCommandExecutor::plan(
            &manager,
            &context,
            &[
                IncomingCommand::EnterDoor { item_id: 10 },
                IncomingCommand::LetUserIn {
                    username: "alice".to_owned(),
                },
                IncomingCommand::RemoveItem { item_id: 11 },
                IncomingCommand::AssignRights {
                    username: "bob".to_owned(),
                },
                IncomingCommand::RemoveRights {
                    username: "cora".to_owned(),
                },
                IncomingCommand::KickUser {
                    username: "drew".to_owned(),
                },
                IncomingCommand::AddWallItem {
                    sprite: "poster".to_owned(),
                    wall_position: ":w=1,2".to_owned(),
                    extra_data: "0".to_owned(),
                },
                IncomingCommand::ReturnItemToInventory { item_id: 12 },
            ],
        ),
        vec![
            IncomingExecutionEffect::EnterDoor { item_id: 10 },
            IncomingExecutionEffect::LetUserIn {
                username: "alice".to_owned(),
            },
            IncomingExecutionEffect::RemoveItem { item_id: 11 },
            IncomingExecutionEffect::AssignRights {
                username: "bob".to_owned(),
            },
            IncomingExecutionEffect::RemoveRights {
                username: "cora".to_owned(),
            },
            IncomingExecutionEffect::KickUser {
                username: "drew".to_owned(),
            },
            IncomingExecutionEffect::AddWallItem {
                sprite: "poster".to_owned(),
                wall_position: ":w=1,2".to_owned(),
                extra_data: "0".to_owned(),
            },
            IncomingExecutionEffect::ReturnItemToInventory { item_id: 12 },
            IncomingExecutionEffect::RefreshInventory {
                category: "last".to_owned(),
            },
        ]
    );
}
