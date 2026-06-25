use roseau::dao::mysql::{
    MySqlApplicationTickExecutor, MySqlDriver, MySqlStorageConnector, Storage, StorageSqlExecutor,
};
use roseau::{
    RoseauApplicationEntrypointArguments, RoseauApplicationEntrypointRunner,
    RoseauApplicationEntrypointUsage, StdHostResolver, StdTcpSocketBinder,
};

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
    let tick_executor = MySqlApplicationTickExecutor::new(StorageSqlExecutor::new(driver));
    let resolver = StdHostResolver::new();
    let runner = RoseauApplicationEntrypointRunner::new(settings.loop_runner());
    let mut room_afk_states = Vec::new();

    match runner.run(
        &bootstrap,
        &binder,
        &connector,
        &tick_executor,
        &resolver,
        [],
        settings.first_connection_id(),
        None,
        settings.listener_index(),
        settings.accept_connection(),
        settings.max_bytes(),
        &[],
        &mut room_afk_states,
    ) {
        Ok(report) => {
            for line in report.log_lines() {
                println!("{line}");
            }
            if let Err(error) = report.write_output_logs() {
                eprintln!("{error:?}");
            }
        }
        Err(error) => {
            eprintln!("{error:?}");
        }
    }
}
