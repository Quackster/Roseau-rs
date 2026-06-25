#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    SendAlert(String),
    ReloadItemDefinitions,
    RemoveRoomStatus {
        key: String,
    },
    SetRoomStatus {
        key: String,
        value: String,
        infinite: bool,
        duration: i64,
    },
    MarkRoomNeedsUpdate,
}
