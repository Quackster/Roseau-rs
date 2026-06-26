use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::dao::in_memory::{
    InMemoryDao, InMemoryMessengerDao, InMemoryNavigatorDao, InMemoryPlayerDao, InMemoryRoomDao,
};
use crate::dao::mysql::{
    MySqlPlayerPasswordActionExecutionReport, MySqlPlayerPasswordActionReport,
    SqlExecutionBatchResult,
};
use crate::dao::{
    CreatePlayer, InventoryDao, ItemDao, MessengerDao, PlayerDao, PublicRoomDescriptor, RoomDao,
};
use crate::game::commands::CommandEffect;
use crate::game::item::{Item, ItemDefinition};
use crate::game::player::{
    Permission, PlayerDetails, PlayerLoginOutcome, PlayerPasswordActionOutcome, PlayerSession,
};
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::settings::{RoomState, RoomType};
use crate::game::room::{RoomData, RoomSummary};
use crate::messages::outgoing::{
    ActiveObjects, AddWallItem as AddWallItemPacket, FlatProperty, HeightMap, Items, ObjectsWorld,
    RoomReady, Status, UserEntry, Users, YouAreOwner,
};
use crate::messages::OutgoingMessage;
use crate::messages::{IncomingCommand, IncomingExecutionEffect, PendingIncomingCommandBatch};
use crate::runtime::roseau_bootstrap::DEFAULT_HOTEL_CONFIG;
use crate::runtime::{RoseauApplicationRuntime, RoseauBootstrap};
use crate::server::{PlayerNetworkEffect, StdTcpSocketBinder};

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "roseau-rs-application-password-network-{name}-{nonce}"
    ))
}

fn main_config(server_port: u16, private_server_port: u16) -> String {
    format!(
            "[Server]\nserver.ip=127.0.0.1\nserver.port={server_port}\nserver.private.port={private_server_port}\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=true\nlog.packets=true\n"
        )
}

fn bootstrap_with_config(
    name: &str,
    server_port: u16,
    private_server_port: u16,
) -> (PathBuf, RoseauBootstrap) {
    let root = temp_dir(name);
    fs::create_dir_all(&root).unwrap();
    let main_path = root.join("roseau.properties");
    let hotel_path = root.join("habbohotel.properties");
    fs::write(&main_path, main_config(server_port, private_server_port)).unwrap();
    fs::write(&hotel_path, DEFAULT_HOTEL_CONFIG).unwrap();

    (
        root,
        RoseauBootstrap::new(main_path.to_owned(), hotel_path.to_owned()),
    )
}

fn free_adjacent_port_pair() -> u16 {
    for _ in 0..100 {
        let first = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = first.local_addr().unwrap().port();
        let Some(next_port) = port.checked_add(1) else {
            continue;
        };
        if let Ok(second) = TcpListener::bind(("127.0.0.1", next_port)) {
            drop(second);
            drop(first);
            return port;
        }
    }

    panic!("failed to find adjacent free ports");
}

fn player_details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn public_room(id: i32, name: &str, class_name: &str) -> RoomSummary {
    let mut room = RoomSummary::new(RoomData::new(
        id,
        false,
        RoomType::Public,
        -1,
        "",
        name,
        0,
        "",
        25,
        "description",
        "pool_b",
        class_name,
        "wall",
        "floor",
        false,
        true,
    ));
    room.set_order_id(1);
    room
}

fn private_room(id: i32, owner_id: i32, owner_name: &str, name: &str) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        owner_id,
        owner_name,
        name,
        0,
        "",
        25,
        "description",
        "model_a",
        "default",
        "wall",
        "floor",
        false,
        true,
    )
}

fn password_room(id: i32, owner_id: i32, owner_name: &str, name: &str, password: &str) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        owner_id,
        owner_name,
        name,
        2,
        password,
        25,
        "description",
        "model_a",
        "default",
        "wall",
        "floor",
        false,
        true,
    )
}

fn item_definition(id: i32, sprite: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "", 1, 1, 0.0, "SIF", "name", "desc", "")
}

fn floor_item(id: i32, room_id: i32, owner_id: i32, definition: ItemDefinition) -> Item {
    Item::new(id, room_id, owner_id, "1", 1, 0.0, 0, definition, "", None).unwrap()
}

fn doorbell_room(id: i32, owner_id: i32, owner_name: &str, name: &str) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        owner_id,
        owner_name,
        name,
        RoomState::Doorbell.state_code(),
        "",
        25,
        "description",
        "model_a",
        "default",
        "wall",
        "floor",
        false,
        true,
    )
}

fn client_frame(content: &str) -> Vec<u8> {
    format!("{:04}{content}", content.len()).into_bytes()
}

fn connect_client(
    application: &mut RoseauApplicationRuntime,
    binder: &StdTcpSocketBinder,
) -> TcpStream {
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    client
}

fn connect_client_at(
    application: &mut RoseauApplicationRuntime,
    binder: &StdTcpSocketBinder,
    listener_index: usize,
) -> TcpStream {
    let address = binder.local_addresses().unwrap()[listener_index];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    client
}

fn read_available_text(client: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut buffer = [0; 256];

    loop {
        match client.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => bytes.extend_from_slice(&buffer[..count]),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                break;
            }
            Err(error) => panic!("failed to read available bytes: {error}"),
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn player_dao_with_alice() -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&CreatePlayer::new(
        "alice",
        "secret",
        "alice@example.test",
        "mission",
        "figure",
        55,
        "F",
        "1990-01-01",
    ))
    .unwrap();
    dao
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

fn player_dao_with_alice_and_bob() -> InMemoryPlayerDao {
    let dao = player_dao_with_alice();
    dao.create_player(&CreatePlayer::new(
        "bob",
        "secret",
        "bob@example.test",
        "mission",
        "figure",
        55,
        "M",
        "1990-01-01",
    ))
    .unwrap();
    dao
}

fn player_dao_with_alice_bob_and_carol() -> InMemoryPlayerDao {
    let dao = player_dao_with_alice_and_bob();
    dao.create_player(&CreatePlayer::new(
        "carol",
        "secret",
        "carol@example.test",
        "mission",
        "figure",
        55,
        "F",
        "1990-01-01",
    ))
    .unwrap();
    dao
}

