pub mod catalogue_command_queries;
pub mod inventory_command_queries;
pub mod item_command_queries;
#[cfg(test)]
mod item_command_queries_tests;
pub mod messenger_command_queries;
pub mod navigator_command_queries;
pub mod player_command_queries;
pub mod room_command_queries;
#[cfg(test)]
mod room_command_queries_tests;

pub use catalogue_command_queries::CatalogueCommandQueries;
pub use inventory_command_queries::InventoryCommandQueries;
pub use item_command_queries::ItemCommandQueries;
pub use messenger_command_queries::MessengerCommandQueries;
pub use navigator_command_queries::NavigatorCommandQueries;
pub use player_command_queries::PlayerCommandQueries;
pub use room_command_queries::RoomCommandQueries;
