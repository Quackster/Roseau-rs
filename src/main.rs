use roseau::dao::mysql::{
    MySqlApplicationTickExecutor, MySqlCatalogueDao, MySqlDaoEffect, MySqlDriver,
    MySqlInventoryDao, MySqlItemDao, MySqlMessengerDao, MySqlNavigatorDao, MySqlPlayerDao,
    MySqlRoomDao, MySqlStorageConnector, PlayerPasswordQueries, RoomQueries, RoomResultMapper,
    SqlExecutor, Storage, StorageSqlExecutor,
};
use roseau::dao::{CatalogueDao, ItemDao, RoomDao};
use roseau::runtime::{IncomingDaoSet, RoseauApplicationRuntime};
use roseau::{RoseauApplicationEntrypointArguments, RoseauApplicationEntrypointUsage};
use roseau::{StdHostResolver, StdTcpSocketBinder};
use std::collections::HashMap;

fn main() {
    let settings = match RoseauApplicationEntrypointArguments::parse(std::env::args().skip(1)) {
        RoseauApplicationEntrypointArguments::Run(settings) => settings,
        RoseauApplicationEntrypointArguments::Usage => {
            println!("{}", RoseauApplicationEntrypointUsage::new().text());
            return;
        }
        RoseauApplicationEntrypointArguments::Error(error) => {
            eprintln!("{}", error.message());
            eprintln!();
            eprintln!("{}", RoseauApplicationEntrypointUsage::new().text());
            return;
        }
    };
    let bootstrap = settings.bootstrap();
    if let Err(error) = bootstrap.ensure_config_files() {
        eprintln!("{error}");
        return;
    }
    let binder = StdTcpSocketBinder::new();
    let storage = match bootstrap
        .load_main_config()
        .map_err(|error| error.to_string())
        .and_then(|config| Storage::from_config(&config).map_err(|error| error.to_string()))
    {
        Ok(storage) => storage,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    let driver = match MySqlDriver::connect_storage(&storage) {
        Ok(driver) => driver,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    let connector = MySqlStorageConnector::with_driver(driver.clone());
    let startup_item_dao =
        MySqlItemDao::new(StorageSqlExecutor::new(driver.clone()), HashMap::new());
    let item_definitions = match startup_item_dao.definitions() {
        Ok(definitions) => definitions,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let room_models = match StorageSqlExecutor::new(driver.clone())
        .execute(RoomQueries::models().read_plan())
        .and_then(|result| RoomResultMapper::room_models(result))
    {
        Ok(models) => models,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let room_dao = MySqlRoomDao::new(
        StorageSqlExecutor::new(driver.clone()),
        "",
        room_models.clone(),
        0,
    );
    let public_rooms = match room_dao.public_room_descriptors() {
        Ok(public_rooms) => public_rooms,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let public_room_data = match room_dao.public_rooms(false) {
        Ok(public_rooms) => public_rooms,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let startup_catalogue_dao = MySqlCatalogueDao::new(StorageSqlExecutor::new(driver.clone()));
    let catalogue_items = match startup_catalogue_dao.buyable_items() {
        Ok(items) => items,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let catalogue_deals = match startup_catalogue_dao.item_deals() {
        Ok(deals) => deals,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let tick_executor = MySqlApplicationTickExecutor::new(StorageSqlExecutor::new(driver.clone()));
    let resolver = StdHostResolver::new();
    let mut room_afk_states = Vec::new();

    let prepare_report = match RoseauApplicationRuntime::prepare_with_database_connector(
        &bootstrap,
        &binder,
        &connector,
        public_rooms,
        1,
        None,
    ) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };

    let mut log_lines = prepare_report
        .database_report()
        .effects()
        .iter()
        .filter_map(|effect| match effect {
            MySqlDaoEffect::LogLine(line) => Some(line.clone()),
            MySqlDaoEffect::ConnectStorage
            | MySqlDaoEffect::ConstructPlayerDao
            | MySqlDaoEffect::ConstructRoomDao
            | MySqlDaoEffect::ConstructItemDao
            | MySqlDaoEffect::ConstructCatalogueDao
            | MySqlDaoEffect::ConstructInventoryDao
            | MySqlDaoEffect::ConstructNavigatorDao
            | MySqlDaoEffect::ConstructMessengerDao => None,
        })
        .collect::<Vec<_>>();
    if let Some(application_runtime) = prepare_report.application_runtime() {
        log_lines.extend(application_runtime.startup_log_lines().iter().cloned());
    }
    for line in &log_lines {
        println!("{line}");
        if let Err(error) = prepare_report.logger().write_output_line(line) {
            eprintln!("{error:?}");
        }
    }

    if !prepare_report.ready() {
        return;
    }

    let mut application = prepare_report
        .into_application_runtime()
        .expect("ready prepare report has runtime");
    application
        .game_mut()
        .item_manager_mut()
        .load_definitions(item_definitions.values().cloned());
    application.load_catalogue(catalogue_items.into_values(), catalogue_deals.into_values());
    application.load_public_rooms(public_room_data);
    let variables = application
        .game()
        .variables()
        .expect("prepared application has loaded game variables");
    let incoming_player_dao = MySqlPlayerDao::new(
        StorageSqlExecutor::new(driver.clone()),
        PlayerPasswordQueries::java_compatible(),
        variables.user_default_credits(),
        variables.messenger_greeting(),
        0,
    );
    let incoming_room_dao =
        MySqlRoomDao::new(StorageSqlExecutor::new(driver.clone()), "", room_models, 0);
    let incoming_item_dao = MySqlItemDao::new(
        StorageSqlExecutor::new(driver.clone()),
        item_definitions.clone(),
    );
    let incoming_catalogue_dao = MySqlCatalogueDao::new(StorageSqlExecutor::new(driver.clone()));
    let incoming_inventory_dao =
        MySqlInventoryDao::new(StorageSqlExecutor::new(driver.clone()), item_definitions);
    let incoming_navigator_dao =
        MySqlNavigatorDao::new(StorageSqlExecutor::new(driver.clone()), "");
    let incoming_messenger_dao = MySqlMessengerDao::new(StorageSqlExecutor::new(driver), 0);
    let main_server_players = Vec::<(i32, i32)>::new();
    let loop_runner = settings.loop_runner();

    if let Err(error) = loop_runner.run_with_incoming_daos(
        &mut application,
        &tick_executor,
        &resolver,
        &binder,
        &main_server_players,
        &mut room_afk_states,
        IncomingDaoSet::new(
            &incoming_player_dao,
            &incoming_room_dao,
            &incoming_catalogue_dao,
            &incoming_inventory_dao,
            &incoming_item_dao,
            &incoming_navigator_dao,
            &incoming_messenger_dao,
        ),
    ) {
        eprintln!("{error:?}");
    }
}
