use crate::dao::mysql::DatabaseEngine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauLifecycleStep {
    EnsureConfigFiles,
    StartLogging,
    LoadRuntime,
    ValidateDatabaseEngine {
        engine: DatabaseEngine,
    },
    OpenDatabase {
        engine: DatabaseEngine,
    },
    LoadGame,
    SetupServer,
    ConstructServerHandler {
        class_path: String,
        bind_ip: String,
        ports: Vec<u16>,
    },
    ResolveConfiguredHost {
        host: String,
    },
    Listen {
        bind_ip: String,
        port: u16,
    },
}
