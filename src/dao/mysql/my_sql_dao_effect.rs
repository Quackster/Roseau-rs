#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MySqlDaoEffect {
    LogLine(String),
    ConnectStorage,
    ConstructPlayerDao,
    ConstructRoomDao,
    ConstructItemDao,
    ConstructCatalogueDao,
    ConstructInventoryDao,
    ConstructNavigatorDao,
    ConstructMessengerDao,
}
