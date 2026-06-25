use super::game_scheduler::*;
use crate::Config;

fn variables() -> GameVariables {
    let config = Config::parse(
        r#"
        [Register]
        user.name.chars=abc123
        user.default.credits=100
        messenger.greeting=Hello

        [Scheduler]
        credits.every.x.secs=2
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
        debug.enable=false
        "#,
    )
    .unwrap();

    GameVariables::from_config(&config).unwrap()
}

#[test]
fn emits_credit_save_ip_and_afk_effects_on_matching_tick() {
    let mut scheduler = GameScheduler::new();
    let mut afk_states = vec![RoomAfkState::new(3, 0), RoomAfkState::new(4, 2)];

    let effects = scheduler.tick(
        &variables(),
        [(1, 50), (2, 70)],
        &mut afk_states,
        "localhost",
    );

    assert_eq!(
        effects,
        vec![
            GameTickEffect::AwardCredits {
                user_id: 1,
                amount: 10,
                new_balance: 60,
            },
            GameTickEffect::SavePlayer { user_id: 1 },
            GameTickEffect::AwardCredits {
                user_id: 2,
                amount: 10,
                new_balance: 80,
            },
            GameTickEffect::SavePlayer { user_id: 2 },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 3 },
        ]
    );
    assert_eq!(afk_states[1].seconds_remaining(), 1);
    assert_eq!(scheduler.tick_rate(), 1);
}

#[test]
fn skips_credit_awards_between_configured_intervals() {
    let mut scheduler = GameScheduler::new();
    let mut afk_states = Vec::new();

    scheduler.tick(&variables(), [(1, 50)], &mut afk_states, "127.0.0.1");
    let effects = scheduler.tick(&variables(), [(1, 50)], &mut afk_states, "127.0.0.1");

    assert!(effects.is_empty());
}
