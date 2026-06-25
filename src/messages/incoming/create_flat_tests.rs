use super::create_flat::*;
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
