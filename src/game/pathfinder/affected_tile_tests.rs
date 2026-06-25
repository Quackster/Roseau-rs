use super::affected_tile::*;

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
