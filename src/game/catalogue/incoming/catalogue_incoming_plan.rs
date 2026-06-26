use crate::dao::{CatalogueDao, DaoError, InventoryDao, ItemDao, PlayerDao};
use crate::game::catalogue::{
    CatalogueManager, CatalogueOrderInfoPlan, CataloguePurchaseExecution,
    CataloguePurchaseExecutor, CataloguePurchaseRequest, CatalogueTicketPurchaseExecution,
    CatalogueTicketPurchaseExecutor, CatalogueTicketPurchaseRequest,
};
use crate::game::player::PlayerDetails;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, PartialEq)]
pub enum CatalogueIncomingOutcome {
    OrderInfo(CatalogueOrderInfoPlan),
    Purchase(CataloguePurchaseExecution),
    TicketPurchase(CatalogueTicketPurchaseExecution),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CatalogueIncomingPlan;

impl CatalogueIncomingPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn plan(
        effect: &IncomingExecutionEffect,
        catalogue_manager: &CatalogueManager,
        catalogue_dao: &dyn CatalogueDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        player_dao: &dyn PlayerDao,
        buyer: &PlayerDetails,
    ) -> Result<Vec<CatalogueIncomingOutcome>, DaoError> {
        match effect {
            IncomingExecutionEffect::GetOrderInfo { call_id } => Ok(
                CatalogueOrderInfoPlan::resolve(catalogue_manager, call_id, Some(buyer.username()))
                    .map(CatalogueIncomingOutcome::OrderInfo)
                    .into_iter()
                    .collect(),
            ),
            IncomingExecutionEffect::Purchase { call_id } => {
                let ticket_execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
                    player_dao,
                    CatalogueTicketPurchaseRequest::new(call_id, buyer),
                )?;

                if ticket_execution != CatalogueTicketPurchaseExecution::Ignored {
                    return Ok(vec![CatalogueIncomingOutcome::TicketPurchase(
                        ticket_execution,
                    )]);
                }

                Ok(vec![CatalogueIncomingOutcome::Purchase(
                    CataloguePurchaseExecutor::purchase(
                        catalogue_dao,
                        inventory_dao,
                        item_dao,
                        player_dao,
                        CataloguePurchaseRequest::new(call_id, buyer),
                    )?,
                )])
            }
            _ => Ok(Vec::new()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        catalogue_manager: &CatalogueManager,
        catalogue_dao: &dyn CatalogueDao,
        inventory_dao: &dyn InventoryDao,
        item_dao: &dyn ItemDao,
        player_dao: &dyn PlayerDao,
        buyer: &PlayerDetails,
    ) -> Result<Vec<CatalogueIncomingOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(
                effect,
                catalogue_manager,
                catalogue_dao,
                inventory_dao,
                item_dao,
                player_dao,
                buyer,
            )?);
        }

        Ok(outcomes)
    }
}

#[cfg(test)]
#[path = "catalogue_incoming_plan_tests.rs"]
mod tests;
