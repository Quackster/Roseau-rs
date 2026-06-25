use super::search_flat_for_user::*;
use crate::protocol::NettyRequest;

#[test]
fn records_user_room_search() {
    let mut context = IncomingContext::new();
    SearchFlatForUser.handle(
        &mut context,
        &NettyRequest::from_content("SEARCHFLATFORUSER /alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::SearchFlatForUser {
            username: "alice".to_owned(),
        }]
    );
}
