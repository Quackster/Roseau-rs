use super::pool_queue_interactor::*;
use crate::game::item::ItemDefinition;

#[test]
fn walks_to_custom_data_position() {
    let item = Item::new(
        12,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "poolQueue", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        Some("7,8".to_owned()),
    )
    .unwrap();

    assert_eq!(
        PoolQueueInteractor.on_stopped_walking(&item, Position::new(1, 1, 0.0)),
        vec![ItemInteractionEffect::WalkTo { x: 7, y: 8 }]
    );
}
