use crate::{Config, ConfigError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameVariables {
    credits_every_secs: u64,
    credits_every_amount: i32,
    username_chars: String,
    bot_response_delay: u64,
    carry_drink_interval: u64,
    carry_drink_time: u64,
    talk_look_at_reset: u64,
    talk_distance: i32,
    user_default_credits: i32,
    teleporter_delay: u64,
    max_items_per_page: usize,
    messenger_greeting: String,
    debug_enabled: bool,
    afk_room_kick: u64,
}

impl GameVariables {
    pub const DEFAULT_TELEPORTER_DELAY: u64 = 800;
    pub const DEFAULT_MAX_ITEMS_PER_PAGE: usize = 9;

    pub fn from_config(config: &Config) -> Result<Self, ConfigError> {
        Ok(Self {
            credits_every_secs: config.parse_value("Scheduler", "credits.every.x.secs")?,
            credits_every_amount: config.parse_value("Scheduler", "credits.every.x.amount")?,
            username_chars: config.required("Register", "user.name.chars")?.to_owned(),
            messenger_greeting: config
                .required("Register", "messenger.greeting")?
                .to_owned(),
            bot_response_delay: config.parse_value("Bot", "bot.response.delay")?,
            carry_drink_interval: config.parse_value("Player", "carry.drink.interval")?,
            afk_room_kick: config.parse_value("Player", "afk.room.kick")?,
            carry_drink_time: config.parse_value("Player", "carry.drink.time")?,
            talk_look_at_reset: config.parse_value("Player", "talking.lookat.reset")?,
            talk_distance: config.parse_value("Player", "talking.lookat.distance")?,
            user_default_credits: config.parse_value("Register", "user.default.credits")?,
            debug_enabled: config.get_bool("Debug", "debug.enable")?,
            teleporter_delay: Self::DEFAULT_TELEPORTER_DELAY,
            max_items_per_page: Self::DEFAULT_MAX_ITEMS_PER_PAGE,
        })
    }

    pub fn credits_every_secs(&self) -> u64 {
        self.credits_every_secs
    }

    pub fn credits_every_amount(&self) -> i32 {
        self.credits_every_amount
    }

    pub fn username_chars(&self) -> &str {
        &self.username_chars
    }

    pub fn bot_response_delay(&self) -> u64 {
        self.bot_response_delay
    }

    pub fn carry_drink_interval(&self) -> u64 {
        self.carry_drink_interval
    }

    pub fn carry_drink_time(&self) -> u64 {
        self.carry_drink_time
    }

    pub fn talk_look_at_reset(&self) -> u64 {
        self.talk_look_at_reset
    }

    pub fn talk_distance(&self) -> i32 {
        self.talk_distance
    }

    pub fn user_default_credits(&self) -> i32 {
        self.user_default_credits
    }

    pub fn teleporter_delay(&self) -> u64 {
        self.teleporter_delay
    }

    pub fn max_items_per_page(&self) -> usize {
        self.max_items_per_page
    }

    pub fn messenger_greeting(&self) -> &str {
        &self.messenger_greeting
    }

    pub fn debug_enabled(&self) -> bool {
        self.debug_enabled
    }

    pub fn afk_room_kick(&self) -> u64 {
        self.afk_room_kick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        Config::parse(
            r#"
            [Register]
            user.name.chars=abc123
            user.default.credits=100
            messenger.greeting=Hello

            [Scheduler]
            credits.every.x.secs=600
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
            debug.enable=true
            "#,
        )
        .unwrap()
    }

    #[test]
    fn loads_java_game_variables_from_config() {
        let variables = GameVariables::from_config(&config()).unwrap();

        assert_eq!(variables.credits_every_secs(), 600);
        assert_eq!(variables.credits_every_amount(), 10);
        assert_eq!(variables.username_chars(), "abc123");
        assert_eq!(variables.messenger_greeting(), "Hello");
        assert_eq!(variables.bot_response_delay(), 1500);
        assert_eq!(variables.carry_drink_interval(), 12);
        assert_eq!(variables.carry_drink_time(), 180);
        assert_eq!(variables.talk_look_at_reset(), 6);
        assert_eq!(variables.talk_distance(), 30);
        assert_eq!(variables.user_default_credits(), 100);
        assert_eq!(variables.teleporter_delay(), 800);
        assert_eq!(variables.max_items_per_page(), 9);
        assert!(variables.debug_enabled());
        assert_eq!(variables.afk_room_kick(), 1800);
    }
}
