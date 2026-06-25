use super::distress_message::*;

#[test]
fn extracts_private_room_distress_text_like_java_handler() {
    let body = "/Private Room: den;0;please help;39\tden\tAlex\topen";
    let first_tab_argument = "/Private Room: den;0;please help;39";

    let message =
        DistressMessage::from_payload(RoomType::Private, "den", 39, body, first_tab_argument);

    assert_eq!(message.text(), "please help");
}

#[test]
fn extracts_public_room_distress_text_like_java_handler() {
    let message = DistressMessage::from_payload(
        RoomType::Public,
        "Habbo Lido",
        0,
        "/Habbo Lido;0;splash issue;ignored",
        "",
    );

    assert_eq!(message.text(), "splash issue");
}

#[test]
fn malformed_payloads_produce_empty_text_instead_of_panicking() {
    let private = DistressMessage::from_payload(RoomType::Private, "den", 1, "", "ab");
    let public = DistressMessage::from_payload(RoomType::Public, "Lido", 0, "xy", "");

    assert_eq!(private.text(), "");
    assert_eq!(public.text(), "");
}
