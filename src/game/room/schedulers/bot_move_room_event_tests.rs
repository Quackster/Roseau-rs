use super::bot_move_room_event::*;

#[test]
fn sends_bot_back_to_start_when_players_are_nearby() {
    let mut event = BotMoveRoomEvent::new();
    let bot = BotTickState::new(
        3,
        Position::new(7, 7, 0.0),
        Position::with_rotation(1, 2, 0.0, 4),
    )
    .nearby_player_count(1);

    assert_eq!(
        event.tick(&[bot], 0),
        vec![SchedulerEffect::WalkTo {
            entity_id: 3,
            x: 1,
            y: 2
        }]
    );
}

#[test]
fn rotates_bot_to_start_rotation_when_already_home() {
    let mut event = BotMoveRoomEvent::new();
    let bot = BotTickState::new(
        3,
        Position::with_rotation(1, 2, 0.0, 2),
        Position::with_rotation(1, 2, 0.0, 4),
    )
    .nearby_player_count(1);

    assert_eq!(
        event.tick(&[bot], 0),
        vec![
            SchedulerEffect::SetRotation {
                entity_id: 3,
                rotation: 4
            },
            SchedulerEffect::MarkNeedsUpdate { entity_id: 3 }
        ]
    );
}
