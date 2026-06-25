use super::update::*;
use crate::protocol::NettyRequest;

fn update_body() -> String {
    [
        "name=alice".to_owned(),
        "password=secret".to_owned(),
        "email=a@example.com".to_owned(),
        "figure=sd=001/0".to_owned(),
        "directMail=0".to_owned(),
        "birthday=08.08.1997".to_owned(),
        "phonenumber=+44".to_owned(),
        "customData=hello\u{000b}world".to_owned(),
        "has_read_agreement=1".to_owned(),
        "sex=Male".to_owned(),
    ]
    .join("\r")
}

#[test]
fn records_profile_update() {
    let mut context = IncomingContext::new();
    Update.handle(
        &mut context,
        &NettyRequest::from_content(&format!("UPDATE {}", update_body())),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UpdateProfile {
            password: "secret".to_owned(),
            email: "a@example.com".to_owned(),
            figure: "sd=001/0".to_owned(),
            mission: "hello world".to_owned(),
            sex: "Male".to_owned(),
        }]
    );
}

#[test]
fn preserves_java_split_and_substring_update_field_semantics() {
    let body = update_body()
        .replace("password=secret", "password=sec=ret")
        .replace("email=a@example.com", "email=a=b@example.com")
        .replace("figure=sd=001/0", "figure=sd=001/0=tail")
        .replace("customData=hello\u{000b}world", "customData=hello=world")
        .replace("sex=Male", "sex=Male=ignored");
    let mut context = IncomingContext::new();
    Update.handle(
        &mut context,
        &NettyRequest::from_content(&format!("UPDATE {body}")),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UpdateProfile {
            password: "sec".to_owned(),
            email: "a".to_owned(),
            figure: "sd=001/0=tail".to_owned(),
            mission: "hello=world".to_owned(),
            sex: "Male".to_owned(),
        }]
    );
}

#[test]
fn records_pool_figure_update() {
    let mut context = IncomingContext::new();
    Update.handle(
        &mut context,
        &NettyRequest::from_content("UPDATE ph_figure=abc"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::UpdatePoolFigure {
            pool_figure: "abc".to_owned(),
        }]
    );
}
