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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use std::net::TcpStream;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use crate::dao::mysql::DatabaseEngine;
    use crate::runtime::roseau_bootstrap::{DEFAULT_HOTEL_CONFIG, DEFAULT_MAIN_CONFIG};
    use crate::runtime::ServerBootstrapPlan;
    use crate::server::{ServerListenEffectExecutor, ServerListenPlan, StdTcpSocketBinder};

    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("roseau-rs-runtime-{name}-{nonce}"))
    }

    #[test]
    fn builds_runtime_from_configs_and_seed() {
        let runtime = RoseauRuntime::with_random_seed(
            Config::parse(DEFAULT_MAIN_CONFIG).unwrap(),
            Config::parse(DEFAULT_HOTEL_CONFIG).unwrap(),
            42,
        )
        .unwrap();

        assert_eq!(
            runtime.main_config().get("Server", "server.ip"),
            Some("127.0.0.1")
        );
        assert_eq!(runtime.game_variables().user_default_credits(), 100);
        assert!(runtime.logger().output_enabled());
    }

    #[test]
    fn load_creates_missing_config_files_and_runtime_state() {
        let root = temp_dir("load");
        let bootstrap = RoseauBootstrap::new(
            root.join("roseau.properties"),
            root.join("habbohotel.properties"),
        );

        let mut runtime = RoseauRuntime::load(&bootstrap).unwrap();

        assert!(bootstrap.main_config_path().is_file());
        assert!(bootstrap.hotel_config_path().is_file());
        assert_eq!(runtime.random_mut().next_i32(1), Some(0));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn builds_tcp_server_runtime_with_configured_logging_flags() {
        let runtime = RoseauRuntime::with_random_seed(
            Config::parse(DEFAULT_MAIN_CONFIG).unwrap(),
            Config::parse(DEFAULT_HOTEL_CONFIG).unwrap(),
            42,
        )
        .unwrap();
        let binder = StdTcpSocketBinder::new();
        let listen_plan = ServerListenPlan::new("127.0.0.1", vec![0]);
        let mut listen_executor = ServerListenEffectExecutor::new();
        let outcome = listen_executor.execute_plan(&listen_plan, &binder);
        let address = binder.local_addresses().unwrap()[0];
        let mut client = TcpStream::connect(address).unwrap();
        let plan = ServerBootstrapPlan::new(
            "127.0.0.1",
            "127.0.0.1",
            address.port(),
            37119,
            "roseau::server::ServerHandler",
            DatabaseEngine::MySql,
            vec![address.port()],
        );
        let mut tcp_runtime = runtime.tcp_server_runtime(&plan, 90).unwrap();
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();

        assert!(outcome.listened());
        tcp_runtime.accept_and_open_one(&binder, 0).unwrap();

        let mut bytes = [0; 8];
        client.read_exact(&mut bytes).unwrap();

        assert_eq!(&bytes, b"#HELLO##");
        assert_eq!(
            tcp_runtime
                .connection(0)
                .unwrap()
                .effect_executor()
                .connection_logs(),
            &["[90] Connection from 127.0.0.1".to_owned()]
        );
    }
}
