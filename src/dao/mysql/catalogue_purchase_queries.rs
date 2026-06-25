use crate::dao::mysql::{InventoryQueries, PlayerQueries, SqlExecutionPlan};
use crate::game::catalogue::{CataloguePurchasePlan, CatalogueTicketPurchasePlan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CataloguePurchaseQueries;

impl CataloguePurchaseQueries {
    pub fn plan(
        purchase: &CataloguePurchasePlan,
        buyer_id: i32,
        current_credits: i32,
    ) -> Option<Vec<SqlExecutionPlan>> {
        if purchase
            .items()
            .iter()
            .any(|item| item.is_teleporter_pair())
        {
            return None;
        }

        let mut plans = purchase
            .items()
            .iter()
            .map(|item| {
                InventoryQueries::create_item(item.definition_id(), buyer_id, item.extra_data())
                    .insert_returning_id_plan()
            })
            .collect::<Vec<_>>();

        plans.push(
            PlayerQueries::update_credits(buyer_id, current_credits - purchase.cost())
                .execute_plan(),
        );

        Some(plans)
    }

    pub fn teleporter_pair_insert_plans(
        purchase: &CataloguePurchasePlan,
        buyer_id: i32,
    ) -> Option<Vec<SqlExecutionPlan>> {
        let item = purchase.items().first()?;
        if purchase.items().len() != 1 || !item.is_teleporter_pair() {
            return None;
        }

        Some(vec![
            InventoryQueries::create_item(item.definition_id(), buyer_id, "")
                .insert_returning_id_plan(),
            InventoryQueries::create_item(item.definition_id(), buyer_id, "")
                .insert_returning_id_plan(),
        ])
    }

    pub fn teleporter_pair_link_plans(
        first_item_id: i32,
        second_item_id: i32,
        purchase: &CataloguePurchasePlan,
        buyer_id: i32,
        current_credits: i32,
    ) -> Vec<SqlExecutionPlan> {
        vec![
            InventoryQueries::update_extra_data(first_item_id, second_item_id.to_string())
                .execute_plan(),
            InventoryQueries::update_extra_data(second_item_id, first_item_id.to_string())
                .execute_plan(),
            PlayerQueries::update_credits(buyer_id, current_credits - purchase.cost())
                .execute_plan(),
        ]
    }

    pub fn ticket_purchase_plans(
        purchase: &CatalogueTicketPurchasePlan,
        buyer_id: i32,
        current_buyer_credits: i32,
        target_user_id: i32,
        current_target_tickets: i32,
    ) -> Vec<SqlExecutionPlan> {
        vec![
            PlayerQueries::update_tickets(
                target_user_id,
                current_target_tickets + purchase.credited_tickets(),
            )
            .execute_plan(),
            PlayerQueries::update_credits(
                buyer_id,
                current_buyer_credits - purchase.charged_credits(),
            )
            .execute_plan(),
        ]
    }
}
