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
mod tests {
    use super::*;
    use crate::messages::OutgoingMessage;

    #[test]
    fn builds_insufficient_credit_alert() {
        let mut alert = CatalogueTicketPurchaseOutcome::InsufficientCredits
            .buyer_alert()
            .compose();

        assert_eq!(
            alert.get(),
            "#SYSTEMBROADCAST\rSorry, but you do not have enough Credits to purchase this.##"
        );
    }

    #[test]
    fn builds_missing_target_alert() {
        let mut alert = CatalogueTicketPurchaseOutcome::MissingTarget {
            target_username: "bob".to_owned(),
        }
        .buyer_alert()
        .compose();

        assert_eq!(
            alert.get(),
            "#SYSTEMBROADCAST\rThe player 'bob' cannot be found.##"
        );
    }

    #[test]
    fn builds_self_purchase_alert() {
        let outcome = CatalogueTicketPurchaseOutcome::BoughtForSelf;

        assert_eq!(
            outcome.buyer_alert().compose().get(),
            "#SYSTEMBROADCAST\rYou have bought 10 game tickets!##"
        );
        assert!(outcome.target_alert().is_none());
    }

    #[test]
    fn builds_cross_player_purchase_alerts() {
        let outcome = CatalogueTicketPurchaseOutcome::BoughtForOther {
            buyer_username: "alice".to_owned(),
            target_username: "bob".to_owned(),
        };

        assert_eq!(
            outcome.buyer_alert().compose().get(),
            "#SYSTEMBROADCAST\rYou have bought 10 game tickets for bob##"
        );
        assert_eq!(
            outcome.target_alert().unwrap().compose().get(),
            "#SYSTEMBROADCAST\ralice has bought 10 game tickets for you!##"
        );
    }
}
