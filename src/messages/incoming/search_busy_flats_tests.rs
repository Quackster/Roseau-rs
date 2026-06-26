use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_busy_flat_search_multiplier() {
    let mut context = IncomingContext::new();
    SearchBusyFlats.handle(
        &mut context,
        &NettyRequest::from_content("SEARCHBUSYFLATS /2,whatever"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SearchBusyFlats { multiplier: 2 }]
    );
}

#[test]
fn records_empty_busy_flat_fallback_for_empty_body() {
    let mut context = IncomingContext::new();
    SearchBusyFlats.handle(&mut context, &NettyRequest::from_content("SEARCHBUSYFLATS"));

    assert_eq!(context.commands(), &[IncomingCommand::EmptySearchBusyFlats]);
}

#[test]
fn records_empty_busy_flat_fallback_for_malformed_body() {
    let mut context = IncomingContext::new();
    SearchBusyFlats.handle(
        &mut context,
        &NettyRequest::from_content("SEARCHBUSYFLATS /not-a-page,whatever"),
    );

    assert_eq!(context.commands(), &[IncomingCommand::EmptySearchBusyFlats]);
}
