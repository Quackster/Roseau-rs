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
