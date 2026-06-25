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
mod tests {
    use super::*;
    use crate::Config;

    fn variables() -> GameVariables {
        let config = Config::parse(
            r#"
            [Register]
            user.name.chars=abc123
            user.default.credits=100
            messenger.greeting=Hello

            [Scheduler]
            credits.every.x.secs=2
            credits.every.x.amount=10

            [Bot]
            bot.response.delay=1500

            [Player]
            carry.drink.time=180
            carry.drink.interval=12
            talking.lookat.distance=30
            talking.lookat.reset=6
            afk.room.kick=1800

            [Debug]
            debug.enable=false
            "#,
        )
        .unwrap();

        GameVariables::from_config(&config).unwrap()
    }

    #[test]
    fn emits_credit_save_ip_and_afk_effects_on_matching_tick() {
        let mut scheduler = GameScheduler::new();
        let mut afk_states = vec![RoomAfkState::new(3, 0), RoomAfkState::new(4, 2)];

        let effects = scheduler.tick(
            &variables(),
            [(1, 50), (2, 70)],
            &mut afk_states,
            "localhost",
        );

        assert_eq!(
            effects,
            vec![
                GameTickEffect::AwardCredits {
                    user_id: 1,
                    amount: 10,
                    new_balance: 60,
                },
                GameTickEffect::SavePlayer { user_id: 1 },
                GameTickEffect::AwardCredits {
                    user_id: 2,
                    amount: 10,
                    new_balance: 80,
                },
                GameTickEffect::SavePlayer { user_id: 2 },
                GameTickEffect::ResolveServerIp,
                GameTickEffect::KickAfkUser { user_id: 3 },
            ]
        );
        assert_eq!(afk_states[1].seconds_remaining(), 1);
        assert_eq!(scheduler.tick_rate(), 1);
    }

    #[test]
    fn skips_credit_awards_between_configured_intervals() {
        let mut scheduler = GameScheduler::new();
        let mut afk_states = Vec::new();

        scheduler.tick(&variables(), [(1, 50)], &mut afk_states, "127.0.0.1");
        let effects = scheduler.tick(&variables(), [(1, 50)], &mut afk_states, "127.0.0.1");

        assert!(effects.is_empty());
    }
}
