use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerEffect {
    SendStatus(Vec<i32>),
    SetHeadRotation {
        entity_id: i32,
        rotation: i32,
    },
    RemoveStatus {
        entity_id: i32,
        key: String,
    },
    TickStatus {
        entity_id: i32,
        key: String,
    },
    SetStatus {
        entity_id: i32,
        key: String,
        value: String,
        infinite: bool,
        duration: i64,
    },
    MarkNeedsUpdate {
        entity_id: i32,
    },
    SetLookResetTime {
        entity_id: i32,
        ticks: i64,
    },
    SetTimeUntilNextDrink {
        entity_id: i32,
        ticks: i64,
    },
    WalkTo {
        entity_id: i32,
        x: i32,
        y: i32,
    },
    SetRotation {
        entity_id: i32,
        rotation: i32,
    },
    ShowProgram(Vec<String>),
    TargetCamera {
        player_id: i32,
        username: String,
    },
    SetCamera(i32),
    MoveTo {
        entity_id: i32,
        position: Position,
    },
    UpdateHeight {
        entity_id: i32,
        height: f64,
    },
    SetNext {
        entity_id: i32,
        position: Position,
    },
    PopPath {
        entity_id: i32,
    },
    ClearPath {
        entity_id: i32,
    },
    StopWalking {
        entity_id: i32,
    },
    TriggerCurrentItem {
        entity_id: i32,
        item_id: Option<i32>,
    },
}

impl SchedulerEffect {
    pub fn entity_id(&self) -> Option<i32> {
        match self {
            Self::SendStatus(_)
            | Self::ShowProgram(_)
            | Self::TargetCamera { .. }
            | Self::SetCamera(_) => None,
            Self::SetHeadRotation { entity_id, .. }
            | Self::RemoveStatus { entity_id, .. }
            | Self::TickStatus { entity_id, .. }
            | Self::SetStatus { entity_id, .. }
            | Self::MarkNeedsUpdate { entity_id }
            | Self::SetLookResetTime { entity_id, .. }
            | Self::SetTimeUntilNextDrink { entity_id, .. }
            | Self::WalkTo { entity_id, .. }
            | Self::SetRotation { entity_id, .. }
            | Self::MoveTo { entity_id, .. }
            | Self::UpdateHeight { entity_id, .. }
            | Self::SetNext { entity_id, .. }
            | Self::PopPath { entity_id }
            | Self::ClearPath { entity_id }
            | Self::StopWalking { entity_id }
            | Self::TriggerCurrentItem { entity_id, .. } => Some(*entity_id),
        }
    }
}
