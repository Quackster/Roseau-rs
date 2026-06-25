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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::OutgoingMessage;

    #[test]
    fn maps_applied_decoration_to_flat_property_packet() {
        let outcome = RoomDecorationOutcome::applied("wallpaper", "101");
        let mut response = outcome.flat_property_packet().unwrap().compose();

        assert_eq!(response.get(), "#FLATPROPERTY\rwallpaper/101##");
    }

    #[test]
    fn ignored_decoration_has_no_packet() {
        assert!(RoomDecorationOutcome::Ignored
            .flat_property_packet()
            .is_none());
    }
}
