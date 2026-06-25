pub mod catalogue_deal;
#[cfg(test)]
mod catalogue_deal_tests;
pub mod catalogue_item;
#[cfg(test)]
mod catalogue_item_tests;
pub mod catalogue_manager;
#[cfg(test)]
mod catalogue_manager_tests;

pub use catalogue_deal::{CatalogueDeal, ResolveDealItemError};
pub use catalogue_item::CatalogueItem;
pub use catalogue_manager::CatalogueManager;
