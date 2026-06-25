use crate::game::catalogue::CatalogueManager;
use crate::game::commands::CommandManager;
use crate::game::item::ItemManager;
use crate::game::moderation::ModerationManager;
use crate::game::player::PlayerManager;
use crate::game::room::RoomManager;
use crate::game::{
    GameLoadEffect, GameRuntimeSchedulerEffect, GameRuntimeSchedulerPlan, GameScheduler,
    GameVariables,
};
use crate::{Config, ConfigError};

pub struct Game {
    player_manager: PlayerManager,
    room_manager: RoomManager,
    item_manager: ItemManager,
    catalogue_manager: CatalogueManager,
    command_manager: CommandManager,
    moderation_manager: ModerationManager,
    runtime_scheduler_plan: GameRuntimeSchedulerPlan,
    scheduler: GameScheduler,
    variables: Option<GameVariables>,
    loaded: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_manager: PlayerManager::new(vec![]),
            room_manager: RoomManager::new(),
            item_manager: ItemManager::new(),
            catalogue_manager: CatalogueManager::new(),
            command_manager: CommandManager::new(),
            moderation_manager: ModerationManager::new(),
            runtime_scheduler_plan: GameRuntimeSchedulerPlan::java_default(),
            scheduler: GameScheduler::new(),
            variables: None,
            loaded: false,
        }
    }

    pub fn with_player_manager(player_manager: PlayerManager) -> Self {
        Self {
            player_manager,
            ..Self::new()
        }
    }

    pub fn load(&mut self, hotel_config: &Config) -> Result<(), ConfigError> {
        self.variables = Some(GameVariables::from_config(hotel_config)?);
        self.command_manager.load();
        self.loaded = true;
        Ok(())
    }

    pub fn load_effects(&self) -> Vec<GameLoadEffect> {
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
    }

    pub fn startup_scheduler_effects(&self) -> Vec<GameRuntimeSchedulerEffect> {
        vec![
            self.runtime_scheduler_plan.construction_effect(),
            self.runtime_scheduler_plan.schedule_game_tick_effect(),
        ]
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn variables(&self) -> Option<&GameVariables> {
        self.variables.as_ref()
    }

    pub fn player_manager(&self) -> &PlayerManager {
        &self.player_manager
    }

    pub fn player_manager_mut(&mut self) -> &mut PlayerManager {
        &mut self.player_manager
    }

    pub fn room_manager(&self) -> &RoomManager {
        &self.room_manager
    }

    pub fn room_manager_mut(&mut self) -> &mut RoomManager {
        &mut self.room_manager
    }

    pub fn item_manager(&self) -> &ItemManager {
        &self.item_manager
    }

    pub fn item_manager_mut(&mut self) -> &mut ItemManager {
        &mut self.item_manager
    }

    pub fn catalogue_manager(&self) -> &CatalogueManager {
        &self.catalogue_manager
    }

    pub fn catalogue_manager_mut(&mut self) -> &mut CatalogueManager {
        &mut self.catalogue_manager
    }

    pub fn command_manager(&self) -> &CommandManager {
        &self.command_manager
    }

    pub fn command_manager_mut(&mut self) -> &mut CommandManager {
        &mut self.command_manager
    }

    pub fn moderation_manager(&self) -> &ModerationManager {
        &self.moderation_manager
    }

    pub fn runtime_scheduler_plan(&self) -> &GameRuntimeSchedulerPlan {
        &self.runtime_scheduler_plan
    }

    pub fn scheduler(&self) -> &GameScheduler {
        &self.scheduler
    }

    pub fn scheduler_mut(&mut self) -> &mut GameScheduler {
        &mut self.scheduler
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut details = crate::game::player::PlayerDetails::new();
        details.fill_basic(7, "Alice", "mission", "figure");
        player_manager.insert(crate::game::player::PlayerSession::new(100, 37120, details));

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
                    task: crate::game::GameRuntimeTask::GameTick,
                    initial_delay_ms: 0,
                    interval_ms: 1_000,
                },
            ]
        );
    }
}
