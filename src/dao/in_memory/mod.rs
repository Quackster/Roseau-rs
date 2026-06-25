pub mod in_memory_catalogue_dao;
#[cfg(test)]
mod in_memory_catalogue_dao_tests;
pub mod in_memory_dao;
#[cfg(test)]
mod in_memory_dao_tests;
pub mod in_memory_inventory_dao;
#[cfg(test)]
mod in_memory_inventory_dao_tests;
pub mod in_memory_item_dao;
#[cfg(test)]
mod in_memory_item_dao_tests;
pub mod in_memory_messenger_dao;
#[cfg(test)]
mod in_memory_messenger_dao_tests;
pub mod in_memory_navigator_dao;
#[cfg(test)]
mod in_memory_navigator_dao_tests;
pub mod in_memory_player_dao;
#[cfg(test)]
mod in_memory_player_dao_tests;
pub mod in_memory_room_dao;
#[cfg(test)]
mod in_memory_room_dao_tests;

pub use in_memory_catalogue_dao::InMemoryCatalogueDao;
pub use in_memory_dao::InMemoryDao;
pub use in_memory_inventory_dao::InMemoryInventoryDao;
pub use in_memory_item_dao::InMemoryItemDao;
pub use in_memory_messenger_dao::InMemoryMessengerDao;
pub use in_memory_navigator_dao::InMemoryNavigatorDao;
pub use in_memory_player_dao::InMemoryPlayerDao;
pub use in_memory_room_dao::InMemoryRoomDao;
