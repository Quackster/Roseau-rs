use crate::game::item::Item;
use crate::game::room::model::RoomModel;
use crate::game::room::{RoomOccupant, RoomTile};

#[derive(Debug, Clone, PartialEq)]
pub struct RoomMapping {
    model: RoomModel,
    tiles: Vec<Vec<RoomTile>>,
    room_walkway_ids: Vec<i32>,
}

impl RoomMapping {
    pub fn new(model: RoomModel) -> Self {
        let mut mapping = Self {
            model,
            tiles: Vec::new(),
            room_walkway_ids: Vec::new(),
        };
        mapping.regenerate_collision_maps(Vec::<Item>::new());
        mapping
    }

    pub fn regenerate_collision_maps(&mut self, items: impl IntoIterator<Item = Item>) {
        self.tiles = vec![vec![RoomTile::new(); self.model.map_size_y()]; self.model.map_size_x()];

        for y in 0..self.model.map_size_y() {
            for x in 0..self.model.map_size_x() {
                self.tiles[x][y].set_height(self.model.height(x as i32, y as i32));
            }
        }

        let mut items = items.into_iter().collect::<Vec<_>>();
        items.sort_by(|left, right| {
            left.position()
                .z()
                .partial_cmp(&right.position().z())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for item in &items {
            if item.definition().behaviour().is_on_wall() {
                continue;
            }

            let position = item.position();
            let Some(tile) = self.tile_mut(position.x(), position.y()) else {
                continue;
            };

            tile.add_item_id(item.id());

            if tile.height() < item.total_height()
                || item.definition().behaviour().is_passive_object()
            {
                tile.set_height(item.total_height());
                tile.set_highest_item_id(Some(item.id()));

                for affected_position in item.affected_tiles() {
                    if affected_position.x() == position.x()
                        && affected_position.y() == position.y()
                    {
                        continue;
                    }

                    let Some(affected_tile) =
                        self.tile_mut(affected_position.x(), affected_position.y())
                    else {
                        continue;
                    };

                    if affected_tile.highest_item_id().is_some()
                        && affected_tile.height() > item.total_height()
                    {
                        continue;
                    }

                    affected_tile.set_height(item.total_height());
                    affected_tile.set_highest_item_id(Some(item.id()));
                }
            }

            for (x, y) in item.lock_coordinate_overrides() {
                if let Some(tile) = self.tile_mut(x, y) {
                    tile.set_override_lock(true);
                }
            }
        }
    }

    pub fn is_valid_tile(
        &self,
        entity_id: i32,
        x: i32,
        y: i32,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
    ) -> bool {
        let Some(tile) = self.tile(x, y) else {
            return false;
        };

        if tile.has_override_lock() {
            return false;
        }

        let mut tile_valid = !self.model.is_blocked(x, y);

        if let Some(item_id) = tile.highest_item_id() {
            if let Some(item) = items.iter().find(|item| item.id() == item_id) {
                tile_valid = item.can_walk(pool_figure_available);
            }
        }

        if self
            .occupant_at(x, y, occupants)
            .is_some_and(|occupant| occupant.entity_id() != entity_id)
        {
            tile_valid = false;
        }

        if self
            .occupant_goal_at(x, y, occupants)
            .is_some_and(|occupant| occupant.entity_id() != entity_id)
        {
            tile_valid = false;
        }

        tile_valid
    }

    pub fn stack_height(&self, x: i32, y: i32) -> f64 {
        self.tile(x, y).map(RoomTile::height).unwrap_or(0.0)
    }

    pub fn tile(&self, x: i32, y: i32) -> Option<&RoomTile> {
        if self.model.invalid_xy_coords(x, y) {
            None
        } else {
            Some(&self.tiles[x as usize][y as usize])
        }
    }

    pub fn highest_item_id(&self, x: i32, y: i32) -> Option<i32> {
        self.tile(x, y).and_then(RoomTile::highest_item_id)
    }

    pub fn set_item_tiles_override_lock(&mut self, item: &Item, override_lock: bool) {
        if let Some(tile) = self.tile_mut(item.position().x(), item.position().y()) {
            tile.set_override_lock(override_lock);
        }

        for (x, y) in item.lock_coordinate_overrides() {
            if let Some(tile) = self.tile_mut(x, y) {
                tile.set_override_lock(override_lock);
            }
        }
    }

    pub fn apply_rotation_only_item_adjustment(
        &self,
        moved_item: &Item,
        items: &mut [Item],
    ) -> Vec<Item> {
        let position = moved_item.position();
        items
            .iter_mut()
            .filter(|item| {
                item.id() != moved_item.id()
                    && item.position().x() == position.x()
                    && item.position().y() == position.y()
                    && item.position().z() >= position.z()
            })
            .map(|item| {
                item.position_mut().set_rotation(position.rotation());
                item.clone()
            })
            .collect()
    }

    pub fn room_walkway_ids(&self) -> &[i32] {
        &self.room_walkway_ids
    }

    pub fn set_room_walkway_ids(&mut self, room_walkway_ids: impl Into<Vec<i32>>) {
        self.room_walkway_ids = room_walkway_ids.into();
    }

    pub fn model(&self) -> &RoomModel {
        &self.model
    }

    fn tile_mut(&mut self, x: i32, y: i32) -> Option<&mut RoomTile> {
        if self.model.invalid_xy_coords(x, y) {
            None
        } else {
            Some(&mut self.tiles[x as usize][y as usize])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;
    use crate::game::room::model::Position;

    fn model() -> RoomModel {
        RoomModel::new("model_a", "000 0x0 000", 0, 0, 0, 0, false, false).unwrap()
    }

    fn item(id: i32, x: &str, y: i32, z: f64, flags: &str, sprite: &str) -> Item {
        Item::new(
            id,
            1,
            1,
            x,
            y,
            z,
            0,
            ItemDefinition::new(id, sprite, "", 1, 1, 1.0, flags, "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn regenerates_tile_heights_and_highest_items() {
        let mut mapping = RoomMapping::new(model());
        let chair = item(1, "1", 0, 0.0, "SFC", "chair");
        let table = item(2, "2", 0, 0.5, "SFH", "table");

        mapping.regenerate_collision_maps([chair.clone(), table.clone()]);

        assert_eq!(mapping.highest_item_id(1, 0), Some(1));
        assert_eq!(mapping.stack_height(1, 0), chair.total_height());
        assert_eq!(mapping.highest_item_id(2, 0), Some(2));
        assert_eq!(mapping.stack_height(2, 0), table.total_height());
        assert_eq!(mapping.tile(1, 0).unwrap().item_ids(), &[1]);
    }

    #[test]
    fn applies_affected_tile_height_without_lowering_higher_stack() {
        let mut mapping = RoomMapping::new(model());
        let large = Item::new(
            1,
            1,
            1,
            "0",
            0,
            0.0,
            0,
            ItemDefinition::new(1, "sofa", "", 2, 1, 1.0, "SFC", "", "", ""),
            "",
            None,
        )
        .unwrap();

        mapping.regenerate_collision_maps([large.clone()]);

        assert_eq!(mapping.highest_item_id(1, 0), Some(1));
        assert_eq!(mapping.stack_height(1, 0), large.total_height());
    }

    #[test]
    fn validates_tiles_against_model_locks_items_occupants_and_goals() {
        let mut mapping = RoomMapping::new(model());
        let lock = Item::new(
            3,
            1,
            1,
            "0",
            2,
            0.0,
            0,
            ItemDefinition::new(3, "gate", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            Some("2,2".to_owned()),
        )
        .unwrap();
        let chair = item(1, "1", 0, 0.0, "SFC", "chair");
        let solid = item(2, "0", 1, 0.0, "SF", "solid");
        let items = vec![chair.clone(), solid.clone(), lock.clone()];
        let occupants = vec![
            RoomOccupant::new(10, Position::new(0, 0, 0.0), None),
            RoomOccupant::new(11, Position::new(2, 0, 0.0), Some(Position::new(1, 2, 0.0))),
        ];

        mapping.regenerate_collision_maps(items.clone());

        assert!(mapping.is_valid_tile(10, 1, 0, &items, &occupants, false));
        assert!(!mapping.is_valid_tile(10, 1, 1, &items, &occupants, false));
        assert!(!mapping.is_valid_tile(10, 0, 1, &items, &occupants, false));
        assert!(!mapping.is_valid_tile(12, 0, 0, &items, &occupants, false));
        assert!(!mapping.is_valid_tile(12, 1, 2, &items, &occupants, false));
        assert!(!mapping.is_valid_tile(12, 2, 2, &items, &occupants, false));
    }

    #[test]
    fn finds_nearby_occupants_and_walkway_ids() {
        let mut mapping = RoomMapping::new(model());
        mapping.set_room_walkway_ids(vec![10, 11]);
        let occupants = vec![
            RoomOccupant::new(1, Position::new(0, 0, 0.0), None),
            RoomOccupant::new(2, Position::new(1, 1, 0.0), None),
            RoomOccupant::new(3, Position::new(5, 5, 0.0), None),
        ];

        assert_eq!(mapping.room_walkway_ids(), &[10, 11]);
        assert_eq!(
            mapping.nearby_occupants(1, Position::new(0, 0, 0.0), 2, &occupants),
            vec![occupants[1]]
        );
    }

    #[test]
    fn applies_item_override_locks_to_primary_and_custom_tiles() {
        let mut mapping = RoomMapping::new(model());
        let item = Item::new(
            3,
            1,
            1,
            "0",
            2,
            0.0,
            0,
            ItemDefinition::new(3, "gate", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            Some("1,1 2,2".to_owned()),
        )
        .unwrap();

        mapping.set_item_tiles_override_lock(&item, true);

        assert!(mapping.tile(0, 2).unwrap().has_override_lock());
        assert!(mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(mapping.tile(2, 2).unwrap().has_override_lock());

        mapping.set_item_tiles_override_lock(&item, false);

        assert!(!mapping.tile(0, 2).unwrap().has_override_lock());
        assert!(!mapping.tile(1, 1).unwrap().has_override_lock());
        assert!(!mapping.tile(2, 2).unwrap().has_override_lock());
    }

    #[test]
    fn applies_java_rotation_only_adjustment_to_same_tile_items_at_or_above_height() {
        let mapping = RoomMapping::new(model());
        let mut moved = item(1, "1", 1, 0.5, "SFC", "chair");
        moved.position_mut().set_rotation(6);
        let same_tile_higher = item(2, "1", 1, 1.0, "SFC", "chair");
        let same_tile_lower = item(3, "1", 1, 0.0, "SFC", "chair");
        let different_tile = item(4, "2", 1, 1.0, "SFC", "chair");
        let mut items = vec![
            moved.clone(),
            same_tile_higher,
            same_tile_lower,
            different_tile,
        ];

        let updated = mapping.apply_rotation_only_item_adjustment(&moved, &mut items);

        assert_eq!(updated.iter().map(Item::id).collect::<Vec<_>>(), vec![2]);
        assert_eq!(items[1].position().rotation(), 6);
        assert_eq!(items[2].position().rotation(), 0);
        assert_eq!(items[3].position().rotation(), 0);
    }
}
