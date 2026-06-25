use crate::game::{GameLoadReadiness, GameLoadRuntimeAction, GameRuntimeSchedulerExecutionReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameLoadRuntimeReport {
    variables_loaded: bool,
    room_manager_loaded: bool,
    item_manager_loaded: bool,
    catalogue_manager_loaded: bool,
    command_manager_loaded: bool,
    scheduler_report: GameRuntimeSchedulerExecutionReport,
}

impl GameLoadRuntimeReport {
    pub fn new() -> Self {
        Self {
            variables_loaded: false,
            room_manager_loaded: false,
            item_manager_loaded: false,
            catalogue_manager_loaded: false,
            command_manager_loaded: false,
            scheduler_report: GameRuntimeSchedulerExecutionReport::new(),
        }
    }

    pub fn record(&mut self, action: &GameLoadRuntimeAction) {
        match action {
            GameLoadRuntimeAction::LoadVariables => self.variables_loaded = true,
            GameLoadRuntimeAction::LoadRoomManager => self.room_manager_loaded = true,
            GameLoadRuntimeAction::LoadItemManager => self.item_manager_loaded = true,
            GameLoadRuntimeAction::LoadCatalogueManager => self.catalogue_manager_loaded = true,
            GameLoadRuntimeAction::LoadCommandManager => self.command_manager_loaded = true,
            GameLoadRuntimeAction::Scheduler(effect) => self.scheduler_report.record(effect),
        }
    }

    pub fn from_actions(actions: &[GameLoadRuntimeAction]) -> Self {
        let mut report = Self::new();

        for action in actions {
            report.record(action);
        }

        report
    }

    pub fn variables_loaded(&self) -> bool {
        self.variables_loaded
    }

    pub fn room_manager_loaded(&self) -> bool {
        self.room_manager_loaded
    }

    pub fn item_manager_loaded(&self) -> bool {
        self.item_manager_loaded
    }

    pub fn catalogue_manager_loaded(&self) -> bool {
        self.catalogue_manager_loaded
    }

    pub fn command_manager_loaded(&self) -> bool {
        self.command_manager_loaded
    }

    pub fn scheduler_report(&self) -> &GameRuntimeSchedulerExecutionReport {
        &self.scheduler_report
    }

    pub fn readiness(&self) -> GameLoadReadiness {
        GameLoadReadiness::from_report(self)
    }
}

impl Default for GameLoadRuntimeReport {
    fn default() -> Self {
        Self::new()
    }
}
