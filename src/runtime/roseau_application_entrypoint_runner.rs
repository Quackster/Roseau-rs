use crate::dao::mysql::{MySqlApplicationTickExecutor, SqlExecutor, StorageConnector};
use crate::game::RoomAfkState;
use crate::runtime::{
    HostResolver, RoseauApplicationEntrypointError, RoseauApplicationEntrypointReport,
    RoseauApplicationLoopRunner, RoseauApplicationRuntime, RoseauBootstrap,
};
use crate::server::ServerSocketBinder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationEntrypointRunner {
    loop_runner: RoseauApplicationLoopRunner,
}

impl RoseauApplicationEntrypointRunner {
    pub fn new(loop_runner: RoseauApplicationLoopRunner) -> Self {
        Self { loop_runner }
    }

    pub fn loop_runner(&self) -> &RoseauApplicationLoopRunner {
        &self.loop_runner
    }

    pub fn run<B: ServerSocketBinder, C: StorageConnector, E: SqlExecutor, R: HostResolver>(
        &self,
        bootstrap: &RoseauBootstrap,
        binder: &B,
        connector: &C,
        tick_executor: &MySqlApplicationTickExecutor<E>,
        resolver: &R,
        public_room_ids: impl IntoIterator<Item = i32>,
        first_connection_id: i32,
        resolved_config_ip: Option<&str>,
        listener_index: usize,
        accept_connection: bool,
        max_bytes: usize,
        main_server_players: &[(i32, i32)],
        room_afk_states: &mut [RoomAfkState],
    ) -> Result<RoseauApplicationEntrypointReport, RoseauApplicationEntrypointError> {
        let mut prepare_report = RoseauApplicationRuntime::prepare_with_database_connector(
            bootstrap,
            binder,
            connector,
            public_room_ids,
            first_connection_id,
            resolved_config_ip,
        )?;

        if !prepare_report.ready() {
            return Ok(RoseauApplicationEntrypointReport::new(prepare_report, None));
        }

        let loop_report = self.loop_runner.run(
            prepare_report
                .application_runtime_mut()
                .expect("ready prepare report has runtime"),
            tick_executor,
            resolver,
            binder,
            listener_index,
            accept_connection,
            max_bytes,
            main_server_players,
            room_afk_states,
        )?;

        Ok(RoseauApplicationEntrypointReport::new(
            prepare_report,
            Some(loop_report),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult, Storage, StorageConnector};
    use crate::dao::DaoError;
    use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
    use crate::runtime::{HostResolver, RoseauBootstrap};
    use crate::server::StdTcpSocketBinder;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    struct RecordingConnector {
        result: RefCell<Option<Result<(), DaoError>>>,
    }

    impl RecordingConnector {
        fn succeed() -> Self {
            Self {
                result: RefCell::new(Some(Ok(()))),
            }
        }

        fn fail(message: &str) -> Self {
            Self {
                result: RefCell::new(Some(Err(DaoError::new(message)))),
            }
        }
    }

    impl StorageConnector for RecordingConnector {
        fn connect(&self, _storage: &Storage) -> Result<(), DaoError> {
            self.result
                .borrow_mut()
                .take()
                .unwrap_or_else(|| Err(DaoError::new("missing connector result")))
        }
    }

    #[derive(Debug, Default)]
    struct RecordingExecutor {
        results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
    }

    impl RecordingExecutor {
        fn push_result(&self, result: SqlExecutionResult) {
            self.results.borrow_mut().push_back(Ok(result));
        }
    }

    impl SqlExecutor for RecordingExecutor {
        fn execute(&self, _plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
            self.results
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Err(DaoError::new("missing executor result")))
        }
    }

    #[derive(Debug)]
    struct StaticResolver;

    impl HostResolver for StaticResolver {
        fn resolve_host(&self, _host: &str) -> Result<String, String> {
            Ok("127.0.0.1".to_owned())
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("roseau-rs-entrypoint-{name}-{nonce}"))
    }

    fn bootstrap(name: &str) -> (PathBuf, RoseauBootstrap) {
        let root = temp_dir(name);
        fs::create_dir_all(&root).unwrap();
        let main_path = root.join("roseau.properties");
        let hotel_path = root.join("habbohotel.properties");
        fs::write(
            &main_path,
            "[Server]\nserver.ip=127.0.0.1\nserver.port=0\nserver.private.port=0\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=false\nlog.packets=false\n",
        )
        .unwrap();
        fs::write(&hotel_path, DEFAULT_HOTEL_CONFIG).unwrap();

        (
            root,
            RoseauBootstrap::new(main_path.to_owned(), hotel_path.to_owned()),
        )
    }

    #[test]
    fn skips_loop_when_database_connection_fails() {
        let (root, bootstrap) = bootstrap("db-failed");
        let binder = StdTcpSocketBinder::new();
        let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
        let mut afk_states = Vec::new();
        let runner = RoseauApplicationEntrypointRunner::new(RoseauApplicationLoopRunner::new(1));

        let report = runner
            .run(
                &bootstrap,
                &binder,
                &RecordingConnector::fail("database unavailable"),
                &tick_executor,
                &StaticResolver,
                [],
                1,
                None,
                0,
                false,
                64,
                &[],
                &mut afk_states,
            )
            .unwrap();

        assert!(!report.ready());
        assert!(!report.ran_loop());
        assert!(report.loop_report().is_none());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn prepares_and_runs_loop_after_database_connection() {
        let (root, bootstrap) = bootstrap("ready");
        let binder = StdTcpSocketBinder::new();
        let executor = RecordingExecutor::default();
        executor.push_result(SqlExecutionResult::affected_rows(1));
        let tick_executor = MySqlApplicationTickExecutor::new(executor);
        let mut afk_states = Vec::new();
        let runner = RoseauApplicationEntrypointRunner::new(RoseauApplicationLoopRunner::new(1));

        let report = runner
            .run(
                &bootstrap,
                &binder,
                &RecordingConnector::succeed(),
                &tick_executor,
                &StaticResolver,
                [],
                1,
                None,
                0,
                false,
                64,
                &[(4, 25)],
                &mut afk_states,
            )
            .unwrap();

        assert!(report.ready());
        assert!(report.ran_loop());
        assert_eq!(report.loop_report().unwrap().completed_ticks(), 1);

        fs::remove_dir_all(root).unwrap();
    }
}
