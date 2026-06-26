#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageConnectionEffect {
    LoadDriverCrate {
        crate_name: &'static str,
    },
    OpenConnection {
        connection_url: String,
        username: Option<String>,
    },
}
