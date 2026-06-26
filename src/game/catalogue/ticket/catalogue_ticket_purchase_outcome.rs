use crate::messages::outgoing::SystemBroadcast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogueTicketPurchaseOutcome {
    InsufficientCredits,
    MissingTarget {
        target_username: String,
    },
    BoughtForSelf,
    BoughtForOther {
        buyer_username: String,
        target_username: String,
    },
}

impl CatalogueTicketPurchaseOutcome {
    pub const INSUFFICIENT_CREDITS_MESSAGE: &'static str =
        "Sorry, but you do not have enough Credits to purchase this.";

    pub fn buyer_alert(&self) -> SystemBroadcast {
        match self {
            Self::InsufficientCredits => SystemBroadcast::new(Self::INSUFFICIENT_CREDITS_MESSAGE),
            Self::MissingTarget { target_username } => {
                SystemBroadcast::new(format!("The player '{target_username}' cannot be found."))
            }
            Self::BoughtForSelf => SystemBroadcast::new("You have bought 10 game tickets!"),
            Self::BoughtForOther {
                target_username, ..
            } => SystemBroadcast::new(format!(
                "You have bought 10 game tickets for {target_username}"
            )),
        }
    }

    pub fn target_alert(&self) -> Option<SystemBroadcast> {
        match self {
            Self::BoughtForOther { buyer_username, .. } => Some(SystemBroadcast::new(format!(
                "{buyer_username} has bought 10 game tickets for you!"
            ))),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "catalogue_ticket_purchase_outcome_tests.rs"]
mod tests;
