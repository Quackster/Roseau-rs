use super::add_item::*;
use crate::protocol::NettyRequest;

#[test]
fn records_add_wall_item_command() {
    let mut context = IncomingContext::new();
    AddItem.handle(
        &mut context,
        &NettyRequest::from_content("ADDITEM /post.it/frontwall/FFFF31 note"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::AddWallItem {
                sprite: "post.it".to_owned(),
                wall_position: "frontwall".to_owned(),
                extra_data: "FFFF31 note".to_owned(),
            }
        ]
    );
}
