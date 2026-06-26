use crate::messages::outgoing::StripInfo;

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryCommandExecution {
    Refreshed { strip_info: StripInfo },
    Empty,
}

impl InventoryCommandExecution {
    pub fn strip_info(&self) -> Option<&StripInfo> {
        match self {
            Self::Refreshed { strip_info } => Some(strip_info),
            Self::Empty => None,
        }
    }
}
