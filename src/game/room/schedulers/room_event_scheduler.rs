use crate::game::room::schedulers::SchedulerEffect;

pub struct RoomEventScheduler {
    events: Vec<Box<dyn FnMut() -> Vec<SchedulerEffect>>>,
}

impl RoomEventScheduler {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Box<dyn FnMut() -> Vec<SchedulerEffect>>) {
        self.events.push(event);
    }

    pub fn run(&mut self) -> Vec<SchedulerEffect> {
        let mut effects = Vec::new();

        for event in &mut self.events {
            effects.extend(event());
        }

        effects
    }
}

impl Default for RoomEventScheduler {
    fn default() -> Self {
        Self::new()
    }
}
