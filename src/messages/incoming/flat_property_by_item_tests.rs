use super::flat_property_by_item::*;
use crate::protocol::NettyRequest;

#[test]
fn records_decoration_item_command() {
    let mut context = IncomingContext::new();
    FlatPropertyByItem.handle(
        &mut context,
        &NettyRequest::from_content("FLATPROPERTYBYITEM /wallpaper/42"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::ApplyDecoration {
            decoration: "wallpaper".to_owned(),
            item_id: 42,
        }]
    );
}

#[test]
fn ignores_unknown_decoration_type() {
    let mut context = IncomingContext::new();
    FlatPropertyByItem.handle(
        &mut context,
        &NettyRequest::from_content("FLATPROPERTYBYITEM /ceiling/42"),
    );

    assert!(context.commands().is_empty());
}
