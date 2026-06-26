use super::*;

#[test]
fn stores_tile_state_without_room_runtime_reference() {
    let mut tile = RoomTile::new();
    tile.set_height(2.5);
    tile.set_override_lock(true);
    tile.add_item_id(7);
    tile.set_highest_item_id(Some(7));

    assert_eq!(tile.height(), 2.5);
    assert!(tile.has_override_lock());
    assert_eq!(tile.item_ids(), &[7]);
    assert_eq!(tile.highest_item_id(), Some(7));
}
