use super::*;
use crate::game::item::ItemDefinition;

fn ladder() -> Item {
    Item::new(
        13,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "poolEnter", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        Some("2,3 4,5".to_owned()),
    )
    .unwrap()
}

#[test]
fn enter_ladder_sets_swim_and_builds_warp_effects() {
    let effects = PoolLadderInteractor::new(true).on_trigger(&ladder());

    assert_eq!(
        effects[0],
        ItemInteractionEffect::SetStatus {
            status: "swim".to_owned(),
            value: String::new(),
            persistent: true,
            ticks: -1,
        }
    );
    assert!(effects.contains(&ItemInteractionEffect::SetPosition {
        position: Position::new(2, 3, 0.0),
    }));
    assert!(effects.contains(&ItemInteractionEffect::SetGoal {
        position: Position::new(4, 5, 0.0),
    }));
    assert!(effects.contains(&ItemInteractionEffect::ShowProgram {
        item_id: 13,
        program: "enter".to_owned(),
    }));
}

#[test]
fn exit_ladder_removes_swim() {
    let effects = PoolLadderInteractor::new(false).on_trigger(&ladder());

    assert_eq!(
        effects[0],
        ItemInteractionEffect::RemoveStatus {
            status: "swim".to_owned(),
        }
    );
    assert!(effects.contains(&ItemInteractionEffect::ShowProgram {
        item_id: 13,
        program: "exit".to_owned(),
    }));
}
