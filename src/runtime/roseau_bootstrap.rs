use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::dao::mysql::DatabaseEngine;
use crate::runtime::{BootstrapError, ServerBootstrapPlan};
use crate::util::has_valid_ip_address;

pub const DEFAULT_MAIN_CONFIG: &str = "[Server]\nserver.ip=127.0.0.1\nserver.port=37120\nserver.private.port=37119\nserver.class.path=roseau::server::ServerHandler\n\n[Database]\ntype=mysql\nhostname=127.0.0.1\nport=3306\nusername=user\npassword=\ndatabase=roseau\noptions=\npath=roseau.sqlite\n\n[Logging]\nlog.errors=true\nlog.output=true\nlog.connections=true\nlog.packets=true\n";
pub const DEFAULT_HOTEL_CONFIG: &str = "[Register]\nuser.name.chars=1234567890qwertyuiopasdfghjklzxcvbnm-=?!@:.,\nuser.default.credits=100\nmessenger.greeting=I'm a new user!\n\n[Scheduler]\ncredits.every.x.secs=600\ncredits.every.x.amount=10\n\n[Bot]\nbot.response.delay=1500\n\n[Player]\ncarry.drink.time=180\ncarry.drink.interval=12\n\ntalking.lookat.distance=30\ntalking.lookat.reset=6\n\nafk.room.kick=1800\n\n[Debug]\ndebug.enable=true\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauBootstrap {
    main_config_path: PathBuf,
    hotel_config_path: PathBuf,
}

impl RoseauBootstrap {
    pub fn new(
        main_config_path: impl Into<PathBuf>,
        hotel_config_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            main_config_path: main_config_path.into(),
            hotel_config_path: hotel_config_path.into(),
        }
    }

    pub fn default_paths() -> Self {
        Self::new("roseau.properties", "habbohotel.properties")
    }

    pub fn main_config_path(&self) -> &Path {
        &self.main_config_path
    }

    pub fn hotel_config_path(&self) -> &Path {
        &self.hotel_config_path
    }

    pub fn ensure_config_files(&self) -> Result<[bool; 2], BootstrapError> {
        let main_created = write_if_missing(&self.main_config_path, DEFAULT_MAIN_CONFIG)?;
        let hotel_created = write_if_missing(&self.hotel_config_path, DEFAULT_HOTEL_CONFIG)?;
        Ok([main_created, hotel_created])
    }

    pub fn load_main_config(&self) -> Result<Config, BootstrapError> {
        Ok(Config::load(&self.main_config_path)?)
    }

    pub fn load_hotel_config(&self) -> Result<Config, BootstrapError> {
        Ok(Config::load(&self.hotel_config_path)?)
    }

    pub fn server_plan(
        &self,
        main_config: &Config,
        public_room_ids: impl IntoIterator<Item = i32>,
    ) -> Result<ServerBootstrapPlan, BootstrapError> {
        let raw_config_ip = main_config.required("Server", "server.ip")?.to_owned();
        let bind_ip = if has_valid_ip_address(&raw_config_ip) {
            raw_config_ip.clone()
        } else {
            "0.0.0.0".to_owned()
        };
        let server_port = parse_port(main_config, "server.port")?;
        let private_server_port = parse_port(main_config, "server.private.port")?;
        let server_class_path = main_config
            .required("Server", "server.class.path")?
            .to_owned();
        let database_engine = main_config.parse_value::<DatabaseEngine>("Database", "type")?;

        let mut ports = vec![server_port, private_server_port];
        let mut public_room_ports = Vec::new();
        for room_id in public_room_ids {
            let room_offset =
                u16::try_from(room_id).map_err(|_| BootstrapError::RoomPortOutOfRange {
                    base_port: server_port,
                    room_id,
                })?;
            let port =
                server_port
                    .checked_add(room_offset)
                    .ok_or(BootstrapError::RoomPortOutOfRange {
                        base_port: server_port,
                        room_id,
                    })?;
            ports.push(port);
            public_room_ports.push((room_id, port));
        }

        Ok(ServerBootstrapPlan::new(
            bind_ip,
            raw_config_ip,
            server_port,
            private_server_port,
            server_class_path,
            database_engine,
            ports,
            public_room_ports,
        ))
    }
}

fn write_if_missing(path: &Path, contents: &str) -> Result<bool, BootstrapError> {
    if path.is_file() {
        return Ok(false);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;
    Ok(true)
}

fn parse_port(config: &Config, key: &'static str) -> Result<u16, BootstrapError> {
    let value = config.parse_value::<i32>("Server", key)?;
    u16::try_from(value).map_err(|_| BootstrapError::PortOutOfRange { key, value })
}

#[cfg(test)]
#[path = "roseau_bootstrap_tests.rs"]
mod tests;
