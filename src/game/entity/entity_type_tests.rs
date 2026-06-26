use super::*;

#[test]
fn keeps_java_entity_type_class_mapping_as_names() {
    assert_eq!(EntityType::Player.rust_type_name(), "Player");
    assert_eq!(EntityType::Pet.rust_type_name(), "Entity");
    assert_eq!(EntityType::Bot.rust_type_name(), "Entity");
}
