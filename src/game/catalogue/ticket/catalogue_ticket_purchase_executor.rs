use crate::dao::{DaoError, PlayerDao};
use crate::game::catalogue::{CatalogueTicketPurchaseOutcome, CatalogueTicketPurchasePlan};
use crate::game::player::PlayerDetails;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CatalogueTicketPurchaseExecutor;

impl CatalogueTicketPurchaseExecutor {
    pub fn purchase_tickets(
        player_dao: &dyn PlayerDao,
        request: CatalogueTicketPurchaseRequest<'_>,
    ) -> Result<CatalogueTicketPurchaseExecution, DaoError> {
        if !request.call_id.contains("hyppy") {
            return Ok(CatalogueTicketPurchaseExecution::Ignored);
        }

        let Some(plan) =
            CatalogueTicketPurchasePlan::resolve(request.call_id, request.buyer.credits())
        else {
            return Ok(CatalogueTicketPurchaseExecution::Rejected(
                CatalogueTicketPurchaseOutcome::InsufficientCredits,
            ));
        };

        let Some(target) = player_dao.details_by_username(plan.target_username())? else {
            return Ok(CatalogueTicketPurchaseExecution::Rejected(
                CatalogueTicketPurchaseOutcome::MissingTarget {
                    target_username: plan.target_username().to_owned(),
                },
            ));
        };

        if target.id() == request.buyer.id() {
            let mut buyer = request.buyer.clone();
            buyer.set_tickets(buyer.tickets() + plan.credited_tickets());
            buyer.set_credits(buyer.credits() - plan.charged_credits());
            player_dao.update_player(&buyer)?;

            return Ok(CatalogueTicketPurchaseExecution::Purchased {
                buyer,
                target: None,
                outcome: CatalogueTicketPurchaseOutcome::BoughtForSelf,
            });
        }

        let mut updated_target = target;
        updated_target.set_tickets(updated_target.tickets() + plan.credited_tickets());
        player_dao.update_player(&updated_target)?;

        let mut buyer = request.buyer.clone();
        buyer.set_credits(buyer.credits() - plan.charged_credits());
        player_dao.update_player(&buyer)?;

        Ok(CatalogueTicketPurchaseExecution::Purchased {
            buyer,
            target: Some(updated_target),
            outcome: CatalogueTicketPurchaseOutcome::BoughtForOther {
                buyer_username: request.buyer.username().to_owned(),
                target_username: plan.target_username().to_owned(),
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogueTicketPurchaseRequest<'a> {
    pub call_id: &'a str,
    pub buyer: &'a PlayerDetails,
}

impl<'a> CatalogueTicketPurchaseRequest<'a> {
    pub fn new(call_id: &'a str, buyer: &'a PlayerDetails) -> Self {
        Self { call_id, buyer }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogueTicketPurchaseExecution {
    Purchased {
        buyer: PlayerDetails,
        target: Option<PlayerDetails>,
        outcome: CatalogueTicketPurchaseOutcome,
    },
    Rejected(CatalogueTicketPurchaseOutcome),
    Ignored,
}

impl CatalogueTicketPurchaseExecution {
    pub fn outcome(&self) -> Option<&CatalogueTicketPurchaseOutcome> {
        match self {
            Self::Purchased { outcome, .. } | Self::Rejected(outcome) => Some(outcome),
            Self::Ignored => None,
        }
    }
}

#[cfg(test)]
#[path = "catalogue_ticket_purchase_executor_tests.rs"]
mod tests;
