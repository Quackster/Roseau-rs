use super::club_massiva_disco_event::*;

#[test]
fn emits_lamp_and_floor_programs_on_tick_interval() {
    let mut event = ClubMassivaDiscoEvent::new();

    let effects = event.tick(2, 14, true);

    assert_eq!(event.current_lamp_id(), 2);
    assert_eq!(effects.len(), 7);
    assert_eq!(
        effects[0],
        SchedulerEffect::ShowProgram(vec![
            "lamp".to_owned(),
            "setlamp".to_owned(),
            "2".to_owned()
        ])
    );
    assert_eq!(
        effects[6],
        SchedulerEffect::ShowProgram(vec![
            "df3".to_owned(),
            "setfloorb".to_owned(),
            "14".to_owned()
        ])
    );
}
