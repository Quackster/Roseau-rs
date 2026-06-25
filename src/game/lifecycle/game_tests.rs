use crate::game::player::{PlayerDetails, PlayerManager, PlayerSession};
use crate::game::{Game, GameLoadEffect, GameRuntimeSchedulerEffect, GameRuntimeTask};
use crate::Config;

fn hotel_config() -> Config {
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
fn load_initialises_variables_and_commands() {
    let mut game = Game::new();

    game.load(&hotel_config()).unwrap();

    assert!(game.is_loaded());
    assert_eq!(game.variables().unwrap().credits_every_amount(), 10);
    assert!(game.command_manager().has_command(":about"));
    assert!(game.room_manager().loaded_rooms().is_empty());
    assert_eq!(game.runtime_scheduler_plan().worker_threads(), 8);
}

#[test]
fn can_be_constructed_with_existing_player_manager() {
    let mut player_manager = PlayerManager::new(vec![]);
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "Alice", "mission", "figure");
    player_manager.insert(PlayerSession::new(100, 37120, details));

    let game = Game::with_player_manager(player_manager);

    assert_eq!(
        game.player_manager()
            .get_by_name("alice")
            .unwrap()
            .connection_id(),
        100
    );
    assert!(!game.is_loaded());
    assert!(game.room_manager().loaded_rooms().is_empty());
}

#[test]
fn load_effects_match_java_game_load_order() {
    let game = Game::new();

    assert_eq!(
        game.load_effects(),
        vec![
            GameLoadEffect::LoadVariables,
            GameLoadEffect::LoadRoomManager,
            GameLoadEffect::LoadItemManager,
            GameLoadEffect::LoadCatalogueManager,
            GameLoadEffect::LoadCommandManager,
            GameLoadEffect::ScheduleGameTick {
                initial_delay_secs: 0,
                interval_secs: 1,
            },
        ]
    );
}

#[test]
fn startup_scheduler_effects_match_java_worker_pool_and_game_tick() {
    let game = Game::new();

    assert_eq!(
        game.startup_scheduler_effects(),
        vec![
            GameRuntimeSchedulerEffect::CreateWorkerPool { worker_threads: 8 },
            GameRuntimeSchedulerEffect::ScheduleFixedRate {
                task: GameRuntimeTask::GameTick,
                initial_delay_ms: 0,
                interval_ms: 1_000,
            },
        ]
    );
}
