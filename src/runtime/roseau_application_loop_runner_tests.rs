use super::roseau_application_loop_runner::*;
use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{HostResolver, RoseauBootstrap};
use crate::server::StdTcpSocketBinder;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
struct RecordingExecutor {
    plans: RefCell<Vec<SqlExecutionPlan>>,
    results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
}

impl RecordingExecutor {
    fn push_result(&self, result: SqlExecutionResult) {
        self.results.borrow_mut().push_back(Ok(result));
    }
}

impl SqlExecutor for RecordingExecutor {
    fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        self.plans.borrow_mut().push(plan);
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
    std::env::temp_dir().join(format!("roseau-rs-loop-{name}-{nonce}"))
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
fn runs_bounded_ticks_until_limit_when_server_continues() {
    let (root, bootstrap) = bootstrap("limit");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let tick_executor = MySqlApplicationTickExecutor::new(executor);
    let mut afk_states = Vec::new();
    let runner = RoseauApplicationLoopRunner::new(2);

    let report = runner
        .run(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            0,
            false,
            64,
            &[(4, 25)],
            &mut afk_states,
        )
        .unwrap();

    assert_eq!(report.completed_ticks(), 2);
    assert!(report.should_continue());
    assert_eq!(application.game().scheduler().tick_rate(), 2);

    fs::remove_dir_all(root).unwrap();
}
