use super::item_definition::*;

#[test]
fn lowers_height_for_non_stackable_non_seating_items() {
    let definition = ItemDefinition::new(1, "poster", "red", 1, 1, 2.5, "IW", "Poster", "", "");

    assert_eq!(definition.height(), 0.001);
    assert!(definition.behaviour().is_on_wall());
}

#[test]
fn keeps_height_for_sittable_items() {
    let definition = ItemDefinition::new(2, "chair", "blue", 1, 1, 1.25, "SFC", "Chair", "", "");

    assert_eq!(definition.height(), 1.25);
    assert!(definition.behaviour().can_sit_on_top());
}
