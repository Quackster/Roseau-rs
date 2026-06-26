use std::fmt::{self, Display};

use crate::game::pathfinder::get_affected_tiles_at;
use crate::game::room::model::Position;

use super::ItemDefinition;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub(super) id: i32,
    room_id: i32,
    pub(super) target_teleporter_id: i32,
    pub(super) position: Position,
    item_data: String,
    pub(super) custom_data: Option<String>,
    pub(super) wall_position: Option<String>,
    pub(super) definition: ItemDefinition,
    owner_id: i32,
    current_program: Option<String>,
}

impl Item {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        room_id: i32,
        owner_id: i32,
        x: impl Into<String>,
        y: i32,
        z: f64,
        rotation: i32,
        definition: ItemDefinition,
        item_data: impl Into<String>,
        custom_data: Option<String>,
    ) -> Result<Self, ParseItemError> {
        let x = x.into();
        let position = if definition.behaviour().is_on_wall() {
            Position::new(-1, -1, 0.0)
        } else {
            let mut position =
                Position::new(x.parse().map_err(|_| ParseItemError::InvalidX)?, y, z);
            position.set_rotation(rotation);
            position
        };

        let target_teleporter_id = target_teleporter_id(&definition, custom_data.as_deref());

        Ok(Self {
            id,
            room_id,
            owner_id,
            wall_position: definition.behaviour().is_on_wall().then_some(x),
            position,
            item_data: item_data.into(),
            custom_data,
            definition,
            target_teleporter_id,
            current_program: None,
        })
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn room_id(&self) -> i32 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    pub fn target_teleporter_id(&self) -> i32 {
        self.target_teleporter_id
    }

    pub fn set_target_teleporter_id(&mut self, target_teleporter_id: i32) {
        self.target_teleporter_id = target_teleporter_id;
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn item_data(&self) -> &str {
        &self.item_data
    }

    pub fn set_item_data(&mut self, item_data: impl Into<String>) {
        self.item_data = item_data.into();
    }

    pub fn custom_data(&self) -> Option<&str> {
        self.custom_data.as_deref()
    }

    pub fn set_custom_data(&mut self, custom_data: impl Into<String>) {
        let mut custom_data = custom_data.into();
        if custom_data.chars().count() > 400 {
            custom_data = custom_data.chars().take(400).collect();
        }

        self.custom_data = Some(custom_data);
        self.target_teleporter_id =
            target_teleporter_id(&self.definition, self.custom_data.as_deref());
    }

    pub fn wall_position(&self) -> Option<&str> {
        self.wall_position.as_deref()
    }

    pub fn set_wall_position(&mut self, wall_position: impl Into<String>) {
        self.wall_position = Some(wall_position.into());
    }

    pub fn definition(&self) -> &ItemDefinition {
        &self.definition
    }

    pub fn owner_id(&self) -> i32 {
        self.owner_id
    }

    pub fn set_owner_id(&mut self, owner_id: i32) {
        self.owner_id = owner_id;
    }

    pub fn current_program(&self) -> Option<&str> {
        self.current_program.as_deref()
    }

    pub fn set_current_program(&mut self, current_program: Option<String>) {
        self.current_program = current_program;
    }

    pub fn padding(&self) -> String {
        "0".repeat(self.definition.sprite().chars().count())
    }

    pub fn affected_tiles(&self) -> Vec<Position> {
        get_affected_tiles_at(
            self.definition.length(),
            self.definition.width(),
            self.position.x(),
            self.position.y(),
            self.position.rotation(),
        )
    }

    pub fn total_height(&self) -> f64 {
        self.position.z() + self.definition.height()
    }

    pub fn has_entity_collision(&self, x: i32, y: i32) -> bool {
        (self.position.x() == x && self.position.y() == y)
            || self
                .affected_tiles()
                .iter()
                .any(|tile| tile.x() == x && tile.y() == y)
    }

    pub fn can_walk(&self, pool_figure_available: bool) -> bool {
        let behaviour = self.definition.behaviour();

        behaviour.can_sit_on_top()
            || behaviour.can_stand_on_top()
            || behaviour.can_lay_on_top()
            || (behaviour.is_teleporter()
                && self.definition.data_class() == "DOOROPEN"
                && self.custom_data.as_deref() == Some("TRUE"))
            || matches!(
                self.definition.sprite(),
                "poolBooth" | "stair" | "poolQueue"
            )
            || (matches!(
                self.definition.sprite(),
                "poolLift" | "poolEnter" | "poolExit"
            ) && pool_figure_available)
    }

    pub fn lock_coordinate_overrides(&self) -> Vec<(i32, i32)> {
        self.custom_data
            .as_deref()
            .map(|custom_data| {
                custom_data
                    .split(' ')
                    .filter_map(|coordinate| {
                        let (x, y) = coordinate.split_once(',')?;
                        Some((x.parse().ok()?, y.parse().ok()?))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseItemError {
    InvalidX,
}

impl Display for ParseItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidX => write!(f, "item x coordinate is invalid"),
        }
    }
}

impl std::error::Error for ParseItemError {}

fn target_teleporter_id(definition: &ItemDefinition, custom_data: Option<&str>) -> i32 {
    if definition.behaviour().is_teleporter() {
        custom_data
            .and_then(|custom_data| custom_data.parse::<i32>().ok())
            .unwrap_or(0)
    } else {
        0
    }
}
