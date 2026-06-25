use crate::dao::{DaoError, InventoryDao};
use crate::game::inventory::{Inventory, InventoryCommandExecution, InventoryRefresh};
use crate::game::item::Item;
use crate::messages::outgoing::{StripInfo, StripItem, StripItemKind};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryCommandExecutor;

impl InventoryCommandExecutor {
    pub fn refresh_inventory(
        inventory_dao: &dyn InventoryDao,
        user_id: i32,
        mode: &str,
    ) -> Result<InventoryCommandExecution, DaoError> {
        let mut items = inventory_dao.inventory_items(user_id)?;
        items.sort_by_key(Item::id);
        let mut inventory = Inventory::with_items(items);

        match inventory.refresh(mode) {
            InventoryRefresh::Page { items, .. } => {
                let strip_items = items.iter().map(strip_item_from_item);
                Ok(InventoryCommandExecution::Refreshed {
                    strip_info: StripInfo::new(strip_items),
                })
            }
            InventoryRefresh::Empty => Ok(InventoryCommandExecution::Empty),
        }
    }
}

fn strip_item_from_item(item: &Item) -> StripItem {
    let definition = item.definition();
    let behaviour = definition.behaviour();
    let custom_data = item.custom_data().unwrap_or_default();
    let kind = if behaviour.is_stuff() {
        StripItemKind::Stuff {
            length: definition.length(),
            width: definition.width(),
            color: definition.color().to_owned(),
        }
    } else if behaviour.is_item() {
        StripItemKind::Item {
            post_it: behaviour.is_post_it(),
        }
    } else {
        StripItemKind::Other
    };

    StripItem::new(
        item.id(),
        definition.sprite(),
        definition.name(),
        custom_data,
        kind,
    )
}
