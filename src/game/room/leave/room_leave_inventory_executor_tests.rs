use super::*;
use crate::game::item::{Item, ItemDefinition};
use crate::game::player::PlayerDetails;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn item(id: i32) -> Item {
    Item::new(
        id,
        0,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", ""),
        "",
        None,
    )
    .unwrap()
}

fn player() -> Player {
    let mut player = Player::with_details(10, 30000, details(7, "alice"));
    player.inventory_mut().add_item(item(1));
    player.inventory_mut().add_item(item(2));
    player
}

#[test]
fn disposes_matching_player_inventory() {
    let mut player = player();

    let applied = RoomLeaveInventoryExecutor::apply(
        &mut player,
        &RoomLeaveEffect::DisposeInventory { user_id: 7 },
    );

    assert!(applied);
    assert!(player.inventory().items().is_empty());
    assert!(player.inventory().paginated_items().is_empty());
    assert_eq!(player.inventory().cursor(), 0);
}

#[test]
fn ignores_non_matching_and_non_inventory_leave_effects() {
    let mut player = player();

    let count = RoomLeaveInventoryExecutor::apply_all(
        &mut player,
        &[
            RoomLeaveEffect::DisposeInventory { user_id: 8 },
            RoomLeaveEffect::BroadcastLogout {
                username: "alice".to_owned(),
            },
        ],
    );

    assert_eq!(count, 0);
    assert_eq!(player.inventory().items().len(), 2);
}
