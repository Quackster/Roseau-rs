use crate::game::room::schedulers::{RoomEvent, SchedulerEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClubMassivaDiscoEvent {
    event: RoomEvent,
    current_lamp_id: i32,
}

impl ClubMassivaDiscoEvent {
    pub fn new() -> Self {
        Self {
            event: RoomEvent::new(),
            current_lamp_id: -1,
        }
    }

    pub fn tick(
        &mut self,
        preferred_lamp_id: i32,
        disco_id: i32,
        include_floor_b: bool,
    ) -> Vec<SchedulerEffect> {
        let mut effects = Vec::new();

        if self.event.can_tick(10) {
            let lamp_id = self.next_lamp_id(preferred_lamp_id);
            self.current_lamp_id = lamp_id;

            effects.push(SchedulerEffect::ShowProgram(vec![
                "lamp".to_owned(),
                "setlamp".to_owned(),
                lamp_id.to_string(),
            ]));

            for floor in ["df1", "df2", "df3"] {
                effects.push(SchedulerEffect::ShowProgram(vec![
                    floor.to_owned(),
                    "setfloora".to_owned(),
                    disco_id.to_string(),
                ]));
            }

            if include_floor_b {
                for floor in ["df1", "df2", "df3"] {
                    effects.push(SchedulerEffect::ShowProgram(vec![
                        floor.to_owned(),
                        "setfloorb".to_owned(),
                        disco_id.to_string(),
                    ]));
                }
            }
        }

        self.event.increase_ticked();
        effects
    }

    pub fn current_lamp_id(&self) -> i32 {
        self.current_lamp_id
    }

    fn next_lamp_id(&self, preferred_lamp_id: i32) -> i32 {
        let lamp_id = preferred_lamp_id.clamp(1, 5);

        if lamp_id == self.current_lamp_id {
            if lamp_id == 5 {
                1
            } else {
                lamp_id + 1
            }
        } else {
            lamp_id
        }
    }
}

impl Default for ClubMassivaDiscoEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "club_massiva_disco_event_tests.rs"]
mod tests;
