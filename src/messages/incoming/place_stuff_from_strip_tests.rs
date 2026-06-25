use super::place_stuff_from_strip::*;
use crate::protocol::NettyRequest;

#[test]
fn records_floor_item_placement() {
    let mut context = IncomingContext::new();
    PlaceStuffFromStrip.handle(
        &mut context,
        &NettyRequest::from_content("PLACESTUFFFROMSTRIP 42 5 7"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::PlaceFloorItemFromInventory {
                item_id: 42,
                x: 5,
                y: 7,
                rotation: 0,
            }
        ]
    );
}
