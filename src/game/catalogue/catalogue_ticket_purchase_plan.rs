use crate::util::filter_input;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueTicketPurchasePlan {
    target_username: String,
    credited_tickets: i32,
    charged_credits: i32,
}

impl CatalogueTicketPurchasePlan {
    pub const MINIMUM_CREDITS: i32 = 10;
    pub const CHARGED_CREDITS: i32 = 5;
    pub const CREDITED_TICKETS: i32 = 10;

    pub fn resolve(call_id: &str, available_credits: i32) -> Option<Self> {
        if !call_id.contains("hyppy") || available_credits < Self::MINIMUM_CREDITS {
            return None;
        }

        let target_username = filter_input(call_id.split(' ').nth(2)?);
        if target_username.is_empty() {
            return None;
        }

        Some(Self {
            target_username,
            credited_tickets: Self::CREDITED_TICKETS,
            charged_credits: Self::CHARGED_CREDITS,
        })
    }

    pub fn target_username(&self) -> &str {
        &self.target_username
    }

    pub fn credited_tickets(&self) -> i32 {
        self.credited_tickets
    }

    pub fn charged_credits(&self) -> i32 {
        self.charged_credits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_ticket_purchase_calls() {
        assert_eq!(CatalogueTicketPurchasePlan::resolve("chair", 50), None);
    }

    #[test]
    fn rejects_ticket_purchase_below_java_credit_threshold() {
        assert_eq!(
            CatalogueTicketPurchasePlan::resolve("x hyppy alice", 9),
            None
        );
    }

    #[test]
    fn resolves_ticket_purchase_target_and_amounts() {
        let plan = CatalogueTicketPurchasePlan::resolve("x hyppy alice", 10).unwrap();

        assert_eq!(plan.target_username(), "alice");
        assert_eq!(plan.credited_tickets(), 10);
        assert_eq!(plan.charged_credits(), 5);
    }

    #[test]
    fn filters_ticket_purchase_target_like_java_input() {
        let plan = CatalogueTicketPurchasePlan::resolve("x hyppy al\nice", 10).unwrap();

        assert_eq!(plan.target_username(), "al ice");
    }
}
