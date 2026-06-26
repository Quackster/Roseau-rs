use crate::game::room::entity::RoomUser;
use crate::game::room::model::Position;
use crate::game::room::schedulers::RoomScheduler;

fn bot_user(entity_id: i32) -> RoomUser {
    let mut user = RoomUser::new(entity_id, "Zoe", "figure", "mission", None::<String>);
    user.set_room_id(13);
    user
}

fn start() -> Position {
    Position::new(6, 29, 7.0)
}

#[test]
fn register_mirrors_java_first_player_entry() {
    let mut scheduler =
        RoomScheduler::register("pool_a", vec![bot_user(101)], vec![start()], vec![vec![(6, 29)]]);

    assert_eq!(scheduler.bots().len(), 1);
    assert_eq!(scheduler.bot_start_positions()[0], start());
    assert_eq!(scheduler.bot_patrol_positions()[0], vec![(6, 29)]);
    assert!(scheduler.bot_move_event_mut().is_some());
    assert!(scheduler.lido_event_mut().is_none());
    assert!(scheduler.disco_event_mut().is_none());
}

#[test]
fn register_only_adds_model_events_for_their_models() {
    let mut lido = RoomScheduler::register("pool_b", Vec::new(), Vec::new(), Vec::new());
    assert!(lido.lido_event_mut().is_some());
    assert!(lido.bot_move_event_mut().is_none());
    assert!(lido.disco_event_mut().is_none());

    let mut disco = RoomScheduler::register("bar_b", Vec::new(), Vec::new(), Vec::new());
    assert!(disco.disco_event_mut().is_some());
    assert!(disco.lido_event_mut().is_none());
}
