use super::room_model::*;

#[test]
fn parses_height_map_and_door_square() {
    let model = RoomModel::new("model_a", "012 x2x", 1, 1, 3, 2, false, true).unwrap();

    assert_eq!(model.map_size_x(), 3);
    assert_eq!(model.map_size_y(), 2);
    assert_eq!(model.height_map(), "012\rx2x");
    assert_eq!(model.height(1, 0), 1.0);
    assert_eq!(model.height(1, 1), 3.0);
    assert!(!model.is_blocked(1, 1));
    assert!(model.is_blocked(0, 1));
    assert_eq!(model.door_position(), Position::new(1, 1, 3.0));
    assert!(model.has_disabled_height_check());
}

#[test]
fn treats_invalid_coordinates_as_blocked_zero_height() {
    let model = RoomModel::new("model_a", "0", 0, 0, 0, 0, false, false).unwrap();

    assert_eq!(model.height(-1, 0), 0.0);
    assert!(model.is_blocked(4, 0));
}

#[test]
fn applies_java_public_room_hardcoded_closures() {
    let rows = vec!["000000000000"; 10].join(" ");
    let model = RoomModel::new("pub_a", rows, 0, 0, 0, 0, false, false).unwrap();

    assert!(model.is_blocked(9, 9));
    assert!(model.is_blocked(11, 1));
}
