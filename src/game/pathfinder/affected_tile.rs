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
mod tests {
    use super::*;

    #[test]
    fn returns_tiles_for_square_item() {
        let tiles = get_affected_tiles_at(2, 2, 3, 4, 2);

        assert_eq!(
            tiles,
            vec![
                Position::new(3, 4, 0.0),
                Position::new(3, 5, 0.0),
                Position::new(4, 4, 0.0),
                Position::new(4, 5, 0.0),
            ]
        );
    }

    #[test]
    fn flips_non_square_dimensions_for_north_south_rotations() {
        let tiles = get_affected_tiles_at(3, 1, 10, 20, 0);

        assert_eq!(
            tiles,
            vec![
                Position::new(10, 20, 0.0),
                Position::new(11, 20, 0.0),
                Position::new(12, 20, 0.0),
            ]
        );
    }
}
