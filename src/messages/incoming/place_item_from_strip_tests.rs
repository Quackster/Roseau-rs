use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_wall_item_placement() {
    let mut context = IncomingContext::new();
    PlaceItemFromStrip.handle(
        &mut context,
        &NettyRequest::from_content("PLACEITEMFROMSTRIP 42 frontwall 1,2,3"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::PlaceWallItemFromInventory {
                item_id: 42,
                wall_position: "frontwall 1,2,3".to_owned(),
            }
        ]
    );
}