#[test]
fn applies_password_action_packets_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("password-runtime-packet", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 910, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    let report = MySqlPlayerPasswordActionExecutionReport::new(
        MySqlPlayerPasswordActionReport::from_outcomes(
            [PlayerPasswordActionOutcome::Login(
                PlayerLoginOutcome::failed(),
            )],
            910,
            1234,
        ),
        SqlExecutionBatchResult::new([]),
    );

    let unapplied = application.apply_password_action_runtime_plans(&report);
    let expected = b"#ERROR Login incorrect##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_login_updates_context_so_getcredits_sends_wallet_balance() {
    let (root, bootstrap) = bootstrap_with_config("incoming-login-credits", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 940, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);

    client
        .write_all(&client_frame("LOGIN alice secret"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    client.write_all(&client_frame("GETCREDITS")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#WALLETBALANCE\r55##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);
    assert!(application
        .game()
        .player_manager()
        .players()
        .contains_key(&940));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_login_matches_seed_user_case_insensitively() {
    let (root, bootstrap) = bootstrap_with_config("incoming-login-alex", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 945, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);

    client.write_all(&client_frame("LOGIN alex 123")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    client.write_all(&client_frame("GETCREDITS")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#WALLETBALANCE\r1289##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_login_messenger_init_and_info_retrieve_in_one_read_send_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-login-burst", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 946, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("MESSENGERINIT"));
    burst.extend(client_frame("INFORETRIEVE Alex"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected_prefix = b"#MYPERSISTENTMSG\r###MESSENGERREADY###USEROBJECT\rname=Alex";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_set_strip_item_data_deletes_last_post_it_and_refreshes_inventory() {
    let (root, bootstrap) = bootstrap_with_config("incoming-set-strip-item-data", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 945, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let chair = ItemDefinition::new(91, "chair", "red", 1, 2, 1.0, "SF", "Chair", "", "");
    let note = ItemDefinition::new(92, "note", "yellow", 1, 1, 0.0, "IJ", "Post-it", "", "");
    dao.item().insert_definition(chair.clone());
    dao.item().insert_definition(note.clone());
    dao.item().insert_item(
        Item::new(
            901,
            0,
            1,
            "1",
            0,
            0.0,
            0,
            chair,
            "",
            Some("blue".to_owned()),
        )
        .unwrap(),
    );
    dao.item()
        .insert_item(Item::new(902, 0, 1, "1", 0, 0.0, 0, note, "", Some("1".to_owned())).unwrap());
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN alice secret"));
    burst.extend(client_frame("SETSTRIPITEMDATA ignored\r902"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
        )
        .unwrap();

    let expected = b"#STRIPINFO\rroseau;901;0;S;0;chair;Chair;blue;1;2;red/##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);
    assert!(dao.item().item(902).unwrap().is_none());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_login_and_getcredits_in_one_read_sends_wallet_balance() {
    let (root, bootstrap) = bootstrap_with_config("incoming-login-credits-burst", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 947, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("GETCREDITS"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected_prefix = b"#WALLETBALANCE\r1289##";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_search_busy_flats_sends_busy_flat_results() {
    let (root, bootstrap) = bootstrap_with_config("incoming-search-busy-flats", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 948, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(private_room(5, 1, "Alex", "Busy Room"));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);

    client
        .write_all(&client_frame("SEARCHBUSYFLATS /0"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected_prefix = b"#BUSY_FLAT_RESULTS 1\r5/Busy Room/Alex";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_empty_init_unit_listener_and_busy_flat_search_send_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-empty-init-and-busy", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 960, Some("127.0.0.1")).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(public_room(1, "Habbo Lido", "lido"));
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("INITUNITLISTENER "));
    burst.extend(client_frame("SEARCHBUSYFLATS "));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    let expected =
        b"#ALLUNITS\rHabbo Lido,0,25,127.0.0.1/127.0.0.1,1,Habbo Lido\tlido,0,25,pool_b###BUSY_FLAT_RESULTS 1##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_find_user_before_login_sends_member_info() {
    let (root, bootstrap) = bootstrap_with_config("incoming-find-user-before-login", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 949, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);

    client.write_all(&client_frame("FINDUSER Alex")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected_prefix = b"#MEMBERINFO \rAlex\r";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_get_flat_info_sends_flat_info_packet() {
    let (root, bootstrap) = bootstrap_with_config("incoming-get-flat-info", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 953, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(private_room(42, 1, "Alex", "Info Room"));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);

    client.write_all(&client_frame("GETFLATINFO /42")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#SETFLATINFO\r/42/##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_create_flat_sends_flat_created_packet() {
    let (root, bootstrap) = bootstrap_with_config("incoming-create-flat", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 954, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame(
        "CREATEFLAT /first floor/Alex Den/model_a/open/1",
    ));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#FLATCREATED\r1 Alex Den##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_create_flat_short_name_sends_java_alert_packet() {
    let binder = StdTcpSocketBinder::new();
    let public = public_room(5, "Habbo Lido", "lido");
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config(
        "incoming-create-flat-short-name",
        server_port,
        server_port + 1,
    );
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(5, "Habbo Lido")],
        955,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let player_dao = player_dao_with_alex();
    let alex = player_dao.login("Alex", "123").unwrap().unwrap().details;
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut public_client = connect_client_at(&mut application, &binder, 2);
    let base_port = i32::from(server_port);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(955, base_port, alex.clone()));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(956, base_port + 5, alex));
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("CREATEFLAT /first floor/ab/model_a/open/1"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#SYSTEMBROADCAST\rThe room name needs to be at least 3 characters long##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();
    let mut byte = [0; 1];

    assert_eq!(bytes, expected);
    assert_eq!(public_client.read(&mut byte).unwrap(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_create_flat_invalid_model_closes_user_connection_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-create-flat-invalid-model", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 956, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("CREATEFLAT /first floor/Alex Den/hax/open/1"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    let mut byte = [0; 1];
    assert_eq!(client.read(&mut byte).unwrap(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_create_flat_closes_same_user_public_room_connection_like_java() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) =
        bootstrap_with_config("incoming-create-flat-close-public", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(5, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(5, "Habbo Lido")],
        957,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let player_dao = player_dao_with_alex();
    let alex = player_dao.login("Alex", "123").unwrap().unwrap().details;
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut main_client = connect_client(&mut application, &binder);
    let mut public_client = connect_client_at(&mut application, &binder, 2);
    let base_port = i32::from(server_port);

    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(957, base_port, alex.clone()));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(958, base_port + 5, alex));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                957,
                base_port,
                [IncomingCommand::CreateFlat {
                    floor: "first floor".to_owned(),
                    room_name: "Alex Den".to_owned(),
                    room_model: "model_a".to_owned(),
                    state: 0,
                    show_owner_name: true,
                }],
            )],
        )
        .unwrap();

    let expected = b"#FLATCREATED\r1 Alex Den##";
    let mut bytes = vec![0; expected.len()];
    main_client.read_exact(&mut bytes).unwrap();
    let mut byte = [0; 1];

    assert_eq!(bytes, expected);
    assert_eq!(public_client.read(&mut byte).unwrap(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_try_flat_sends_flat_let_in_for_correct_password() {
    let (root, bootstrap) = bootstrap_with_config("incoming-try-flat", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 955, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(password_room(7, 2, "bob", "Locked Room", "door"));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("TRYFLAT /7/door"));

    client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();
    let expected = b"#FLAT_LETIN##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_private_room_login_uses_try_flat_room_on_private_socket() {
    let server_port = free_adjacent_port_pair();
    let private_server_port = server_port + 1;
    let (root, bootstrap) = bootstrap_with_config(
        "incoming-private-room-login",
        server_port,
        private_server_port,
    );
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1100, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room()
        .insert_room(private_room(37, 1, "Alex", "Alex Den"));
    dao.room()
        .insert_model(RoomModel::new("model_a", "00\r00", 0, 0, 0, 2, false, false).unwrap());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut main_client = connect_client(&mut application, &binder);
    let mut burst = Vec::new();
    burst.extend(client_frame("LOGIN Alex 123"));
    burst.extend(client_frame("TRYFLAT /37"));

    main_client.write_all(&burst).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
        )
        .unwrap();
    let expected_let_in = b"#FLAT_LETIN##";
    let mut let_in = vec![0; expected_let_in.len()];
    main_client.read_exact(&mut let_in).unwrap();
    assert_eq!(let_in, expected_let_in);

    let mut room_client = connect_client_at(&mut application, &binder, 1);
    room_client
        .write_all(&client_frame("LOGIN Alex 123"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
        )
        .unwrap();

    let text = read_available_text(&mut room_client);
    assert!(text.contains("#ROOM_READY\rdescription##"), "{text:?}");
    assert!(text.contains("#HEIGHTMAP\r0000##"), "{text:?}");
    assert!(
        text.contains("#USERS\r  Alex figure 0 0 0 mission##"),
        "{text:?}"
    );
    assert!(application
        .game()
        .player_manager()
        .players()
        .values()
        .any(
            |session| session.server_port() == i32::from(private_server_port)
                && session.room_user().is_some()
                && session.room_user().unwrap().room_id() == 37
        ));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_try_flat_doorbell_rings_online_room_owner() {
    let (root, bootstrap) = bootstrap_with_config("incoming-try-flat-doorbell", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 956, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let bob = player_dao.login("bob", "secret").unwrap().unwrap().details;
    let alice = player_dao
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(doorbell_room(7, bob.id(), bob.username(), "Doorbell Room"));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut bob_client = connect_client(&mut application, &binder);
    let mut alice_client = connect_client(&mut application, &binder);
    let mut bob_session = PlayerSession::new(956, 0, bob.clone());
    let mut bob_room_user = crate::game::room::entity::RoomUser::new(
        bob.id(),
        bob.username(),
        bob.figure(),
        bob.mission(),
        None::<String>,
    );
    bob_room_user.set_room_id(7);
    bob_session.set_room_user(bob_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(bob_session);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(957, 0, alice));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                957,
                0,
                [IncomingCommand::TryFlat {
                    room_id: 7,
                    password: String::new(),
                }],
            )],
        )
        .unwrap();
    let expected = b"#DOORBELL_RINGING\ralice##";
    let mut bytes = vec![0; expected.len()];
    bob_client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);
    assert_eq!(read_available_text(&mut alice_client), "");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_try_flat_leaves_current_room_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-try-flat-leaves-current-room", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 960, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let alice = player_dao
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let bob = player_dao.login("bob", "secret").unwrap().unwrap().details;
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(private_room(7, bob.id(), bob.username(), "Target Room"));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice_client = connect_client(&mut application, &binder);
    let mut bob_client = connect_client(&mut application, &binder);

    let old_room_port = 12;
    let mut old_room = RoomSummary::new(private_room(12, bob.id(), bob.username(), "Old Room"));
    old_room.set_player_count(2);
    application.game_mut().room_manager_mut().add(old_room);

    let mut alice_session = PlayerSession::new(960, old_room_port, alice.clone());
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice.id(),
        alice.username(),
        alice.figure(),
        alice.mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(12);
    alice_session.set_room_user(alice_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(alice_session);

    let mut bob_session = PlayerSession::new(961, old_room_port, bob.clone());
    let mut bob_room_user = crate::game::room::entity::RoomUser::new(
        bob.id(),
        bob.username(),
        bob.figure(),
        bob.mission(),
        None::<String>,
    );
    bob_room_user.set_room_id(12);
    bob_session.set_room_user(bob_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(bob_session);

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                960,
                old_room_port,
                [IncomingCommand::TryFlat {
                    room_id: 7,
                    password: String::new(),
                }],
            )],
        )
        .unwrap();

    let expected_alice = b"#FLAT_LETIN##";
    let mut alice_bytes = vec![0; expected_alice.len()];
    alice_client.read_exact(&mut alice_bytes).unwrap();
    assert_eq!(alice_bytes, expected_alice);

    let expected_bob = b"#LOGOUT\ralice##";
    let mut bob_bytes = vec![0; expected_bob.len()];
    bob_client.read_exact(&mut bob_bytes).unwrap();
    assert_eq!(bob_bytes, expected_bob);
    assert!(application
        .game()
        .player_manager()
        .players()
        .get(&960)
        .unwrap()
        .room_user()
        .is_none());
    assert_eq!(
        application
            .game()
            .room_manager()
            .get_room_by_id(12)
            .unwrap()
            .player_count(),
        1
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_room_login_then_chat_sends_chat_packet() {
    let (root, bootstrap) = bootstrap_with_config("incoming-room-chat", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1, None).unwrap();
    let player_dao = player_dao_with_alex();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client_at(&mut application, &binder, 1);

    client
        .write_all(&client_frame("LOGIN Alex 123 room"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    client.write_all(&client_frame("CHAT hello")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    let expected = b"#CHAT\rAlex hello##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_public_room_login_sends_room_bootstrap_without_extra_login_argument() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-public-room-login", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Main Lobby", "lobby");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Main Lobby")],
        1,
        None,
    )
    .unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(public.clone());
    let dao = InMemoryDao::new(player_dao_with_alex());
    dao.room().insert_room(public.data().clone());
    dao.room()
        .insert_model(RoomModel::new("pool_b", "00 00", 1, 1, 0, 2, false, false).unwrap());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut client = connect_client_at(&mut application, &binder, 2);

    client.write_all(&client_frame("LOGIN Alex 123")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
        )
        .unwrap();

    let text = read_available_text(&mut client);

    assert!(text.contains("#HEIGHTMAP\r00\r00##"), "{text:?}");
    assert!(text.contains("# OBJECTS WORLD 0 pool_b##"), "{text:?}");
    assert!(
        text.contains("#USERS\r  Alex figure 1 1 0 mission##"),
        "{text:?}"
    );
    assert!(application
        .game()
        .player_manager()
        .players()
        .values()
        .any(
            |session| session.server_port() == i32::from(server_port) + 1
                && session.room_user().is_some()
        ));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_chat_turns_nearby_recipient_toward_speaker_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-chat-look-at", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 994, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let alice = dao
        .player()
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let bob = dao
        .player()
        .login("bob", "secret")
        .unwrap()
        .unwrap()
        .details;

    let mut alice_client = connect_client(&mut application, &binder);
    let mut bob_client = connect_client(&mut application, &binder);
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice.id(),
        alice.username(),
        alice.figure(),
        alice.mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(42);
    alice_room_user.set_position(Position::with_rotation(0, 0, 0.0, 0));
    let mut alice_session = PlayerSession::new(994, 42, alice.clone());
    alice_session.set_room_user(alice_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(alice_session);

    let mut bob_room_user = crate::game::room::entity::RoomUser::new(
        bob.id(),
        bob.username(),
        bob.figure(),
        bob.mission(),
        None::<String>,
    );
    bob_room_user.set_room_id(42);
    bob_room_user.set_position(Position::with_rotation(2, 0, 0.0, 0));
    let mut bob_session = PlayerSession::new(995, 42, bob.clone());
    bob_session.set_room_user(bob_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(bob_session);

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                994,
                42,
                [IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                }],
            )],
        )
        .unwrap();

    let alice_text = read_available_text(&mut alice_client);
    let bob_text = read_available_text(&mut bob_client);
    assert!(alice_text.contains("#CHAT\ralice hello##"));
    assert!(bob_text.contains("#CHAT\ralice hello##"));
    assert!(bob_text.contains("#STATUS \rbob 2,0,0,1,0/##"));

    let bob_room_user = application
        .game()
        .player_manager()
        .players()
        .get(&995)
        .and_then(|session| session.room_user())
        .unwrap();
    assert_eq!(bob_room_user.position().head_rotation(), 1);
    assert_eq!(bob_room_user.look_reset_time(), 6);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_delete_flat_removes_items_loaded_room_and_room_connections() {
    let (root, bootstrap) = bootstrap_with_config("incoming-delete-flat-cleanup", 0, 5000);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 994, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let alice = dao
        .player()
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let bob = dao
        .player()
        .login("bob", "secret")
        .unwrap()
        .unwrap()
        .details;
    let room = private_room(42, alice.id(), alice.username(), "Owned");
    let room_port = room.server_port(5000);

    dao.room().insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    dao.item().insert_definition(item_definition(5, "chair"));
    dao.item().insert_item(floor_item(
        10,
        42,
        alice.id(),
        dao.item().definition(5).unwrap(),
    ));
    dao.item().insert_item(floor_item(
        11,
        99,
        alice.id(),
        dao.item().definition(5).unwrap(),
    ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(994, room_port, alice));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(995, room_port, bob));

    let unapplied = application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                994,
                room_port,
                [IncomingCommand::DeleteFlat { room_id: 42 }],
            )],
        )
        .unwrap();

    assert!(dao.room().room(42, false).unwrap().is_none());
    assert!(application
        .game()
        .room_manager()
        .get_room_by_id(42)
        .is_none());
    assert!(dao.item().item(10).unwrap().is_none());
    assert!(dao.item().item(11).unwrap().is_some());
    assert!(unapplied.contains(
        &crate::runtime::RoseauIncomingExecutionRuntimePlan::Network(
            PlayerNetworkEffect::CloseConnection { connection_id: 994 },
        )
    ));
    assert!(unapplied.contains(
        &crate::runtime::RoseauIncomingExecutionRuntimePlan::Network(
            PlayerNetworkEffect::CloseConnection { connection_id: 995 },
        )
    ));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_set_flat_info_refreshes_all_super_user_privileges() {
    let (root, bootstrap) = bootstrap_with_config("incoming-set-flat-info-privileges", 0, 5000);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 994, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let alice = dao
        .player()
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let bob = dao
        .player()
        .login("bob", "secret")
        .unwrap()
        .unwrap()
        .details;
    let room = private_room(42, alice.id(), alice.username(), "Owned");
    let room_port = room.server_port(5000);

    dao.room().insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(994, room_port, alice));
    let mut bob_room_user = crate::game::room::entity::RoomUser::new(
        bob.id(),
        bob.username(),
        bob.figure(),
        bob.mission(),
        None::<String>,
    );
    bob_room_user.set_room_id(42);
    let mut bob_session = PlayerSession::new(995, room_port, bob);
    bob_session.set_room_user(bob_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(bob_session);

    let unapplied = application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                994,
                room_port,
                [IncomingCommand::SetFlatInfo {
                    room_id: 42,
                    description: "new desc".to_owned(),
                    password: String::new(),
                    all_super_user: true,
                }],
            )],
        )
        .unwrap();

    assert!(dao
        .room()
        .room(42, false)
        .unwrap()
        .unwrap()
        .has_all_super_user());
    assert!(application
        .game()
        .player_manager()
        .players()
        .get(&995)
        .and_then(|session| session.room_user())
        .unwrap()
        .contains_status("flatctrl"));
    assert!(unapplied.contains(
        &crate::runtime::RoseauIncomingExecutionRuntimePlan::Network(
            PlayerNetworkEffect::WriteResponse {
                connection_id: 995,
                packet: "#YOUARECONTROLLER##".to_owned(),
            },
        )
    ));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_pool_packets_broadcast_to_room_connections() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-room-pool", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Habbo Lido")],
        1,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let pool_map = vec!["00000000000000000000"; 20].join(" ");
    dao.room()
        .insert_model(RoomModel::new("pool_b", pool_map, 0, 0, 0, 0, true, false).unwrap());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut alice = connect_client_at(&mut application, &binder, 2);
    let mut bob = connect_client_at(&mut application, &binder, 2);

    let mut login_burst = Vec::new();
    login_burst.extend(client_frame("LOGIN alice secret room"));
    login_burst.extend(client_frame("LOGIN bob secret room"));
    alice
        .write_all(&login_burst[..client_frame("LOGIN alice secret room").len()])
        .unwrap();
    bob.write_all(&login_burst[client_frame("LOGIN alice secret room").len()..])
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(dao.player(), dao.room(), &navigator_dao, dao.messenger())
        .unwrap();
    let room_port = i32::from(server_port) + 1;
    assert_eq!(
        application.game().player_manager().players().len(),
        2,
        "room logins should create sessions for both clients"
    );
    assert!(application
        .game()
        .player_manager()
        .players()
        .values()
        .all(|session| session.server_port() == room_port));
    let pool_lift = ItemDefinition::new(77, "poolLift", "", 1, 1, 0.0, "SIF", "Pool Lift", "", "");
    dao.item().insert_definition(pool_lift.clone());
    dao.item()
        .insert_item(Item::new(700, 1, 0, "0", 0, 0.0, 0, pool_lift, "SIF", None).unwrap());
    let alice_session = application
        .game()
        .player_manager()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .cloned()
        .unwrap();
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice_session.details().id(),
        alice_session.details().username(),
        alice_session.details().figure(),
        alice_session.details().mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(1);
    alice_room_user.set_current_item_id(Some(700));
    application
        .game_mut()
        .player_manager_mut()
        .get_mut(alice_session.connection_id())
        .unwrap()
        .set_room_user(alice_room_user);

    alice
        .write_all(&client_frame("JUMPPERF a\rb\rc\ract=jump\rheight=1"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let jump_batches = application.drain_pending_incoming_commands();
    assert_eq!(
        jump_batches[0].commands(),
        &[IncomingCommand::JumpPerformance {
            data: "act=jump".to_owned(),
        }]
    );
    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &jump_batches,
        )
        .unwrap();

    let expected_jump = b"#JUMPDATA\ralice\ract=jump##";
    let mut alice_jump = vec![0; expected_jump.len()];
    let mut bob_jump = vec![0; expected_jump.len()];
    alice.read_exact(&mut alice_jump).unwrap();
    bob.read_exact(&mut bob_jump).unwrap();
    assert_eq!(alice_jump, expected_jump);
    assert_eq!(bob_jump, expected_jump);

    alice
        .write_all(&client_frame("SPLASH_POSITION 17,18,0.0"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
        )
        .unwrap();

    let alice_splash = read_available_text(&mut alice);
    let bob_splash = read_available_text(&mut bob);
    assert!(
        alice_splash.contains("#SHOWPROGRAM\rBIGSPLASH POSITION 17,18##"),
        "{alice_splash:?}"
    );
    assert!(
        bob_splash.contains("#SHOWPROGRAM\rBIGSPLASH POSITION 17,18##"),
        "{bob_splash:?}"
    );
    assert!(
        alice_splash.contains("#SHOWPROGRAM\rSIF open##"),
        "{alice_splash:?}"
    );
    assert!(
        bob_splash.contains("#SHOWPROGRAM\rSIF open##"),
        "{bob_splash:?}"
    );
    let alice_room_user = application
        .game()
        .player_manager()
        .players()
        .get(&alice_session.connection_id())
        .unwrap()
        .room_user()
        .unwrap();
    assert_eq!(alice_room_user.position(), Position::new(17, 18, 0.0));
    assert!(alice_room_user.contains_status("swim"));
    assert_eq!(alice_room_user.goal(), Some(Position::new(18, 19, 0.0)));

    alice.write_all(&client_frame("Sign 3")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(dao.player(), dao.room(), &navigator_dao, dao.messenger())
        .unwrap();

    let expected_status = b"#STATUS \ralice 17,18,0,0,0/sign 3/swim/##";
    let mut alice_status = vec![0; expected_status.len()];
    let mut bob_status = vec![0; expected_status.len()];
    alice.read_exact(&mut alice_status).unwrap();
    bob.read_exact(&mut bob_status).unwrap();
    assert_eq!(alice_status, expected_status);
    assert_eq!(bob_status, expected_status);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_jump_performance_ignores_non_pool_lift_current_item_like_java() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-room-pool-non-lift", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Habbo Lido")],
        980,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut alice = connect_client_at(&mut application, &binder, 2);
    let mut bob = connect_client_at(&mut application, &binder, 2);

    alice
        .write_all(&client_frame("LOGIN alice secret room"))
        .unwrap();
    bob.write_all(&client_frame("LOGIN bob secret room"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(dao.player(), dao.room(), &navigator_dao, dao.messenger())
        .unwrap();

    let chair = ItemDefinition::new(78, "chair", "", 1, 1, 0.0, "SIF", "Chair", "", "");
    dao.item().insert_definition(chair.clone());
    dao.item()
        .insert_item(Item::new(701, 1, 0, "0", 0, 0.0, 0, chair, "", None).unwrap());
    let alice_session = application
        .game()
        .player_manager()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .cloned()
        .unwrap();
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice_session.details().id(),
        alice_session.details().username(),
        alice_session.details().figure(),
        alice_session.details().mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(1);
    alice_room_user.set_current_item_id(Some(701));
    application
        .game_mut()
        .player_manager_mut()
        .get_mut(alice_session.connection_id())
        .unwrap()
        .set_room_user(alice_room_user);

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                alice_session.connection_id(),
                i32::from(server_port) + 1,
                [IncomingCommand::JumpPerformance {
                    data: "act=jump".to_owned(),
                }],
            )],
        )
        .unwrap();

    assert_eq!(read_available_text(&mut alice), "");
    assert_eq!(read_available_text(&mut bob), "");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_splash_position_closes_user_connections_without_pool_lift_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-room-splash-invalid", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 990, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut acting_client = connect_client(&mut application, &binder);
    let mut other_client = connect_client(&mut application, &binder);
    let alice = dao
        .player()
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;

    let chair = ItemDefinition::new(79, "chair", "", 1, 1, 0.0, "SIF", "Chair", "", "");
    dao.item().insert_definition(chair.clone());
    dao.item()
        .insert_item(Item::new(702, 1, 0, "0", 0, 0.0, 0, chair, "", None).unwrap());
    let mut acting_session = PlayerSession::new(990, 7, alice.clone());
    let mut room_user = crate::game::room::entity::RoomUser::new(
        alice.id(),
        alice.username(),
        alice.figure(),
        alice.mission(),
        None::<String>,
    );
    room_user.set_room_id(1);
    room_user.set_current_item_id(Some(702));
    acting_session.set_room_user(room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(acting_session);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(991, 0, alice));

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                990,
                7,
                [IncomingCommand::SplashPosition {
                    position: "17,18,0.0".to_owned(),
                }],
            )],
        )
        .unwrap();

    let mut byte = [0; 1];
    assert_eq!(acting_client.read(&mut byte).unwrap(), 0);
    assert_eq!(other_client.read(&mut byte).unwrap(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_flat_property_by_item_broadcasts_to_room_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-decoration-broadcast", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 992, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let alice = dao
        .player()
        .login("alice", "secret")
        .unwrap()
        .unwrap()
        .details;
    let bob = dao
        .player()
        .login("bob", "secret")
        .unwrap()
        .unwrap()
        .details;
    let room = private_room(42, alice.id(), alice.username(), "Decor Room");
    dao.room().insert_room(room.clone());
    let mut loaded_room = RoomSummary::new(room);
    loaded_room.set_player_count(2);
    application.game_mut().room_manager_mut().add(loaded_room);
    let paper = ItemDefinition::new(80, "paper", "", 1, 1, 0.0, "V", "Paper", "", "");
    dao.item().insert_definition(paper.clone());
    dao.item().insert_item(
        Item::new(
            710,
            0,
            alice.id(),
            "0",
            0,
            0.0,
            0,
            paper,
            "",
            Some("101".to_owned()),
        )
        .unwrap(),
    );

    let mut alice_client = connect_client(&mut application, &binder);
    let mut bob_client = connect_client(&mut application, &binder);
    let mut alice_session = PlayerSession::new(992, 42, alice.clone());
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice.id(),
        alice.username(),
        alice.figure(),
        alice.mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(42);
    alice_session.set_room_user(alice_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(alice_session);
    let mut bob_session = PlayerSession::new(993, 42, bob.clone());
    let mut bob_room_user = crate::game::room::entity::RoomUser::new(
        bob.id(),
        bob.username(),
        bob.figure(),
        bob.mission(),
        None::<String>,
    );
    bob_room_user.set_room_id(42);
    bob_session.set_room_user(bob_room_user);
    application
        .game_mut()
        .player_manager_mut()
        .insert(bob_session);

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                992,
                42,
                [IncomingCommand::ApplyDecoration {
                    decoration: "wallpaper".to_owned(),
                    item_id: 710,
                }],
            )],
        )
        .unwrap();

    let alice_response = read_available_text(&mut alice_client);
    let bob_response = read_available_text(&mut bob_client);
    assert!(
        alice_response.contains("#FLATPROPERTY\rwallpaper/101##"),
        "{alice_response}"
    );
    assert!(alice_response.contains("#STRIPINFO##"), "{alice_response}");
    assert!(
        bob_response.contains("#FLATPROPERTY\rwallpaper/101##"),
        "{bob_response}"
    );
    assert!(!bob_response.contains("#STRIPINFO"), "{bob_response}");
    assert_eq!(dao.room().room(42, false).unwrap().unwrap().wall(), "101");
    assert!(dao.item().item(710).unwrap().is_none());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn assign_personal_message_updates_player_state_without_packet() {
    let (root, bootstrap) = bootstrap_with_config("assign-personal-message-runtime", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 910, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let messenger_dao = InMemoryMessengerDao::new();
    let login = player_dao.login("alice", "secret").unwrap().unwrap();

    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(970, 910, login.details));

    let plans = application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                970,
                910,
                [IncomingCommand::AssignPersonalMessage {
                    message: "hello there".to_owned(),
                }],
            )],
        )
        .unwrap();

    assert!(plans.is_empty());
    assert_eq!(
        player_dao
            .details_by_username("alice")
            .unwrap()
            .unwrap()
            .personal_greeting(),
        "hello there"
    );
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&970)
            .unwrap()
            .details()
            .personal_greeting(),
        "hello there"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_room_move_broadcasts_status_and_persists_room_user_state() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-room-move", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Habbo Lido")],
        1,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    room_dao
        .insert_model(RoomModel::new("pool_b", "000 000 000", 0, 0, 0, 0, false, false).unwrap());
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice = connect_client_at(&mut application, &binder, 2);
    let mut bob = connect_client_at(&mut application, &binder, 2);

    alice
        .write_all(&client_frame("LOGIN alice secret room"))
        .unwrap();
    bob.write_all(&client_frame("LOGIN bob secret room"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(&player_dao, &room_dao, &navigator_dao, &messenger_dao)
        .unwrap();

    alice.write_all(&client_frame("Move 1 0")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let move_batches = application.drain_pending_incoming_commands();
    assert_eq!(
        move_batches[0].commands(),
        &[IncomingCommand::WalkTo { x: 1, y: 0 }]
    );
    let unapplied = application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &move_batches,
        )
        .unwrap();
    assert!(unapplied.is_empty(), "{unapplied:?}");

    let alice_session = application
        .game()
        .player_manager()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .unwrap();
    let room_user = alice_session.room_user().unwrap();
    assert!(room_user.is_walking());
    assert!(room_user.next().is_none());
    assert_eq!(room_user.path().front().unwrap().x(), 1);
    assert_eq!(room_user.path().front().unwrap().y(), 0);

    let alice_status = read_available_text(&mut alice);
    let bob_status = read_available_text(&mut bob);
    assert_eq!(alice_status, "");
    assert_eq!(bob_status, "");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_into_door_walks_to_adjacent_teleporter_and_broadcasts_status() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-room-into-door", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Habbo Lido")],
        1,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let player_dao = InMemoryPlayerDao::new();
    player_dao
        .create_player(&CreatePlayer::new(
            "alice",
            "secret",
            "alice@example.test",
            "mission",
            "figure",
            1289,
            "Female",
            "1990-01-01",
        ))
        .unwrap();
    player_dao
        .create_player(&CreatePlayer::new(
            "bob",
            "secret",
            "bob@example.test",
            "mission",
            "figure",
            1289,
            "Male",
            "1990-01-01",
        ))
        .unwrap();
    let dao = InMemoryDao::new(player_dao);
    dao.room()
        .insert_model(RoomModel::new("pool_b", "000 000 000", 0, 0, 0, 0, false, false).unwrap());
    let teleporter_definition =
        ItemDefinition::new(70, "teleport", "", 1, 1, 0.0, "SFX", "", "", "DOOROPEN");
    dao.item().insert_definition(teleporter_definition.clone());
    dao.item().insert_item(
        Item::new(
            501,
            1,
            7,
            "1",
            0,
            0.0,
            0,
            teleporter_definition,
            "",
            Some("TRUE".to_owned()),
        )
        .unwrap(),
    );
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let mut alice = connect_client_at(&mut application, &binder, 2);
    let mut bob = connect_client_at(&mut application, &binder, 2);

    alice
        .write_all(&client_frame("LOGIN alice secret room"))
        .unwrap();
    bob.write_all(&client_frame("LOGIN bob secret room"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(dao.player(), dao.room(), &navigator_dao, dao.messenger())
        .unwrap();

    let alice_session = application
        .game_mut()
        .player_manager_mut()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .cloned()
        .unwrap();
    let mut alice_room_user = crate::game::room::entity::RoomUser::new(
        alice_session.details().id(),
        alice_session.details().username(),
        alice_session.details().figure(),
        alice_session.details().mission(),
        None::<String>,
    );
    alice_room_user.set_room_id(1);
    alice_room_user.set_position(crate::game::room::model::Position::new(2, 0, 0.0));
    application
        .game_mut()
        .player_manager_mut()
        .get_mut(alice_session.connection_id())
        .unwrap()
        .set_room_user(alice_room_user);

    alice.write_all(&client_frame("IntoDoor 501")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let into_door_batches = application.drain_pending_incoming_commands();
    assert_eq!(
        into_door_batches[0].commands(),
        &[IncomingCommand::EnterDoor { item_id: 501 }]
    );
    let unapplied = application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &into_door_batches,
        )
        .unwrap();
    assert!(unapplied.is_empty(), "{unapplied:?}");

    assert_eq!(read_available_text(&mut alice), "");
    assert_eq!(read_available_text(&mut bob), "");

    let alice_session = application
        .game()
        .player_manager()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .unwrap();
    let room_user = alice_session.room_user().unwrap();
    assert!(room_user.is_walking());
    assert!(room_user.next().is_none());
    assert_eq!(room_user.path().front().unwrap().x(), 1);
    assert_eq!(room_user.path().front().unwrap().y(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_assign_rights_persists_and_sends_controller_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-assign-rights", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 970, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Rights Room");
    room_dao.insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            970,
            42,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            971,
            42,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                970,
                42,
                [IncomingCommand::AssignRights {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    let expected_controller = b"#YOUARECONTROLLER##";
    let expected_status = b"#STATUS \rbob 0,0,0,0,0/flatctrl/##";
    let mut bob_controller = vec![0; expected_controller.len()];
    let mut alice_status = vec![0; expected_status.len()];
    let mut bob_status = vec![0; expected_status.len()];
    bob.read_exact(&mut bob_controller).unwrap();
    alice.read_exact(&mut alice_status).unwrap();
    bob.read_exact(&mut bob_status).unwrap();

    assert_eq!(bob_controller, expected_controller);
    assert_eq!(alice_status, expected_status);
    assert_eq!(bob_status, expected_status);
    assert_eq!(room_dao.room_rights(42).unwrap(), vec![2]);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_assign_rights_all_rights_user_can_control_private_room_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-assign-rights-admin", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 970, None).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .set_permissions([Permission::new("room_all_rights", true, 5)]);
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Rights Room");
    room_dao.insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut admin = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    let mut admin_details = PlayerDetails::new();
    admin_details.fill_full(
        3,
        "admin",
        "mission",
        "figure",
        "",
        "admin@example.test",
        5,
        0,
        "M",
        "",
        "",
        "",
        0,
        "",
        0,
    );
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(970, 42, admin_details));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            971,
            42,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                970,
                42,
                [IncomingCommand::AssignRights {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    let bob_response = read_available_text(&mut bob);
    let admin_response = read_available_text(&mut admin);
    assert!(
        bob_response.contains("#YOUARECONTROLLER##"),
        "{bob_response}"
    );
    assert!(
        bob_response.contains("#STATUS \rbob 0,0,0,0,0/flatctrl/##"),
        "{bob_response}"
    );
    assert!(
        admin_response.contains("#STATUS \rbob 0,0,0,0,0/flatctrl/##"),
        "{admin_response}"
    );
    assert_eq!(room_dao.room_rights(42).unwrap(), vec![2]);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_let_user_in_sends_flat_let_in_to_waiting_private_connection() {
    let (root, bootstrap) = bootstrap_with_config("incoming-let-user-in", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 980, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Doorbell Room");
    room_dao.insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut _alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            980,
            42,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            981,
            0,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                980,
                42,
                [IncomingCommand::LetUserIn {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    let expected = b"#FLAT_LETIN##";
    let mut bytes = vec![0; expected.len()];
    bob.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_remove_rights_persists_and_sends_no_controller_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-remove-rights", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 990, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Rights Room");
    room_dao.insert_room(room.clone());
    room_dao.save_room_rights(42, &[2]).unwrap();
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            990,
            42,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            991,
            42,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                990,
                42,
                [IncomingCommand::RemoveRights {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    let expected_controller = b"#YOUARENOTCONTROLLER##";
    let expected_status = b"#STATUS \rbob 0,0,0,0,0/##";
    let mut bob_controller = vec![0; expected_controller.len()];
    let mut alice_status = vec![0; expected_status.len()];
    let mut bob_status = vec![0; expected_status.len()];
    bob.read_exact(&mut bob_controller).unwrap();
    alice.read_exact(&mut alice_status).unwrap();
    bob.read_exact(&mut bob_status).unwrap();

    assert_eq!(bob_controller, expected_controller);
    assert_eq!(alice_status, expected_status);
    assert_eq!(bob_status, expected_status);
    assert!(room_dao.room_rights(42).unwrap().is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_kill_user_closes_target_room_connection() {
    let (root, bootstrap) = bootstrap_with_config("incoming-kill-user", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1000, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Kick Room");
    room_dao.insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut _alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1000,
            42,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1001,
            42,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                1000,
                42,
                [IncomingCommand::KickUser {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    let mut byte = [0; 1];
    assert_eq!(bob.read(&mut byte).unwrap(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn live_close_uimakoppi_opens_booth_and_walks_out() {
    let server_port = free_adjacent_port_pair();
    let (root, bootstrap) = bootstrap_with_config("incoming-close-uimakoppi", server_port, 0);
    let binder = StdTcpSocketBinder::new();
    let public = public_room(1, "Habbo Lido", "lido");
    let mut application = RoseauApplicationRuntime::prepare(
        &bootstrap,
        &binder,
        [PublicRoomDescriptor::new(1, "Habbo Lido")],
        1,
        None,
    )
    .unwrap();
    application.game_mut().room_manager_mut().add(public);
    let dao = InMemoryDao::new(player_dao_with_alice());
    dao.room()
        .insert_model(RoomModel::new("pool_b", "00", 0, 0, 0, 0, false, false).unwrap());
    let booth_definition =
        ItemDefinition::new(7, "poolBooth", "", 1, 1, 0.0, "SF", "Pool Booth", "", "");
    dao.item().insert_definition(booth_definition.clone());
    dao.item().insert_item(
        Item::new(
            70,
            1,
            0,
            "0",
            0,
            0.0,
            0,
            booth_definition,
            "pool_booth",
            Some("1,0".to_owned()),
        )
        .unwrap(),
    );
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice = connect_client_at(&mut application, &binder, 2);

    alice
        .write_all(&client_frame("LOGIN alice secret room"))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands(dao.player(), dao.room(), &navigator_dao, &messenger_dao)
        .unwrap();

    alice.write_all(&client_frame("CLOSE_UIMAKOPPI")).unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    application
        .apply_pending_incoming_commands_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            &messenger_dao,
        )
        .unwrap();

    let output = read_available_text(&mut alice);
    assert_eq!(output, "#SHOWPROGRAM\rpool_booth open##");
    let room_user = application
        .game()
        .player_manager()
        .players()
        .values()
        .find(|session| session.details().username() == "alice")
        .unwrap()
        .room_user()
        .unwrap();
    assert!(room_user.can_walk());
    assert!(room_user.is_walking());
    assert!(room_user.next().is_none());
    assert_eq!(room_user.path().front().unwrap().x(), 1);
    assert_eq!(room_user.path().front().unwrap().y(), 0);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_add_item_places_wall_item_and_broadcasts_to_room() {
    let (root, bootstrap) = bootstrap_with_config("incoming-add-wall-item", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1020, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::default();
    let room = private_room(42, 1, "alice", "Item Room");
    dao.room().insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    dao.item().insert_definition(ItemDefinition::new(
        5, "poster", "red", 1, 1, 0.0, "SIW", "Poster", "", "",
    ));
    dao.inventory().new_item(5, 1, "").unwrap();

    let mut alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1020,
            42,
            dao.player()
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1021,
            42,
            dao.player()
                .login("bob", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                1020,
                42,
                [IncomingCommand::AddWallItem {
                    sprite: "poster".to_owned(),
                    wall_position: "frontwall -2.8438,-8.4444,2844".to_owned(),
                    extra_data: "FFFF31 note".to_owned(),
                }],
            )],
        )
        .unwrap();

    let placed_item = dao
        .item()
        .room_items(42)
        .unwrap()
        .into_values()
        .find(|item| item.definition().sprite() == "poster")
        .unwrap();
    let expected = AddWallItemPacket::new(placed_item.clone()).compose().get();
    let mut alice_packet = vec![0; expected.len()];
    let mut bob_packet = vec![0; expected.len()];
    alice.read_exact(&mut alice_packet).unwrap();
    bob.read_exact(&mut bob_packet).unwrap();

    assert_eq!(alice_packet, expected.as_bytes());
    assert_eq!(bob_packet, expected.as_bytes());
    assert_eq!(placed_item.owner_id(), 1);
    assert_eq!(placed_item.room_id(), 42);
    assert_eq!(
        placed_item.wall_position(),
        Some("frontwall -2.8438,-8.4444,2844")
    );
    assert_eq!(placed_item.custom_data(), Some("FFFF31 note"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_add_item_requires_explicit_rights_like_java() {
    let (root, bootstrap) = bootstrap_with_config("incoming-add-wall-item-rights", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1022, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::default();
    let mut room = private_room(42, 1, "alice", "All Super Item Room");
    room.set_all_super_user(true);
    dao.room().insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    dao.item().insert_definition(ItemDefinition::new(
        5, "poster", "red", 1, 1, 0.0, "SIW", "Poster", "", "",
    ));
    dao.inventory().new_item(5, 2, "").unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1022,
            42,
            dao.player()
                .login("bob", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                1022,
                42,
                [IncomingCommand::AddWallItem {
                    sprite: "poster".to_owned(),
                    wall_position: "frontwall -2.8438,-8.4444,2844".to_owned(),
                    extra_data: "FFFF31 note".to_owned(),
                }],
            )],
        )
        .unwrap();

    assert!(dao.item().room_items(42).unwrap().is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_pool_figure_update_persists_session_and_broadcasts_users() {
    let (root, bootstrap) = bootstrap_with_config("incoming-update-pool-figure", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1030, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let room = private_room(42, 1, "alice", "Pool Figure Room");
    room_dao.insert_room(room.clone());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut alice = connect_client(&mut application, &binder);
    let mut bob = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1030,
            42,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1031,
            42,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                1030,
                42,
                [IncomingCommand::UpdatePoolFigure {
                    pool_figure: "ph=001".to_owned(),
                }],
            )],
        )
        .unwrap();

    let expected = b"#USERS\r  alice figure 0 0 0 mission ph=001##";
    let mut alice_packet = vec![0; expected.len()];
    let mut bob_packet = vec![0; expected.len()];
    alice.read_exact(&mut alice_packet).unwrap();
    bob.read_exact(&mut bob_packet).unwrap();

    assert_eq!(alice_packet, expected);
    assert_eq!(bob_packet, expected);
    assert_eq!(
        player_dao
            .details_by_username("alice")
            .unwrap()
            .unwrap()
            .pool_figure(),
        "ph=001"
    );
    assert_eq!(
        application
            .game()
            .player_manager()
            .players()
            .get(&1030)
            .unwrap()
            .details()
            .pool_figure(),
        "ph=001"
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_profile_update_refreshes_active_session_details() {
    let (root, bootstrap) = bootstrap_with_config("incoming-update-profile-session", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1040, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut _alice = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1040,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                1040,
                0,
                [IncomingCommand::UpdateProfile {
                    password: "newsecret".to_owned(),
                    email: "new@example.test".to_owned(),
                    figure: "hd=200".to_owned(),
                    mission: "new mission".to_owned(),
                    sex: "Female".to_owned(),
                }],
            )],
        )
        .unwrap();

    let session_details = application
        .game()
        .player_manager()
        .players()
        .get(&1040)
        .unwrap()
        .details();
    assert_eq!(session_details.email(), "new@example.test");
    assert_eq!(session_details.figure(), "hd=200");
    assert_eq!(session_details.mission(), "new mission");
    assert_eq!(session_details.sex(), "Female");

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_go_to_flat_sends_room_bootstrap_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-goto-flat", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 1050, None).unwrap();
    let dao = InMemoryDao::new(player_dao_with_alice_and_bob());
    let navigator_dao = InMemoryNavigatorDao::default();
    let mut room = private_room(42, 1, "alice", "Bootstrap Room");
    room.set_wall("101");
    room.set_floor("2");
    dao.room().insert_room(room.clone());
    dao.room()
        .insert_model(RoomModel::new("model_a", "00 00", 0, 0, 0, 2, false, false).unwrap());
    application
        .game_mut()
        .room_manager_mut()
        .add(RoomSummary::new(room));

    let passive_definition =
        ItemDefinition::new(3, "plant", "green", 1, 1, 0.0, "SIP", "Plant", "", "");
    let floor_definition =
        ItemDefinition::new(4, "chair", "red", 1, 1, 1.0, "SIF", "Chair", "", "");
    let wall_definition =
        ItemDefinition::new(5, "poster", "blue", 1, 1, 0.0, "SIW", "Poster", "", "");
    dao.item().insert_definition(passive_definition.clone());
    dao.item().insert_definition(floor_definition.clone());
    dao.item().insert_definition(wall_definition.clone());
    dao.item().insert_item(
        Item::new(
            30,
            42,
            0,
            "2",
            1,
            0.0,
            0,
            passive_definition,
            "model_a",
            None,
        )
        .unwrap(),
    );
    dao.item()
        .insert_item(Item::new(31, 42, 1, "1", 1, 0.0, 0, floor_definition, "", None).unwrap());
    dao.item().insert_item(
        Item::new(
            32,
            42,
            1,
            "frontwall -2.8438,-8.4444,2844",
            0,
            0.0,
            0,
            wall_definition,
            "",
            Some("FFFF31 note".to_owned()),
        )
        .unwrap(),
    );

    let mut alice = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            1050,
            42,
            dao.player()
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches_with_catalogue(
            dao.player(),
            dao.room(),
            dao.catalogue(),
            dao.inventory(),
            dao.item(),
            &navigator_dao,
            dao.messenger(),
            &[PendingIncomingCommandBatch::new(
                1050,
                42,
                [IncomingCommand::GoToFlat],
            )],
        )
        .unwrap();

    let mut room_items = dao
        .item()
        .room_items(42)
        .unwrap()
        .into_values()
        .collect::<Vec<_>>();
    room_items.sort_by_key(|item| item.id());
    let passive_items = dao
        .item()
        .public_room_items("model_a", 42)
        .unwrap()
        .into_values()
        .collect::<Vec<_>>();
    let expected = [
        RoomReady::new("description").compose().get(),
        FlatProperty::new("wallpaper", "101").compose().get(),
        FlatProperty::new("floor", "2").compose().get(),
        YouAreOwner.compose().get(),
        HeightMap::new("00\r00").compose().get(),
        ObjectsWorld::new("model_a", passive_items).compose().get(),
        ActiveObjects::new(
            room_items
                .iter()
                .filter(|item| item.definition().behaviour().is_on_floor())
                .cloned(),
        )
        .compose()
        .get(),
        Items::new(
            room_items
                .iter()
                .filter(|item| item.definition().behaviour().is_on_wall())
                .cloned(),
        )
        .compose()
        .get(),
        Users::new([] as [UserEntry; 0]).compose().get(),
        Status::new([]).compose().get(),
        Users::new([UserEntry::new(
            "alice",
            "figure",
            0,
            0,
            0.0,
            "mission",
            None::<String>,
        )])
        .compose()
        .get(),
        Status::new([application
            .game()
            .player_manager()
            .players()
            .get(&1050)
            .and_then(|session| session.room_user())
            .map(|user| user.status_entity())
            .unwrap()])
        .compose()
        .get(),
    ]
    .concat();
    let mut bytes = vec![0; expected.len()];
    alice.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected.as_bytes());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_info_retrieve_sends_user_object() {
    let (root, bootstrap) = bootstrap_with_config("incoming-info-retrieve", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 941, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            941,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                941,
                0,
                [IncomingCommand::RetrieveUserInfo],
            )],
        )
        .unwrap();
    let expected_prefix = b"#USEROBJECT\rname=alice";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_find_user_and_empty_busy_flat_search_send_java_packets() {
    let (root, bootstrap) = bootstrap_with_config("incoming-find-navigator", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 942, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            942,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                942,
                0,
                [
                    IncomingCommand::FindUser {
                        username: "alice".to_owned(),
                    },
                    IncomingCommand::FindUser {
                        username: "missing".to_owned(),
                    },
                    IncomingCommand::EmptySearchBusyFlats,
                ],
            )],
        )
        .unwrap();
    let expected =
        b"#MEMBERINFO \ralice\r\rnow\rOn Hotel View\rfigure###NOSUCHUSER###BUSY_FLAT_RESULTS 1##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_messenger_init_sends_persistent_message_and_ready() {
    let (root, bootstrap) = bootstrap_with_config("incoming-messenger-init", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 943, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            943,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                943,
                0,
                [IncomingCommand::InitMessenger],
            )],
        )
        .unwrap();
    let expected = b"#MYPERSISTENTMSG\r###MESSENGERREADY##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_init_unit_listener_sends_all_units() {
    let (root, bootstrap) = bootstrap_with_config("incoming-unit-listener", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 943, None).unwrap();
    let player_dao = player_dao_with_alice();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    let mut client = connect_client(&mut application, &binder);
    application
        .game_mut()
        .room_manager_mut()
        .add(public_room(5, "Habbo Lido", "lido"));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                943,
                0,
                [IncomingCommand::InitUnitListener],
            )],
        )
        .unwrap();
    let expected_prefix = b"#ALLUNITS\rHabbo Lido";
    let mut bytes = vec![0; expected_prefix.len()];
    client.read_exact(&mut bytes).unwrap();

    assert_eq!(bytes, expected_prefix);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_messenger_request_buddy_persists_request() {
    let (root, bootstrap) = bootstrap_with_config("incoming-messenger-request", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 944, None).unwrap();
    let player_dao = player_dao_with_alice_and_bob();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            944,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                944,
                0,
                [IncomingCommand::RequestBuddy {
                    username: "bob".to_owned(),
                }],
            )],
        )
        .unwrap();

    assert!(messenger_dao.request_exists(1, 2).unwrap());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pending_messenger_send_delivers_to_multiple_online_recipients() {
    let (root, bootstrap) = bootstrap_with_config("incoming-messenger-multi-send", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 950, None).unwrap();
    let player_dao = player_dao_with_alice_bob_and_carol();
    let room_dao = InMemoryRoomDao::new();
    let navigator_dao = InMemoryNavigatorDao::default();
    let messenger_dao = InMemoryMessengerDao::new().with_current_time(1234);
    messenger_dao.new_friend(1, 2).unwrap();
    messenger_dao.new_friend(1, 3).unwrap();
    let mut _alice_client = connect_client(&mut application, &binder);
    let mut bob_client = connect_client(&mut application, &binder);
    let mut carol_client = connect_client(&mut application, &binder);
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            950,
            0,
            player_dao
                .login("alice", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            951,
            0,
            player_dao.login("bob", "secret").unwrap().unwrap().details,
        ));
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(
            952,
            0,
            player_dao
                .login("carol", "secret")
                .unwrap()
                .unwrap()
                .details,
        ));

    application
        .apply_pending_incoming_command_batches(
            &player_dao,
            &room_dao,
            &navigator_dao,
            &messenger_dao,
            &[PendingIncomingCommandBatch::new(
                950,
                0,
                [IncomingCommand::SendMessengerMessage {
                    receiver_ids: vec![2, 3],
                    message: "hello".to_owned(),
                }],
            )],
        )
        .unwrap();

    let bob_expected = b"#MESSENGER_MSG\r1\r1\r[]\r1234\rhello\rfigure\r##";
    let mut bob_bytes = vec![0; bob_expected.len()];
    bob_client.read_exact(&mut bob_bytes).unwrap();
    assert_eq!(bob_bytes, bob_expected);

    let carol_expected = b"#MESSENGER_MSG\r2\r1\r[]\r1234\rhello\rfigure\r##";
    let mut carol_bytes = vec![0; carol_expected.len()];
    carol_client.read_exact(&mut carol_bytes).unwrap();
    assert_eq!(carol_bytes, carol_expected);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_password_action_duplicate_connection_closes() {
    let (root, bootstrap) = bootstrap_with_config("password-runtime-close", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 920, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();
    application
        .game_mut()
        .player_manager_mut()
        .insert(PlayerSession::new(920, 37120, player_details(7, "alice")));
    let report = MySqlPlayerPasswordActionExecutionReport::new(
        MySqlPlayerPasswordActionReport::from_outcomes(
            [PlayerPasswordActionOutcome::Login(
                PlayerLoginOutcome::authenticated(
                    &player_details(7, "alice"),
                    "secret",
                    false,
                    37120,
                    37120,
                    Some(920),
                ),
            )],
            921,
            1234,
        ),
        SqlExecutionBatchResult::new([]),
    );

    let unapplied = application.apply_password_action_runtime_plans(&report);

    assert!(unapplied.is_empty());
    assert!(application
        .startup_runtime()
        .tcp_runtime()
        .unwrap()
        .connections()
        .is_empty());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn applies_direct_incoming_execution_network_plans_to_active_connections() {
    let (root, bootstrap) = bootstrap_with_config("incoming-runtime-packet", 0, 0);
    let binder = StdTcpSocketBinder::new();
    let mut application =
        RoseauApplicationRuntime::prepare(&bootstrap, &binder, [], 930, None).unwrap();
    let address = binder.local_addresses().unwrap()[0];
    let mut client = TcpStream::connect(address).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    application.startup_runtime_mut().run_loop_step(&binder);
    let mut hello = [0; 8];
    client.read_exact(&mut hello).unwrap();

    let unapplied = application.apply_incoming_execution_runtime_plans(
        &[IncomingExecutionEffect::Command(CommandEffect::SendAlert(
            "hello".to_owned(),
        ))],
        930,
    );
    let expected = b"#SYSTEMBROADCAST\rhello##";
    let mut bytes = vec![0; expected.len()];
    client.read_exact(&mut bytes).unwrap();

    assert!(unapplied.is_empty());
    assert_eq!(bytes, expected);

    fs::remove_dir_all(root).unwrap();
}
