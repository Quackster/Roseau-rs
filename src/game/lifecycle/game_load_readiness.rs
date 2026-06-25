use crate::game::{GameLoadRuntimeReport, GameRuntimeSchedulerEffect, GameRuntimeTask};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameLoadReadiness {
    missing_steps: Vec<&'static str>,
    game_tick_scheduled: bool,
}

impl GameLoadReadiness {
    pub fn from_report(report: &GameLoadRuntimeReport) -> Self {
        let mut missing_steps = Vec::new();

        if !report.variables_loaded() {
            missing_steps.push("variables");
        }
        if !report.room_manager_loaded() {
            missing_steps.push("room_manager");
        }
        if !report.item_manager_loaded() {
            missing_steps.push("item_manager");
        }
        if !report.catalogue_manager_loaded() {
            missing_steps.push("catalogue_manager");
        }
        if !report.command_manager_loaded() {
            missing_steps.push("command_manager");
        }

        let game_tick_scheduled =
            report
                .scheduler_report()
                .fixed_rate_tasks()
                .iter()
                .any(|effect| {
                    matches!(
                        effect,
                        GameRuntimeSchedulerEffect::ScheduleFixedRate {
                            task: GameRuntimeTask::GameTick,
                            ..
                        }
                    )
                });

        Self {
            missing_steps,
            game_tick_scheduled,
        }
    }

    pub fn ready(&self) -> bool {
        self.missing_steps.is_empty() && self.game_tick_scheduled
    }

    pub fn missing_steps(&self) -> &[&'static str] {
        &self.missing_steps
    }

    pub fn game_tick_scheduled(&self) -> bool {
        self.game_tick_scheduled
    }
}
