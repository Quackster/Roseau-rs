use super::pool_lift_interactor::*;
use crate::game::item::ItemDefinition;

fn lift() -> Item {
    Item::new(
        11,
        1,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "poolLift", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn closes_lift_and_records_ticket_side_effects() {
    assert_eq!(
        PoolLiftInteractor.on_stopped_walking(&lift(), Position::new(1, 1, 0.0)),
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: 11,
                program: "close".to_owned(),
            },
            ItemInteractionEffect::LockTiles { item_id: 11 },
            ItemInteractionEffect::SendJumpingPlaceOk,
            ItemInteractionEffect::SetCanWalk { can_walk: false },
            ItemInteractionEffect::DecrementTickets { amount: 1 },
            ItemInteractionEffect::SendTickets,
            ItemInteractionEffect::SavePlayer,
        ]
    );
}

#[test]
fn maps_jump_performance_to_jump_data_effect() {
    assert_eq!(
        PoolLiftInteractor::jump_performance("alice", "jump=1"),
        vec![ItemInteractionEffect::SendJumpData {
            username: "alice".to_owned(),
            data: "jump=1".to_owned(),
        }]
    );
}
