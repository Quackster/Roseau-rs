use super::pool_change_booth_interactor::*;
use crate::game::item::ItemDefinition;

fn booth() -> Item {
    Item::new(
        10,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "poolBooth", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn closes_booth_opens_ui_and_stops_walking() {
    assert_eq!(
        PoolChangeBoothInteractor.on_stopped_walking(&booth(), Position::new(1, 1, 0.0)),
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: 10,
                program: "close".to_owned(),
            },
            ItemInteractionEffect::LockTiles { item_id: 10 },
            ItemInteractionEffect::OpenPoolChangeBooth,
            ItemInteractionEffect::SetCanWalk { can_walk: false },
        ]
    );
}

#[test]
fn close_ui_opens_booth_and_walks_to_custom_data_position() {
    let item = Item::new(
        10,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "poolBooth", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        Some("7,8".to_owned()),
    )
    .unwrap();

    assert_eq!(
        PoolChangeBoothInteractor::close_ui(&item),
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: 10,
                program: "open".to_owned(),
            },
            ItemInteractionEffect::UnlockTiles { item_id: 10 },
            ItemInteractionEffect::SetCanWalk { can_walk: true },
            ItemInteractionEffect::WalkTo { x: 7, y: 8 },
        ]
    );
}

#[test]
fn close_ui_only_opens_booth_when_custom_data_position_is_invalid() {
    let item = booth();

    assert_eq!(
        PoolChangeBoothInteractor::close_ui(&item),
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: 10,
                program: "open".to_owned(),
            },
            ItemInteractionEffect::UnlockTiles { item_id: 10 },
        ]
    );
}
