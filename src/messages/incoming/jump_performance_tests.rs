use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_jump_performance_data() {
    let mut context = IncomingContext::new();
    JumpPerformance.handle(
        &mut context,
        &NettyRequest::from_content("JUMPPERF a\rb\rc\rdive"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::JumpPerformance {
            data: "dive".to_owned(),
        }]
    );
}
