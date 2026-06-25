use crate::dao::mysql::{InventoryQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryCommandQueries;

impl InventoryCommandQueries {
    pub fn read_plan(effect: &IncomingExecutionEffect, user_id: i32) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::RefreshInventory { .. } => {
                Some(InventoryQueries::inventory_items(user_id).read_plan())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

    #[test]
    fn maps_refresh_inventory_to_inventory_item_read() {
        let plan = InventoryCommandQueries::read_plan(
            &IncomingExecutionEffect::RefreshInventory {
                category: "new".to_owned(),
            },
            7,
        )
        .unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(
            plan.sql(),
            "SELECT * FROM items WHERE room_id = ? AND user_id = ?"
        );
        assert_eq!(
            plan.parameters(),
            &[SqlParameter::Integer(0), SqlParameter::Integer(7)]
        );
    }

    #[test]
    fn ignores_non_inventory_effects() {
        assert_eq!(
            InventoryCommandQueries::read_plan(&IncomingExecutionEffect::RetrieveUserInfo, 7),
            None
        );
    }
}
