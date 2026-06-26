use std::collections::HashMap;

use crate::dao::DaoError;
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};

pub trait CatalogueDao {
    fn buyable_items(&self) -> Result<HashMap<String, CatalogueItem>, DaoError>;
    fn item_deals(&self) -> Result<HashMap<String, CatalogueDeal>, DaoError>;
}
