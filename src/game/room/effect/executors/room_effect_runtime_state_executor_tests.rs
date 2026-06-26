use super::*;
use crate::game::item::ItemDefinition;
use crate::game::room::model::Position;

fn item(id: i32) -> Item {
    Item::new(
        id,
        7,
        0,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(id, "chair", "", 1, 1, 1.0, "SF", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

fn bot(x: i32, y: i32) -> Bot {
    Bot::new(Position::new(x, y, 0.0), vec![], vec![], vec![])
}

#[test]
fn clears_runtime_items_and_bots() {
    let mut items = vec![item(10), item(11)];
    let mut bots = vec![bot(1, 2)];

    let applied =
        RoomEffectRuntimeStateExecutor::apply(&mut items, &mut bots, &RoomEffect::ClearRuntimeData);

    assert!(applied);
    assert!(items.is_empty());
    assert!(bots.is_empty());
}

#[test]
fn ignores_non_runtime_clear_effects() {
    let mut items = vec![item(10)];
    let mut bots = vec![bot(1, 2)];

    let count = RoomEffectRuntimeStateExecutor::apply_all(
        &mut items,
        &mut bots,
        &[
            RoomEffect::RemoveLoadedRoom { room_id: 7 },
            RoomEffect::SendOwnerPrivileges { user_id: 9 },
        ],
    );

    assert_eq!(count, 0);
    assert_eq!(items.len(), 1);
    assert_eq!(bots.len(), 1);
}
