use crate::game::item::interactors::{ItemInteractionEffect, ItemInteractionEffectRoomExecutor};
use crate::game::item::{Item, ItemDefinition};
use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::RoomMapping;

fn model() -> RoomModel {
    RoomModel::new("room", "0000 0000 0000 0000", 0, 0, 0, 0, false, false).unwrap()
}

fn item(id: i32, sprite: &str, x: &str, y: i32, custom_data: Option<String>) -> Item {
    Item::new(
        id,
        1,
        1,
        x,
        y,
        0.0,
        0,
        ItemDefinition::new(id, sprite, "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        custom_data,
    )
    .unwrap()
}

fn user() -> RoomUser {
    let mut user = RoomUser::new(7, "Alice", "hd-100", "hello", Some("pool"));
    user.set_room_id(1);
    user.set_position(Position::new(0, 0, 0.0));
    user
}

#[test]
fn locks_and_unlocks_item_tiles() {
    let mut mapping = RoomMapping::new(model());
    let lock = item(10, "poolLift", "1", 1, Some("2,2".to_owned()));
    let items = vec![lock.clone()];
    mapping.regenerate_collision_maps(items.clone());
    let mut user = user();

    ItemInteractionEffectRoomExecutor::apply(
        &mut user,
        &ItemInteractionEffect::LockTiles { item_id: 10 },
        &mut mapping,
        &items,
        &[],
        true,
        1,
    );

    assert!(mapping.tile(1, 1).unwrap().has_override_lock());
    assert!(mapping.tile(2, 2).unwrap().has_override_lock());

    ItemInteractionEffectRoomExecutor::apply(
        &mut user,
        &ItemInteractionEffect::UnlockTiles { item_id: 10 },
        &mut mapping,
        &items,
        &[],
        true,
        1,
    );

    assert!(!mapping.tile(1, 1).unwrap().has_override_lock());
    assert!(!mapping.tile(2, 2).unwrap().has_override_lock());
}

#[test]
fn walks_to_valid_target_with_built_path() {
    let mut mapping = RoomMapping::new(model());
    let mut user = user();

    let effects = ItemInteractionEffectRoomExecutor::apply(
        &mut user,
        &ItemInteractionEffect::WalkTo { x: 3, y: 0 },
        &mut mapping,
        &[],
        &[],
        false,
        0,
    );

    assert!(effects.is_empty());
    assert!(user.is_walking());
    assert_eq!(user.goal(), Some(Position::new(3, 0, 0.0)));
    assert_eq!(user.path().back().copied(), Some(Position::new(3, 0, 0.0)));
}

#[test]
fn builds_path_to_existing_goal_for_pool_ladder_effect() {
    let mut mapping = RoomMapping::new(model());
    let mut user = user();
    user.set_goal(Some(Position::new(0, 3, 0.0)));

    ItemInteractionEffectRoomExecutor::apply(
        &mut user,
        &ItemInteractionEffect::BuildPathToGoal,
        &mut mapping,
        &[],
        &[],
        true,
        0,
    );

    assert!(user.is_walking());
    assert_eq!(user.goal(), Some(Position::new(0, 3, 0.0)));
    assert_eq!(user.path().back().copied(), Some(Position::new(0, 3, 0.0)));
}

#[test]
fn sends_no_ticket_effect_for_pool_targets() {
    let mut mapping = RoomMapping::new(model());
    let lift = item(11, "poolLift", "1", 0, None);
    let items = vec![lift];
    mapping.regenerate_collision_maps(items.clone());
    let mut user = user();

    let effects = ItemInteractionEffectRoomExecutor::apply(
        &mut user,
        &ItemInteractionEffect::WalkTo { x: 1, y: 0 },
        &mut mapping,
        &items,
        &[],
        true,
        0,
    );

    assert_eq!(effects, vec![RoomUserEffect::NotEnoughTickets]);
    assert!(!user.is_walking());
}

#[test]
fn ignores_effects_owned_by_other_boundaries() {
    let mut mapping = RoomMapping::new(model());
    let mut user = user();

    let effects = ItemInteractionEffectRoomExecutor::apply_all(
        &mut user,
        &[
            ItemInteractionEffect::ShowProgram {
                item_id: 1,
                program: "open".to_owned(),
            },
            ItemInteractionEffect::SendTickets,
            ItemInteractionEffect::SavePlayer,
        ],
        &mut mapping,
        &[],
        &[],
        true,
        0,
    );

    assert!(effects.is_empty());
    assert!(!user.is_walking());
}
