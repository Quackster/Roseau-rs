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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeTask};

    #[test]
    fn defaults_to_unloaded_managers_and_empty_scheduler_report() {
        let report = GameLoadRuntimeReport::new();

        assert!(!report.variables_loaded());
        assert!(!report.room_manager_loaded());
        assert!(!report.item_manager_loaded());
        assert!(!report.catalogue_manager_loaded());
        assert!(!report.command_manager_loaded());
        assert!(report.scheduler_report().fixed_rate_tasks().is_empty());
        assert!(report.scheduler_report().delayed_tasks().is_empty());
        assert!(report.scheduler_report().cancelled_room_ids().is_empty());
    }

    #[test]
    fn records_manager_load_actions_and_scheduler_work() {
        let scheduler_effect = GameRuntimeSchedulerEffect::ScheduleFixedRate {
            task: GameRuntimeTask::GameTick,
            initial_delay_ms: 0,
            interval_ms: 1_000,
        };
        let actions = vec![
            GameLoadRuntimeAction::LoadVariables,
            GameLoadRuntimeAction::LoadRoomManager,
            GameLoadRuntimeAction::LoadItemManager,
            GameLoadRuntimeAction::LoadCatalogueManager,
            GameLoadRuntimeAction::LoadCommandManager,
            GameLoadRuntimeAction::Scheduler(scheduler_effect.clone()),
        ];

        let report = GameLoadRuntimeReport::from_actions(&actions);

        assert!(report.variables_loaded());
        assert!(report.room_manager_loaded());
        assert!(report.item_manager_loaded());
        assert!(report.catalogue_manager_loaded());
        assert!(report.command_manager_loaded());
        assert_eq!(
            report.scheduler_report().fixed_rate_tasks(),
            &[scheduler_effect]
        );
        assert!(report.scheduler_report().delayed_tasks().is_empty());
        assert!(report.readiness().ready());
    }

    #[test]
    fn exposes_readiness_for_incomplete_load_reports() {
        let report = GameLoadRuntimeReport::from_actions(&[GameLoadRuntimeAction::LoadVariables]);

        let readiness = report.readiness();

        assert!(!readiness.ready());
        assert_eq!(
            readiness.missing_steps(),
            &[
                "room_manager",
                "item_manager",
                "catalogue_manager",
                "command_manager"
            ]
        );
        assert!(!readiness.game_tick_scheduled());
    }
}
