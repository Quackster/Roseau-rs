use super::*;
use crate::messages::IncomingCommand;
use crate::protocol::NettyRequest;

#[test]
fn registers_java_message_headers() {
    let handler = MessageHandler::new();
    let expected = [
        "ADDITEM",
        "ADDSTRIPITEM",
        "APPROVENAME",
        "ASSIGNRIGHTS",
        "CHAT",
        "CLOSE_UIMAKOPPI",
        "CREATEFLAT",
        "CRYFORHELP",
        "CarryDrink",
        "CarryItem",
        "DELETEFLAT",
        "Dance",
        "FINDUSER",
        "FLATPROPERTYBYITEM",
        "GETADFORME",
        "GETCREDITS",
        "GETFLATINFO",
        "GETORDERINFO",
        "GETSTRIP",
        "GETUNITUSERS",
        "GIVE_TICKETS",
        "GOAWAY",
        "GOTOFLAT",
        "INFORETRIEVE",
        "INITUNITLISTENER",
        "IntoDoor",
        "JUMPPERF",
        "KILLUSER",
        "LETUSERIN",
        "LOGIN",
        "LOOKTO",
        "MESSENGERINIT",
        "MESSENGER_ACCEPTBUDDY",
        "MESSENGER_ASSIGNPERSMSG",
        "MESSENGER_DECLINEBUDDY",
        "MESSENGER_MARKREAD",
        "MESSENGER_REMOVEBUDDY",
        "MESSENGER_REQUESTBUDDY",
        "MESSENGER_SENDMSG",
        "MOVESTUFF",
        "Move",
        "PLACEITEMFROMSTRIP",
        "PLACESTUFFFROMSTRIP",
        "PURCHASE",
        "REGISTER",
        "REMOVEITEM",
        "REMOVERIGHTS",
        "REMOVESTUFF",
        "SEARCHBUSYFLATS",
        "SEARCHFLAT",
        "SEARCHFLATFORUSER",
        "SETFLATINFO",
        "SETITEMDATA",
        "SETSTRIPITEMDATA",
        "SETSTUFFDATA",
        "SHOUT",
        "SPLASH_POSITION",
        "STAT",
        "STATUSOK",
        "STOP",
        "Sign",
        "TRYFLAT",
        "UNIQUEMACHINEID",
        "UPDATE",
        "UPDATEFLAT",
        "VERSIONCHECK",
        "WHISPER",
    ];

    let mut actual = handler.headers().collect::<Vec<_>>();
    actual.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn dispatches_request_to_registered_handler() {
    let handler = MessageHandler::new();
    let context = handler.handle_request(
        IncomingContext::new().in_room(true),
        &NettyRequest::from_content("CHAT hello"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            }
        ]
    );
}

#[test]
fn ignores_unknown_header() {
    let handler = MessageHandler::new();
    let context = handler.handle_request(
        IncomingContext::new().in_room(true),
        &NettyRequest::from_content("UNKNOWN hello"),
    );

    assert!(context.commands().is_empty());
    assert!(context.sent().is_empty());
}
