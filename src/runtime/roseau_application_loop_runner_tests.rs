use super::*;
use crate::dao::in_memory::{InMemoryDao, InMemoryNavigatorDao, InMemoryPlayerDao};
use crate::dao::mysql::{SqlExecutionPlan, SqlExecutionResult};
use crate::dao::{CreatePlayer, PlayerDao};
use crate::game::item::{Item, ItemDefinition};
use crate::game::player::{PlayerDetails, PlayerSession};
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomConnection, RoomData, RoomSummary};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{HostResolver, RoseauBootstrap};
use crate::server::StdTcpSocketBinder;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::path::PathBuf;
use std::time::Duration;
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

fn client_frame(content: &str) -> Vec<u8> {
    format!("{:04}{content}", content.len()).into_bytes()
}

fn read_available_text(client: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut buffer = [0; 4096];

    loop {
        match client.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => bytes.extend_from_slice(&buffer[..read]),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                break;
            }
            Err(error) => panic!("failed reading client response: {error}"),
        }
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

fn player_dao_with_alex() -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&CreatePlayer::new(
        "Alex",
        "123",
        "alex@example.test",
        "mission",
        "figure",
        1289,
        "Male",
        "1990-01-01",
    ))
    .unwrap();
    dao
}

fn player_details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
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
    let runner = RoseauApplicationLoopRunner::bounded(2);

    let report = runner
        .run(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[(4, 25)],
            &mut afk_states,
        )
        .unwrap();

    assert_eq!(report.completed_ticks(), 2);
    assert!(report.should_continue());
    assert_eq!(application.game().scheduler().tick_rate(), 2);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn run_with_incoming_daos_sends_login_burst_packets_to_tcp_client() {
    let (root, bootstrap) = bootstrap("incoming-login-burst");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let tick_executor = MySqlApplicationTickExecutor::new(executor);
    let dao = InMemoryDao::new(player_dao_with_alex());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut afk_states = Vec::new();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("MESSENGERINIT"));
    burst.extend(client_frame("GETCREDITS"));
    burst.extend(client_frame("INFORETRIEVE Alex"));
    client.write_all(&burst).unwrap();

    RoseauApplicationLoopRunner::bounded(2)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut afk_states,
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &navigator_dao,
                dao.messenger(),
            ),
        )
        .unwrap();

    let response = read_available_text(&mut client);
    assert!(response.contains("#HELLO##"), "{response}");
    assert!(response.contains("#WALLETBALANCE\r1289##"), "{response}");
    assert!(response.contains("#MESSENGERSREADY##"), "{response}");
    assert!(response.contains("#USEROBJECT"), "{response}");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn run_with_incoming_daos_advances_room_walk_ticks_without_new_move_packets() {
    let (root, bootstrap) = bootstrap("room-walk-tick");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("walk", "00", 0, 0, 0, 0, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            1,
            false,
            RoomType::Public,
            0,
            "",
            "Walk Room",
            0,
            "",
            25,
            "",
            "walk",
            "",
            "0",
            "0",
            false,
            false,
        )));

    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let _hello = read_available_text(&mut client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(1);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    let mut session = PlayerSession::new(1, 1, player_details(7, "Alex"));
    session.set_room_user(room_user);
    application.game_mut().player_manager_mut().insert(session);

    RoseauApplicationLoopRunner::bounded(1)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    let response = read_available_text(&mut client);
    assert_eq!(response, "#STATUS \rAlex 0,0,0,2,2/mv 1,0,0/##");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn room_walk_tick_writes_to_room_socket_not_same_user_main_socket() {
    let (root, bootstrap) = bootstrap("room-walk-room-socket");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("walk", "00", 0, 0, 0, 0, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            1,
            false,
            RoomType::Public,
            0,
            "",
            "Walk Room",
            0,
            "",
            25,
            "",
            "walk",
            "",
            "0",
            "0",
            false,
            false,
        )));

    let address = binder.local_addresses().unwrap()[0];
    let mut main_client = TcpStream::connect(address).unwrap();
    let mut room_client = TcpStream::connect(address).unwrap();
    main_client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    room_client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application.startup_runtime_mut().run_loop_step(&binder);
    let _main_hello = read_available_text(&mut main_client);
    let _room_hello = read_available_text(&mut room_client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(1);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(1, 0, player_details(7, "Alex")));
    let mut room_session = PlayerSession::new(2, 1, player_details(7, "Alex"));
    room_session.set_room_user(room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(room_session);

    RoseauApplicationLoopRunner::bounded(1)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    assert_eq!(read_available_text(&mut main_client), "");
    assert_eq!(
        read_available_text(&mut room_client),
        "#STATUS \rAlex 0,0,0,2,2/mv 1,0,0/##"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn room_walk_tick_writes_private_room_status_to_private_room_socket() {
    let (root, bootstrap) = bootstrap("private-room-walk-room-socket");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("walk", "00", 0, 0, 0, 0, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            37,
            false,
            RoomType::Private,
            7,
            "Alex",
            "Alex Den",
            0,
            "",
            25,
            "",
            "walk",
            "",
            "0",
            "0",
            false,
            false,
        )));

    let address = binder.local_addresses().unwrap()[0];
    let mut private_client = TcpStream::connect(address).unwrap();
    private_client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let _hello = read_available_text(&mut private_client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(37);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    let mut private_session = PlayerSession::new(1, 0, player_details(7, "Alex"));
    private_session.set_room_user(room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(private_session);

    RoseauApplicationLoopRunner::bounded(1)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    assert_eq!(
        read_available_text(&mut private_client),
        "#STATUS \rAlex 0,0,0,2,2/mv 1,0,0/##"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn room_walk_stop_on_public_connection_loads_target_lido_room() {
    let (root, bootstrap) = bootstrap("public-room-connection-transition");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("pool_a", "00", 0, 0, 0, 0, false, false).unwrap());
    dao.room()
        .insert_model(RoomModel::new("pool_b", "00", 0, 0, 0, 0, false, false).unwrap());
    dao.room().insert_connections(
        1,
        vec![RoomConnection::with_source(
            1,
            2,
            Position::new(1, 0, 0.0),
            Position::with_rotation(0, 0, 0.0, 2),
        )],
    );
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            1,
            false,
            RoomType::Public,
            0,
            "",
            "Habbo Lido Changing",
            0,
            "",
            25,
            "",
            "pool_a",
            "lido",
            "0",
            "0",
            false,
            false,
        )));
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            2,
            false,
            RoomType::Public,
            0,
            "",
            "Habbo Lido Diving",
            0,
            "",
            25,
            "",
            "pool_b",
            "lido",
            "0",
            "0",
            false,
            false,
        )));

    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let _hello = read_available_text(&mut client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(1);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    let mut session = PlayerSession::new(1, 1, player_details(7, "Alex"));
    session.set_room_user(room_user);
    application.game_mut().player_manager_mut().insert(session);

    RoseauApplicationLoopRunner::bounded(2)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    let response = read_available_text(&mut client);
    assert!(response.contains("#HEIGHTMAP\r00##"), "{response}");
    assert!(
        response.contains("# OBJECTS WORLD 0 pool_b##"),
        "{response}"
    );
    assert!(
        response.contains("#USERS\r  Alex figure 0 0 0 mission##"),
        "{response}"
    );
    let session = application
        .game()
        .player_manager()
        .players()
        .get(&1)
        .unwrap();
    let room_user = session.room_user().unwrap();
    assert_eq!(session.server_port(), 2);
    assert_eq!(room_user.room_id(), 2);
    assert_eq!(room_user.position(), Position::with_rotation(0, 0, 0.0, 2));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn room_walk_stop_triggers_chair_sit_status() {
    let (root, bootstrap) = bootstrap("room-walk-chair-sit");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("walk", "00", 0, 0, 0, 0, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            1,
            false,
            RoomType::Public,
            0,
            "",
            "Chair Room",
            0,
            "",
            25,
            "",
            "walk",
            "",
            "0",
            "0",
            false,
            false,
        )));
    let chair = ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "");
    dao.item().insert_definition(chair.clone());
    dao.item()
        .insert_item(Item::new(50, 1, 7, "1", 0, 0.0, 2, chair, "walk", None).unwrap());

    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let _hello = read_available_text(&mut client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(1);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    let mut session = PlayerSession::new(1, 1, player_details(7, "Alex"));
    session.set_room_user(room_user);
    application.game_mut().player_manager_mut().insert(session);

    RoseauApplicationLoopRunner::bounded(2)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    let response = read_available_text(&mut client);
    assert!(response.contains("/sit 1/"), "{response}");
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&1)
            .unwrap()
            .room_user()
            .unwrap()
            .current_item_id(),
        Some(50)
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn room_walk_stop_triggers_bed_lay_status() {
    let (root, bootstrap) = bootstrap("room-walk-bed-lay");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_model(RoomModel::new("walk", "000", 0, 0, 0, 0, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(RoomData::new(
            1,
            false,
            RoomType::Public,
            0,
            "",
            "Bed Room",
            0,
            "",
            25,
            "",
            "walk",
            "",
            "0",
            "0",
            false,
            false,
        )));
    let bed = ItemDefinition::new(6, "bed", "red", 2, 1, 0.25, "SFB", "Bed", "", "");
    dao.item().insert_definition(bed.clone());
    dao.item()
        .insert_item(Item::new(60, 1, 7, "1", 0, 0.0, 0, bed, "walk", None).unwrap());

    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_millis(250)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let _hello = read_available_text(&mut client);

    let mut room_user = RoomUser::new(7, "Alex", "figure", "mission", None::<String>);
    room_user.set_room_id(1);
    room_user.set_position(Position::new(0, 0, 0.0));
    assert!(room_user.walk_to(1, 0, VecDeque::from([Position::new(1, 0, 0.0)])));
    let mut session = PlayerSession::new(1, 1, player_details(7, "Alex"));
    session.set_room_user(room_user);
    application.game_mut().player_manager_mut().insert(session);

    RoseauApplicationLoopRunner::bounded(2)
        .run_with_incoming_daos(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut [],
            IncomingDaoSet::new(
                dao.player(),
                dao.room(),
                dao.catalogue(),
                dao.inventory(),
                dao.item(),
                &InMemoryNavigatorDao::new([]),
                dao.messenger(),
            ),
        )
        .unwrap();

    let response = read_available_text(&mut client);
    assert!(response.contains("/lay 1.75 null/"), "{response}");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn closed_tcp_connections_remove_game_player_sessions() {
    let (root, bootstrap) = bootstrap("disconnect-removes-player-session");
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let tick_executor = MySqlApplicationTickExecutor::new(RecordingExecutor::default());
    let mut afk_states = Vec::new();
    let address = binder.local_addresses().unwrap()[0];
    let client = TcpStream::connect(address).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(1, 0, player_details(7, "Alex")));

    client.shutdown(Shutdown::Write).unwrap();
    RoseauApplicationLoopRunner::bounded(1)
        .run(
            &mut application,
            &tick_executor,
            &StaticResolver,
            &binder,
            &[],
            &mut afk_states,
        )
        .unwrap();

    assert!(application.game().player_manager().players().is_empty());
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}
