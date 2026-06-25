use super::bot::*;

#[test]
fn finds_trigger_case_insensitively() {
    let bot = Bot::new(
        Position::new(2, 3, 0.0),
        vec![],
        vec![],
        vec!["coffee".to_owned(), "tea".to_owned()],
    );

    assert_eq!(bot.contains_trigger("Can I have COFFEE?"), Some("coffee"));
    assert_eq!(bot.contains_trigger("water"), None);
}

#[test]
fn replaces_username_and_item_tokens_in_response() {
    let bot = Bot::new(
        Position::new(2, 3, 0.0),
        vec![(1, 1)],
        vec!["Hello %username%, here is %item%".to_owned()],
        vec![],
    );

    assert_eq!(
        bot.first_response("alice", "tea"),
        Some("Hello alice, here is tea".to_owned())
    );
    assert_eq!(bot.response_at(1, "alice", "tea"), None);
}

#[test]
fn exposes_bot_metadata() {
    let bot = Bot::new(
        Position::with_rotation(2, 3, 0.0, 4),
        vec![(2, 4), (3, 4)],
        vec![],
        vec![],
    );

    assert_eq!(bot.entity_type(), EntityType::Bot);
    assert_eq!(bot.start_position(), Position::with_rotation(2, 3, 0.0, 4));
    assert_eq!(bot.positions(), &[(2, 4), (3, 4)]);
}
