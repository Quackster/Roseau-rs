use crate::config::Config;
use crate::game::GameVariables;
use crate::logging::Logger;
use crate::runtime::{
    BootstrapError, RandomSource, RoseauBootstrap, RoseauServerFactory, ServerBootstrapPlan,
};
use crate::server::TcpServerRuntime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauRuntime {
    main_config: Config,
    hotel_config: Config,
    game_variables: GameVariables,
    logger: Logger,
    random: RandomSource,
}

impl RoseauRuntime {
    pub fn load(bootstrap: &RoseauBootstrap) -> Result<Self, BootstrapError> {
        bootstrap.ensure_config_files()?;
        let main_config = bootstrap.load_main_config()?;
        let hotel_config = bootstrap.load_hotel_config()?;
        let game_variables = GameVariables::from_config(&hotel_config)?;
        let logger = Logger::from_config(&main_config, "log");
        let random = RandomSource::from_clock();

        Ok(Self {
            main_config,
            hotel_config,
            game_variables,
            logger,
            random,
        })
    }

    pub fn with_random_seed(
        main_config: Config,
        hotel_config: Config,
        seed: u64,
    ) -> Result<Self, BootstrapError> {
        let game_variables = GameVariables::from_config(&hotel_config)?;
        let logger = Logger::from_config(&main_config, "log");
        let random = RandomSource::seeded(seed);

        Ok(Self {
            main_config,
            hotel_config,
            game_variables,
            logger,
            random,
        })
    }

    pub fn main_config(&self) -> &Config {
        &self.main_config
    }

    pub fn hotel_config(&self) -> &Config {
        &self.hotel_config
    }

    pub fn game_variables(&self) -> &GameVariables {
        &self.game_variables
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn random(&self) -> &RandomSource {
        &self.random
    }

    pub fn random_mut(&mut self) -> &mut RandomSource {
        &mut self.random
    }

    pub fn tcp_server_runtime(
        &self,
        plan: &ServerBootstrapPlan,
        first_connection_id: i32,
    ) -> Result<TcpServerRuntime, BootstrapError> {
        let log_connections = self
            .main_config
            .get_bool("Logging", "log.connections")
            .unwrap_or(false);
        let log_packets = self
            .main_config
            .get_bool("Logging", "log.packets")
            .unwrap_or(false);

        RoseauServerFactory::new().construct_tcp_runtime(
            plan,
            log_connections,
            log_packets,
            first_connection_id,
        )
    }
}
