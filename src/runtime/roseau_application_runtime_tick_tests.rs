use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::dao::mysql::{
    MySqlApplicationTickExecutor, SqlExecutionPlan, SqlExecutionResult, SqlExecutor,
};
use crate::dao::DaoError;
use crate::game::player::{PlayerDetails, PlayerSession};
use crate::game::{GameTickEffect, RoomAfkState};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{
    HostResolver, RoseauApplicationRuntime, RoseauBootstrap, RoseauGameTickRuntimeActionPlan,
};
use crate::server::{PlayerNetwork, PlayerNetworkEffect, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("roseau-rs-application-tick-{name}-{nonce}"))
}

fn main_config_with_ip(server_ip: &str, server_port: u16, private_server_port: u16) -> String {
    format!(
            "[Server]\nserver.ip={server_ip}\nserver.port={server_port}\nserver.private.port={private_server_port}\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=true\nlog.packets=true\n"
        )
}

fn bootstrap_with_config(
    name: &str,
    server_port: u16,
    private_server_port: u16,
) -> (PathBuf, RoseauBootstrap) {
    bootstrap_with_ip_config(name, "127.0.0.1", server_port, private_server_port)
}

fn bootstrap_with_ip_config(
    name: &str,
    server_ip: &str,
    server_port: u16,
    private_server_port: u16,
) -> (PathBuf, RoseauBootstrap) {
    let root = temp_dir(name);
    fs::create_dir_all(&root).unwrap();
    let main_path = root.join("roseau.properties");
    let hotel_path = root.join("habbohotel.properties");
    fs::write(
        &main_path,
        main_config_with_ip(server_ip, server_port, private_server_port),
    )
    .unwrap();
    fs::write(&hotel_path, DEFAULT_HOTEL_CONFIG).unwrap();

    (
        root,
        RoseauBootstrap::new(main_path.to_owned(), hotel_path.to_owned()),
    )
}

#[derive(Debug, Default)]
struct RecordingExecutor {
    plans: RefCell<Vec<SqlExecutionPlan>>,
    results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
}

impl RecordingExecutor {
    fn push_result(&self, result: SqlExecutionResult) {
        self.results.borrow_mut().push_back(Ok(result));
    }

    fn plans(&self) -> Vec<SqlExecutionPlan> {
        self.plans.borrow().clone()
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

fn player_details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

#[derive(Debug)]
struct RecordingHostResolver {
    result: Result<String, String>,
}

impl HostResolver for RecordingHostResolver {
    fn resolve_host(&self, _host: &str) -> Result<String, String> {
        self.result.clone()
    }
}

#[test]
fn runs_bounded_application_tick_for_game_and_server() {
    let (root, bootstrap) = bootstrap_with_config("application-tick", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 700, None).unwrap();
    let mut afk_states = vec![RoomAfkState::new(9, 0)];

    let outcome = application.run_tick(&binder, 0, true, 64, [(4, 25)], &mut afk_states);

    assert_eq!(
        outcome.game_effects(),
        &[
            GameTickEffect::AwardCredits {
                user_id: 4,
                amount: 10,
                new_balance: 35,
            },
            GameTickEffect::SavePlayer { user_id: 4 },
            GameTickEffect::KickAfkUser { user_id: 9 },
        ]
    );
    assert!(outcome.should_continue());
    assert_eq!(
        outcome.server_outcome().tick().unwrap().accept_error(),
        None
    );
    assert_eq!(application.game().scheduler().tick_rate(), 1);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn runs_tick_persistence_and_runtime_action_planning() {
    let (root, bootstrap) = bootstrap_with_config("application-tick-execution", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 800, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(31, 37120, player_details(9, "alice")));
    let mut afk_states = vec![RoomAfkState::new(9, 0)];
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let tick_executor = MySqlApplicationTickExecutor::new(executor);

    let report = application
        .run_tick_execution_report(
            &tick_executor,
            &binder,
            0,
            true,
            64,
            [(4, 25)],
            &mut afk_states,
        )
        .unwrap();
    let executor = tick_executor.into_executor();

    assert_eq!(
        report.database_report().database_result().results(),
        &[SqlExecutionResult::AffectedRows(1)]
    );
    assert_eq!(
        executor.plans()[0].sql(),
        "UPDATE users SET credits = ? WHERE id = ?"
    );
    assert_eq!(
        report.runtime_plans(),
        &[
            RoseauGameTickRuntimeActionPlan::SyncPlayerCredits {
                user_id: 4,
                credits: 35,
            },
            RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                connection_id: 31
            }),
        ]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_tick_runtime_network_plans_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("application-runtime-plan-apply", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 900, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application
        .startup_runtime_mut()
        .run_loop_step(&binder, 0, true, 64);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(900, 37120, player_details(4, "alice")));
    let mut afk_states = vec![RoomAfkState::new(4, 0)];
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let tick_executor = MySqlApplicationTickExecutor::new(executor);

    let report = application
        .run_tick_execution_report(
            &tick_executor,
            &binder,
            0,
            false,
            64,
            [(4, 25)],
            &mut afk_states,
        )
        .unwrap();
    let unapplied = application.apply_tick_runtime_plans(&report);

    assert!(unapplied.is_empty());
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&900)
            .unwrap()
            .details()
            .credits(),
        35
    );
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()[0]
        .network()
        .is_closed());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn runs_tick_persists_and_applies_runtime_plans_in_one_step() {
    let (root, bootstrap) =
        bootstrap_with_ip_config("application-full-tick-run", "roseau.local", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 950, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application
        .startup_runtime_mut()
        .run_loop_step(&binder, 0, true, 64);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(950, 37120, player_details(9, "alice")));
    let mut afk_states = vec![RoomAfkState::new(9, 0)];
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let tick_executor = MySqlApplicationTickExecutor::new(executor);
    let resolver = RecordingHostResolver {
        result: Ok("10.0.0.25".to_owned()),
    };

    let report = application
        .run_tick_and_apply_runtime_plans(
            &tick_executor,
            &resolver,
            &binder,
            0,
            false,
            64,
            [(4, 25)],
            &mut afk_states,
        )
        .unwrap();
    let executor = tick_executor.into_executor();

    assert!(report.should_continue());
    assert!(report.server_outcome().tick().is_some());
    assert!(report.unapplied_runtime_plans().is_empty());
    assert_eq!(
        report
            .execution_report()
            .database_report()
            .database_result()
            .results(),
        &[SqlExecutionResult::AffectedRows(1)]
    );
    assert_eq!(
        executor.plans()[0].sql(),
        "UPDATE users SET credits = ? WHERE id = ?"
    );
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()[0]
        .network()
        .is_closed());
    assert_eq!(application.resolved_config_ip(), Some("10.0.0.25"));
    assert_eq!(application.advertised_server_ip(), "10.0.0.25");

    fs::remove_dir_all(root).unwrap();
}
