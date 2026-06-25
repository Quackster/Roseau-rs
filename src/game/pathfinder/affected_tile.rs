use crate::game::room::model::Position;

pub fn get_affected_tiles_at(
    mut length: i32,
    mut width: i32,
    position_x: i32,
    position_y: i32,
    rotation: i32,
) -> Vec<Position> {
    if length != width && matches!(rotation, 0 | 4) {
        std::mem::swap(&mut length, &mut width);
    }

    let mut tiles = Vec::new();

    for x in position_x..position_x + width {
        for y in position_y..position_y + length {
            tiles.push(Position::new(x, y, 0.0));
        }
    }

    tiles
}

#[cfg(test)]
#[path = "affected_tile_tests.rs"]
mod tests;
