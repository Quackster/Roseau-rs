use super::register::*;
use crate::protocol::NettyRequest;

fn registration_body(name: &str) -> String {
    [
        format!("name={name}"),
        "password=secret".to_owned(),
        "email=a@example.com".to_owned(),
        "figure=sd=001/0".to_owned(),
        "directMail=0".to_owned(),
        "birthday=08.08.1997".to_owned(),
        "phonenumber=+44".to_owned(),
        "customData=hello\u{000b}world".to_owned(),
        "has_read_agreement=1".to_owned(),
        "sex=Male".to_owned(),
        "country=".to_owned(),
    ]
    .join("\r")
}

#[test]
fn records_register_player_command() {
    let mut context = IncomingContext::new();
    Register.handle(
        &mut context,
        &NettyRequest::from_content(&format!("REGISTER {}", registration_body("alice"))),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RegisterPlayer {
            name: "alice".to_owned(),
            password: "secret".to_owned(),
            email: "a@example.com".to_owned(),
            mission: "hello world".to_owned(),
            figure: "sd=001/0".to_owned(),
            sex: "Male".to_owned(),
            birthday: "08.08.1997".to_owned(),
        }]
    );
}

#[test]
fn rejects_unapproved_register_name() {
    let mut context = IncomingContext::new();
    Register.handle(
        &mut context,
        &NettyRequest::from_content(&format!("REGISTER {}", registration_body("MOD-alice"))),
    );

    assert!(context.commands().is_empty());
}

#[test]
fn preserves_java_split_and_substring_registration_field_semantics() {
    let body = registration_body("alice")
        .replace("name=alice", "name=alice=ignored")
        .replace("password=secret", "password=sec=ret")
        .replace("email=a@example.com", "email=a=b@example.com")
        .replace("figure=sd=001/0", "figure=sd=001/0=tail")
        .replace("birthday=08.08.1997", "birthday=08=08=1997")
        .replace("customData=hello\u{000b}world", "customData=hello=world")
        .replace("sex=Male", "sex=Male=ignored");
    let mut context = IncomingContext::new();
    Register.handle(
        &mut context,
        &NettyRequest::from_content(&format!("REGISTER {body}")),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RegisterPlayer {
            name: "alice".to_owned(),
            password: "sec".to_owned(),
            email: "a".to_owned(),
            mission: "hello=world".to_owned(),
            figure: "sd=001/0=tail".to_owned(),
            sex: "Male".to_owned(),
            birthday: "08".to_owned(),
        }]
    );
}
