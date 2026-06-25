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
