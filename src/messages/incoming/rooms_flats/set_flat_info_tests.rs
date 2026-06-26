use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_flat_info_update() {
    let mut context = IncomingContext::new();
    SetFlatInfo.handle(
        &mut context,
        &NettyRequest::from_content(
            "SETFLATINFO /36/description=hello\rpassword=secret\rallsuperuser=1",
        ),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SetFlatInfo {
            room_id: 36,
            description: "hello".to_owned(),
            password: "secret".to_owned(),
            all_super_user: true,
        }]
    );
}

#[test]
fn preserves_slashes_inside_flat_info_fields() {
    let mut context = IncomingContext::new();
    SetFlatInfo.handle(
        &mut context,
        &NettyRequest::from_content(
            "SETFLATINFO /36/description=hello/world\rpassword=a/b\rallsuperuser=0",
        ),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SetFlatInfo {
            room_id: 36,
            description: "hello/world".to_owned(),
            password: "a/b".to_owned(),
            all_super_user: false,
        }]
    );
}
