use super::search_flat::*;
use crate::protocol::NettyRequest;

#[test]
fn records_flat_name_search_query() {
    let mut context = IncomingContext::new();
    SearchFlat.handle(
        &mut context,
        &NettyRequest::from_content("SEARCHFLAT //cafe/"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SearchFlat {
            query: "cafe".to_owned(),
        }]
    );
}

#[test]
fn ignores_too_short_flat_name_search_query() {
    let mut context = IncomingContext::new();
    SearchFlat.handle(&mut context, &NettyRequest::from_content("SEARCHFLAT //a/"));

    assert!(context.commands().is_empty());
}
