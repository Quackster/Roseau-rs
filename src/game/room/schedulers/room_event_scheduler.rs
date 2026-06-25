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

#[cfg(test)]
mod tests {
    use super::*;

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
}
