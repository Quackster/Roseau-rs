use crate::game::{GameTickEffect, GameVariables, RoomAfkState};
use crate::util::has_valid_ip_address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameScheduler {
    tick_rate: u64,
}

impl GameScheduler {
    pub fn new() -> Self {
        Self { tick_rate: 0 }
    }

    pub fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    pub fn tick(
        &mut self,
        variables: &GameVariables,
        main_server_players: impl IntoIterator<Item = (i32, i32)>,
        room_afk_states: &mut [RoomAfkState],
        raw_config_ip: &str,
    ) -> Vec<GameTickEffect> {
        let mut effects = Vec::new();

        if variables.credits_every_secs() != 0
            && self.tick_rate % variables.credits_every_secs() == 0
        {
            for (user_id, credits) in main_server_players {
                effects.push(GameTickEffect::AwardCredits {
                    user_id,
                    amount: variables.credits_every_amount(),
                    new_balance: credits + variables.credits_every_amount(),
                });
                effects.push(GameTickEffect::SavePlayer { user_id });
            }
        }

        if self.tick_rate.is_multiple_of(300) && !has_valid_ip_address(raw_config_ip) {
            effects.push(GameTickEffect::ResolveServerIp);
        }

        for state in room_afk_states {
            if state.seconds_remaining() > 0 {
                state.tick();
            } else if state.should_kick() {
                effects.push(GameTickEffect::KickAfkUser {
                    user_id: state.user_id(),
                });
            }
        }

        self.tick_rate += 1;
        effects
    }
}

impl Default for GameScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "game_scheduler_tests.rs"]
mod tests;
