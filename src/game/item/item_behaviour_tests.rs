use super::item_behaviour::*;

#[test]
fn parses_java_item_behaviour_flags() {
    let behaviour = ItemBehaviour::parse("SIFWCBKPEMGTHVJNDXLY");

    assert!(behaviour.is_stuff());
    assert!(behaviour.is_item());
    assert!(behaviour.is_on_floor());
    assert!(behaviour.is_on_wall());
    assert!(behaviour.can_sit_on_top());
    assert!(behaviour.can_lay_on_top());
    assert!(behaviour.can_stand_on_top());
    assert!(behaviour.is_passive_object());
    assert!(behaviour.is_invisible());
    assert!(behaviour.is_trigger());
    assert!(behaviour.requires_rights_for_interaction());
    assert!(behaviour.requires_touching_for_interaction());
    assert!(behaviour.can_stack_on_top());
    assert!(behaviour.is_decoration());
    assert!(behaviour.is_post_it());
    assert!(behaviour.is_photo());
    assert!(behaviour.is_door());
    assert!(behaviour.is_teleporter());
    assert!(behaviour.is_dice());
    assert!(behaviour.is_prize_trophy());
}

#[test]
fn serialises_flags_in_java_order() {
    let behaviour = ItemBehaviour::parse("YXLDSIFW");

    assert_eq!(behaviour.to_string(), "SIFWDXLY");
}
