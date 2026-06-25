use super::room_event_scheduler::*;

#[test]
fn collects_effects_from_registered_events() {
    let mut scheduler = RoomEventScheduler::new();
    scheduler.add_event(Box::new(|| {
        vec![SchedulerEffect::ShowProgram(vec!["lamp".to_owned()])]
    }));
    scheduler.add_event(Box::new(|| vec![SchedulerEffect::SetCamera(2)]));

    assert_eq!(
        scheduler.run(),
        vec![
            SchedulerEffect::ShowProgram(vec!["lamp".to_owned()]),
            SchedulerEffect::SetCamera(2)
        ]
    );
}
