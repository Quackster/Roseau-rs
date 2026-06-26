use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_try_flat_command_with_password() {
    let mut context = IncomingContext::new();
    TryFlat.handle(
        &mut context,
        &NettyRequest::from_content("TRYFLAT /42/sesame"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::TryFlat {
            room_id: 42,
            password: "sesame".to_owned(),
        }]
    );
}

#[test]
fn records_try_flat_command_without_password() {
    let mut context = IncomingContext::new();
    TryFlat.handle(&mut context, &NettyRequest::from_content("TRYFLAT /42"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::TryFlat {
            room_id: 42,
            password: String::new(),
        }]
    );
}
