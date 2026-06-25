use super::*;
use crate::game::item::ItemDefinition;

fn teleporter(id: i32, room_id: i32, x: &str, y: i32) -> Item {
    Item::new(
        id,
        room_id,
        1,
        x,
        y,
        0.0,
        2,
        ItemDefinition::new(1, "teleport", "", 1, 1, 0.0, "SFX", "", "", "DOOROPEN"),
        "",
        Some("TRUE".to_owned()),
    )
    .unwrap()
}

#[test]
fn schedules_same_room_transfer_and_exit() {
    let current = teleporter(1, 10, "1", 1);
    let target = teleporter(2, 10, "5", 6);

    let effects = TeleporterInteractor::teleport_between_items(&current, &target, true);

    assert_eq!(
        effects[0],
        ItemInteractionEffect::SetCanWalk { can_walk: false }
    );
    assert_eq!(
        effects[1],
        ItemInteractionEffect::SendDoorOut { item_id: 1 }
    );

    let ItemInteractionEffect::Schedule {
        delay_ms,
        effects: scheduled,
    } = &effects[2]
    else {
        panic!("expected scheduled teleporter effects");
    };

    assert_eq!(*delay_ms, GameVariables::DEFAULT_TELEPORTER_DELAY);
    assert!(scheduled.contains(&ItemInteractionEffect::SetPosition {
        position: target.position(),
    }));
    assert!(scheduled.contains(&ItemInteractionEffect::SendDoorIn { item_id: 2 }));
}

#[test]
fn leave_teleporter_opens_door_and_walks_in_front() {
    let target = teleporter(2, 10, "5", 6);
    let effects = TeleporterInteractor::leave_teleporter(&target);

    assert_eq!(effects[1], ItemInteractionEffect::SendDoorIn { item_id: 2 });
    let ItemInteractionEffect::Schedule {
        delay_ms: _,
        effects: scheduled,
    } = &effects[2]
    else {
        panic!("expected scheduled leave effects");
    };
    assert!(scheduled.contains(&ItemInteractionEffect::WalkTo { x: 6, y: 6 }));
}
