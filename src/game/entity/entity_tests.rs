use super::entity::*;
use crate::game::player::{Bot, Player};
use crate::game::room::model::Position;

#[test]
fn player_exposes_entity_contract() {
    let player = Player::new(12, 30000);

    assert_eq!(Entity::entity_type(&player), EntityType::Player);
    assert_eq!(Entity::details(&player).id(), -1);
    assert!(Entity::room_user(&player).is_none());
}

#[test]
fn bot_exposes_entity_contract() {
    let bot = Bot::new(Position::new(1, 2, 0.0), Vec::new(), Vec::new(), Vec::new());

    assert_eq!(Entity::entity_type(&bot), EntityType::Bot);
    assert_eq!(Entity::details(&bot).id(), -1);
    assert!(Entity::room_user(&bot).is_none());
}
