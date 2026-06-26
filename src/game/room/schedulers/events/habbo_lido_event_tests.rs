use super::*;

#[test]
fn targets_camera_and_moves_pool_queue_players() {
    let mut event = HabboLidoEvent::new();
    let players = [LidoPlayerState::new(
        5,
        "Alice",
        Position::new(3, 4, 0.0),
        false,
    )];
    let queue_tiles = [PoolQueueTile::new(
        Position::new(3, 4, 0.0),
        Position::new(9, 8, 0.0),
    )];

    assert_eq!(
        event.tick(&players, &queue_tiles, 1, 0),
        vec![
            SchedulerEffect::TargetCamera {
                player_id: 5,
                username: "Alice".to_owned()
            },
            SchedulerEffect::SetCamera(1),
            SchedulerEffect::WalkTo {
                entity_id: 5,
                x: 9,
                y: 8
            }
        ]
    );
    assert_eq!(event.following_id(), 5);
    assert_eq!(event.camera_type(), 1);
}
