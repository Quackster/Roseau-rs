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
