use crate::game::room::schedulers::{RoomEvent, RoomUserTickState, SchedulerEffect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStatusEvent {
    event: RoomEvent,
}

impl UserStatusEvent {
    pub fn new() -> Self {
        Self {
            event: RoomEvent::new(),
        }
    }

    pub fn tick(&mut self, users: &[RoomUserTickState]) -> Vec<SchedulerEffect> {
        let mut effects = Vec::new();

        if self.event.can_tick(2) {
            for user in users {
                self.tick_look_reset(user, &mut effects);
                self.tick_statuses(user, &mut effects);
            }
        }

        self.event.increase_ticked();
        effects
    }

    fn tick_look_reset(&self, user: &RoomUserTickState, effects: &mut Vec<SchedulerEffect>) {
        match user.look_reset_time_value() {
            time if time > 0 => effects.push(SchedulerEffect::SetLookResetTime {
                entity_id: user.entity_id(),
                ticks: time - 1,
            }),
            0 => {
                effects.push(SchedulerEffect::SetHeadRotation {
                    entity_id: user.entity_id(),
                    rotation: user.body_rotation(),
                });
                effects.push(SchedulerEffect::SetLookResetTime {
                    entity_id: user.entity_id(),
                    ticks: -1,
                });
                effects.push(SchedulerEffect::MarkNeedsUpdate {
                    entity_id: user.entity_id(),
                });
            }
            _ => {}
        }
    }

    fn tick_statuses(&self, user: &RoomUserTickState, effects: &mut Vec<SchedulerEffect>) {
        for status in user.statuses().values() {
            let mut status_was_replaced = false;

            if status.key() == "carryd" {
                if user.is_walking() || user.contains_status("dance") || user.contains_status("lay")
                {
                    return;
                }

                if user.time_until_next_drink_value() > 0 {
                    effects.push(SchedulerEffect::SetTimeUntilNextDrink {
                        entity_id: user.entity_id(),
                        ticks: user.time_until_next_drink_value() - 1,
                    });
                } else {
                    effects.push(SchedulerEffect::RemoveStatus {
                        entity_id: user.entity_id(),
                        key: "carryd".to_owned(),
                    });
                    effects.push(SchedulerEffect::SetStatus {
                        entity_id: user.entity_id(),
                        key: "drink".to_owned(),
                        value: String::new(),
                        infinite: false,
                        duration: -1,
                    });
                    effects.push(SchedulerEffect::RemoveStatus {
                        entity_id: user.entity_id(),
                        key: "drink".to_owned(),
                    });
                    effects.push(SchedulerEffect::SetStatus {
                        entity_id: user.entity_id(),
                        key: "carryd".to_owned(),
                        value: status.value().to_owned(),
                        infinite: false,
                        duration: status.duration(),
                    });
                    status_was_replaced = true;
                }
            }

            if !status.is_infinite() && !status_was_replaced {
                let next_duration = status.duration() - 1;
                effects.push(SchedulerEffect::TickStatus {
                    entity_id: user.entity_id(),
                    key: status.key().to_owned(),
                });

                if next_duration == 0 {
                    effects.push(SchedulerEffect::RemoveStatus {
                        entity_id: user.entity_id(),
                        key: status.key().to_owned(),
                    });

                    if status.key() == "carryd" {
                        effects.push(SchedulerEffect::SetTimeUntilNextDrink {
                            entity_id: user.entity_id(),
                            ticks: -1,
                        });
                    }

                    effects.push(SchedulerEffect::MarkNeedsUpdate {
                        entity_id: user.entity_id(),
                    });
                }
            }
        }
    }
}

impl Default for UserStatusEvent {
    fn default() -> Self {
        Self::new()
    }
}
