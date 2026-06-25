use super::game_variables::*;

fn config() -> Config {
    Config::parse(
        r#"
        [Register]
        user.name.chars=abc123
        user.default.credits=100
        messenger.greeting=Hello

        [Scheduler]
        credits.every.x.secs=600
        credits.every.x.amount=10

        [Bot]
        bot.response.delay=1500

        [Player]
        carry.drink.time=180
        carry.drink.interval=12
        talking.lookat.distance=30
        talking.lookat.reset=6
        afk.room.kick=1800

        [Debug]
        debug.enable=true
        "#,
    )
    .unwrap()
}

#[test]
fn loads_java_game_variables_from_config() {
    let variables = GameVariables::from_config(&config()).unwrap();

    assert_eq!(variables.credits_every_secs(), 600);
    assert_eq!(variables.credits_every_amount(), 10);
    assert_eq!(variables.username_chars(), "abc123");
    assert_eq!(variables.messenger_greeting(), "Hello");
    assert_eq!(variables.bot_response_delay(), 1500);
    assert_eq!(variables.carry_drink_interval(), 12);
    assert_eq!(variables.carry_drink_time(), 180);
    assert_eq!(variables.talk_look_at_reset(), 6);
    assert_eq!(variables.talk_distance(), 30);
    assert_eq!(variables.user_default_credits(), 100);
    assert_eq!(variables.teleporter_delay(), 800);
    assert_eq!(variables.max_items_per_page(), 9);
    assert!(variables.debug_enabled());
    assert_eq!(variables.afk_room_kick(), 1800);
}
