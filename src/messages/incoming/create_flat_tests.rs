use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_create_flat_command() {
    let mut context = IncomingContext::new();
    CreateFlat.handle(
        &mut context,
        &NettyRequest::from_content("CREATEFLAT /first floor/My room/model_a/closed/1"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::CreateFlat {
            floor: "first floor".to_owned(),
            room_name: "My room".to_owned(),
            room_model: "model_a".to_owned(),
            state: 1,
            show_owner_name: true,
        }]
    );
}

#[test]
fn records_alert_for_short_room_name_like_java() {
    let mut context = IncomingContext::new();
    CreateFlat.handle(
        &mut context,
        &NettyRequest::from_content("CREATEFLAT /first floor/ab/model_a/open/1"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ClosePublicRoomConnections,
            IncomingCommand::SendAlert {
                message: "The room name needs to be at least 3 characters long".to_owned(),
            },
        ]
    );
}

#[test]
fn records_close_user_connections_for_invalid_floor_or_model_like_java() {
    let mut invalid_floor = IncomingContext::new();
    CreateFlat.handle(
        &mut invalid_floor,
        &NettyRequest::from_content("CREATEFLAT /second floor/My room/model_a/open/1"),
    );
    let mut invalid_model = IncomingContext::new();
    CreateFlat.handle(
        &mut invalid_model,
        &NettyRequest::from_content("CREATEFLAT /first floor/My room/hax/open/1"),
    );

    assert_eq!(
        invalid_floor.commands(),
        &[IncomingCommand::CloseUserConnections]
    );
    assert_eq!(
        invalid_model.commands(),
        &[IncomingCommand::CloseUserConnections]
    );
}
