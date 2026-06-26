use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_set_stuff_data_command() {
    let mut context = IncomingContext::new();
    SetStuffData.handle(
        &mut context,
        &NettyRequest::from_content("SETSTUFFDATA /42/state/open"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SetStuffData {
            item_id: 42,
            data_class: "state".to_owned(),
            custom_data: "open".to_owned(),
        }]
    );
}
