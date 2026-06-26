#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameLoadEffect {
    LoadVariables,
    LoadRoomManager,
    LoadItemManager,
    LoadCatalogueManager,
    LoadCommandManager,
    ScheduleGameTick {
        initial_delay_secs: u64,
        interval_secs: u64,
    },
}
