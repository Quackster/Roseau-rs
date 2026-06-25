use super::blank_interactor::*;
use crate::game::item::ItemDefinition;

fn item() -> Item {
    Item::new(
        1,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "mat", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn clears_sitting_and_laying_statuses_on_stop() {
    assert_eq!(
        BlankInteractor.on_stopped_walking(&item(), Position::new(1, 1, 0.0)),
        vec![
            ItemInteractionEffect::RemoveStatus {
                status: "sit".to_owned(),
            },
            ItemInteractionEffect::RemoveStatus {
                status: "lay".to_owned(),
            },
        ]
    );
}
