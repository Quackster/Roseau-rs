use super::position::*;

#[test]
fn parses_comma_position_like_java_constructor() {
    let position = Position::parse("17,19").unwrap();

    assert_eq!(position.x(), 17);
    assert_eq!(position.y(), 19);
    assert_eq!(position.z(), 0.0);
    assert_eq!(position.to_string(), "17,19");
}

#[test]
fn parses_three_part_position_for_pool_splash_body() {
    let position = Position::parse_xyz("17,18,0.5").unwrap();

    assert_eq!(position, Position::new(17, 18, 0.5));
    assert_eq!(
        Position::parse_xyz("17,18").unwrap(),
        Position::new(17, 18, 0.0)
    );
}

#[test]
fn calculates_distance_like_java_position() {
    let a = Position::new(1, 1, 0.0);
    let b = Position::new(4, 5, 0.0);

    assert_eq!(a.distance_squared(b), 25);
    assert_eq!(a.distance(b), 5);
}

#[test]
fn keeps_java_left_and_right_square_semantics() {
    let east = Position::with_rotation(10, 10, 0.0, 2);
    let west = Position::with_rotation(10, 10, 0.0, 6);

    assert_eq!(east.square_left(), Position::new(10, 11, 0.0));
    assert_eq!(east.square_right(), Position::new(10, 9, 0.0));
    assert_eq!(west.square_left(), Position::new(10, 11, 0.0));
    assert_eq!(west.square_right(), Position::new(10, 9, 0.0));
}
