use super::chair_interactor::*;
use crate::game::item::ItemDefinition;

fn chair() -> Item {
    Item::new(
        1,
        1,
        1,
        "5",
        6,
        0.0,
        2,
        ItemDefinition::new(1, "chair", "", 1, 1, 0.75, "SFC", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn sets_sit_status_on_stop() {
    assert_eq!(
        ChairInteractor.on_stopped_walking(&chair(), Position::new(6, 6, 0.0)),
        vec![
            ItemInteractionEffect::SetBodyRotation { rotation: 2 },
            ItemInteractionEffect::RemoveStatus {
                status: "dance".to_owned(),
            },
            ItemInteractionEffect::RemoveStatus {
                status: "lay".to_owned(),
            },
            ItemInteractionEffect::SetStatus {
                status: "sit".to_owned(),
                value: " 0.75".to_owned(),
                persistent: true,
                ticks: -1,
            },
        ]
    );
}

#[test]
fn validates_neighbour_entry_priority() {
    let item = chair();
    let entity_position = Position::new(5, 7, 0.0);

    assert!(ChairInteractor::has_valid_entry(
        &item,
        entity_position,
        item.position().square_in_front(),
        true,
        false,
        false,
    ));
    assert!(!ChairInteractor::has_valid_entry(
        &item,
        entity_position,
        item.position().square_right(),
        true,
        false,
        false,
    ));
}
