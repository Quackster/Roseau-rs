use super::bed_interactor::*;
use crate::game::item::ItemDefinition;

fn bed(rotation: i32) -> Item {
    Item::new(
        1,
        1,
        1,
        "5",
        6,
        0.0,
        rotation,
        ItemDefinition::new(1, "bed", "", 2, 1, 0.25, "SFB", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn finds_java_pillow_tiles() {
    let item = bed(0);

    assert_eq!(
        BedInteractor::valid_pillow_tiles(&item),
        vec![Position::new(5, 6, 0.0), Position::new(6, 6, 0.0)]
    );
    assert!(BedInteractor::is_valid_pillow_tile(
        &item,
        Position::new(6, 6, 0.0)
    ));
}

#[test]
fn lays_player_down_on_valid_pillow_tile() {
    let effects = BedInteractor.on_stopped_walking(&bed(2), Position::new(5, 7, 0.0));

    assert!(effects.contains(&ItemInteractionEffect::SetBodyRotation { rotation: 2 }));
    assert!(effects.contains(&ItemInteractionEffect::SetStatus {
        status: "lay".to_owned(),
        value: " 1.75 null".to_owned(),
        persistent: true,
        ticks: -1,
    }));
}

#[test]
fn moves_player_to_pillow_tile_before_retriggering() {
    assert_eq!(
        BedInteractor.on_stopped_walking(&bed(0), Position::new(8, 9, 0.0)),
        vec![
            ItemInteractionEffect::SetPosition {
                position: Position::new(5, 6, 0.0),
            },
            ItemInteractionEffect::TriggerCurrentItem,
        ]
    );
}
