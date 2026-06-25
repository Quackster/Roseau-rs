use roseau::dao::mysql::{
    MySqlApplicationTickExecutor, MySqlDaoEffect, MySqlDriver, MySqlRoomDao, MySqlStorageConnector,
    Storage, StorageSqlExecutor,
};
use roseau::dao::RoomDao;
use roseau::runtime::RoseauApplicationRuntime;
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
    let room_dao = MySqlRoomDao::new(
        StorageSqlExecutor::new(driver.clone()),
        "",
        HashMap::new(),
        0,
    );
    let public_room_ids = match room_dao.public_room_ids() {
        Ok(public_room_ids) => public_room_ids,
        Err(error) => {
            eprintln!("{error:?}");
            return;
        }
    };
    let tick_executor = MySqlApplicationTickExecutor::new(StorageSqlExecutor::new(driver));
    let resolver = StdHostResolver::new();
    let mut room_afk_states = Vec::new();

    let prepare_report = match RoseauApplicationRuntime::prepare_with_database_connector(
        &bootstrap,
        &binder,
        &connector,
        public_room_ids,
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
    let main_server_players = Vec::<(i32, i32)>::new();
    let loop_runner = settings.loop_runner();

    if let Err(error) = loop_runner.run(
        &mut application,
        &tick_executor,
        &resolver,
        &binder,
        &main_server_players,
        &mut room_afk_states,
    ) {
        eprintln!("{error:?}");
    }
}
