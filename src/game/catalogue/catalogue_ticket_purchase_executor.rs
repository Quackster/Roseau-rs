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
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryPlayerDao;
    use crate::dao::{CreatePlayer, PlayerDao};

    fn create_player(username: &str, credits: i32) -> CreatePlayer {
        CreatePlayer::new(
            username,
            "secret",
            format!("{username}@example.test"),
            "hello",
            "hd=100",
            credits,
            "Male",
            "08.08.1997",
        )
    }

    fn seed_player(dao: &InMemoryPlayerDao, username: &str, credits: i32, tickets: i32) {
        dao.create_player(&create_player(username, credits))
            .unwrap();
        let mut details = dao.details_by_username(username).unwrap().unwrap();
        details.set_tickets(tickets);
        dao.update_player(&details).unwrap();
    }

    fn seeded_dao() -> InMemoryPlayerDao {
        let dao = InMemoryPlayerDao::new();
        seed_player(&dao, "alice", 20, 1);
        seed_player(&dao, "bob", 5, 3);
        dao
    }

    #[test]
    fn ignores_non_ticket_purchase_calls() {
        let dao = seeded_dao();
        let buyer = dao.details_by_username("alice").unwrap().unwrap();

        let execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
            &dao,
            CatalogueTicketPurchaseRequest::new("chair", &buyer),
        )
        .unwrap();

        assert_eq!(execution, CatalogueTicketPurchaseExecution::Ignored);
    }

    #[test]
    fn rejects_ticket_purchase_when_buyer_has_too_few_credits() {
        let dao = seeded_dao();
        let buyer = dao.details_by_username("bob").unwrap().unwrap();

        let execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
            &dao,
            CatalogueTicketPurchaseRequest::new("x hyppy alice", &buyer),
        )
        .unwrap();

        assert_eq!(
            execution,
            CatalogueTicketPurchaseExecution::Rejected(
                CatalogueTicketPurchaseOutcome::InsufficientCredits
            )
        );
        assert_eq!(
            dao.details_by_username("bob").unwrap().unwrap().credits(),
            5
        );
    }

    #[test]
    fn rejects_ticket_purchase_when_target_is_missing() {
        let dao = seeded_dao();
        let buyer = dao.details_by_username("alice").unwrap().unwrap();

        let execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
            &dao,
            CatalogueTicketPurchaseRequest::new("x hyppy carol", &buyer),
        )
        .unwrap();

        assert_eq!(
            execution,
            CatalogueTicketPurchaseExecution::Rejected(
                CatalogueTicketPurchaseOutcome::MissingTarget {
                    target_username: "carol".to_owned()
                }
            )
        );
    }

    #[test]
    fn buys_tickets_for_self_and_persists_combined_ticket_credit_update() {
        let dao = seeded_dao();
        let buyer = dao.details_by_username("alice").unwrap().unwrap();

        let execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
            &dao,
            CatalogueTicketPurchaseRequest::new("x hyppy alice", &buyer),
        )
        .unwrap();

        let CatalogueTicketPurchaseExecution::Purchased {
            buyer,
            target,
            outcome,
        } = execution
        else {
            panic!("expected ticket purchase");
        };
        let persisted = dao.details_by_username("alice").unwrap().unwrap();
        assert_eq!(outcome, CatalogueTicketPurchaseOutcome::BoughtForSelf);
        assert!(target.is_none());
        assert_eq!(buyer.credits(), 15);
        assert_eq!(buyer.tickets(), 11);
        assert_eq!(persisted.credits(), 15);
        assert_eq!(persisted.tickets(), 11);
    }

    #[test]
    fn buys_tickets_for_other_player_and_persists_both_players() {
        let dao = seeded_dao();
        let buyer = dao.details_by_username("alice").unwrap().unwrap();

        let execution = CatalogueTicketPurchaseExecutor::purchase_tickets(
            &dao,
            CatalogueTicketPurchaseRequest::new("x hyppy bob", &buyer),
        )
        .unwrap();

        let CatalogueTicketPurchaseExecution::Purchased {
            buyer,
            target: Some(target),
            outcome,
        } = execution
        else {
            panic!("expected cross-player ticket purchase");
        };
        let persisted_buyer = dao.details_by_username("alice").unwrap().unwrap();
        let persisted_target = dao.details_by_username("bob").unwrap().unwrap();
        assert_eq!(
            outcome,
            CatalogueTicketPurchaseOutcome::BoughtForOther {
                buyer_username: "alice".to_owned(),
                target_username: "bob".to_owned(),
            }
        );
        assert_eq!(buyer.credits(), 15);
        assert_eq!(target.tickets(), 13);
        assert_eq!(persisted_buyer.credits(), 15);
        assert_eq!(persisted_target.tickets(), 13);
    }
}
