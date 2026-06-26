pub mod aggregate;
pub mod catalogue;
pub mod inventory;
pub mod item;
pub mod messenger;
pub mod navigator;
pub mod player;
pub mod room;

pub use aggregate::{in_memory_dao, InMemoryDao};
pub use catalogue::{in_memory_catalogue_dao, InMemoryCatalogueDao};
pub use inventory::{in_memory_inventory_dao, InMemoryInventoryDao};
pub use item::{in_memory_item_dao, InMemoryItemDao};
pub use messenger::{in_memory_messenger_dao, InMemoryMessengerDao};
pub use navigator::{in_memory_navigator_dao, InMemoryNavigatorDao};
pub use player::{in_memory_player_dao, InMemoryPlayerDao};
pub use room::{in_memory_room_dao, InMemoryRoomDao};
