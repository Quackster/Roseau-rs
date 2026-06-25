use crate::messages::outgoing::FlatProperty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomDecorationOutcome {
    Applied { decoration: String, data: String },
    Ignored,
}

impl RoomDecorationOutcome {
    pub fn applied(decoration: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Applied {
            decoration: decoration.into(),
            data: data.into(),
        }
    }

    pub fn flat_property_packet(&self) -> Option<FlatProperty> {
        match self {
            Self::Applied { decoration, data } => Some(FlatProperty::new(decoration, data)),
            Self::Ignored => None,
        }
    }
}
