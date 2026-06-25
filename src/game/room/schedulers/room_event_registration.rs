use crate::game::room::RoomEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomEventRegistration {
    ClubMassivaDisco,
    HabboLido,
    BotMoveRoom,
    UserStatus,
    Unknown(String),
}

impl RoomEventRegistration {
    pub fn from_name(event_name: &str) -> Self {
        match event_name {
            "club_massiva_disco" => Self::ClubMassivaDisco,
            "habbo_lido" => Self::HabboLido,
            "bot_move_room" => Self::BotMoveRoom,
            "user_status" => Self::UserStatus,
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn from_effect(effect: &RoomEffect) -> Option<Self> {
        match effect {
            RoomEffect::RegisterEvent { event_name } => Some(Self::from_name(event_name)),
            _ => None,
        }
    }

    pub fn collect(effects: &[RoomEffect]) -> Vec<Self> {
        effects.iter().filter_map(Self::from_effect).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_java_room_event_names_to_rust_registrations() {
        assert_eq!(
            RoomEventRegistration::from_name("club_massiva_disco"),
            RoomEventRegistration::ClubMassivaDisco
        );
        assert_eq!(
            RoomEventRegistration::from_name("habbo_lido"),
            RoomEventRegistration::HabboLido
        );
        assert_eq!(
            RoomEventRegistration::from_name("bot_move_room"),
            RoomEventRegistration::BotMoveRoom
        );
        assert_eq!(
            RoomEventRegistration::from_name("user_status"),
            RoomEventRegistration::UserStatus
        );
        assert_eq!(
            RoomEventRegistration::from_name("custom"),
            RoomEventRegistration::Unknown("custom".to_owned())
        );
    }

    #[test]
    fn collects_registrations_from_room_effects() {
        let registrations = RoomEventRegistration::collect(&[
            RoomEffect::ScheduleEventTicks,
            RoomEffect::RegisterEvent {
                event_name: "habbo_lido".to_owned(),
            },
            RoomEffect::RegisterEvent {
                event_name: "user_status".to_owned(),
            },
        ]);

        assert_eq!(
            registrations,
            vec![
                RoomEventRegistration::HabboLido,
                RoomEventRegistration::UserStatus,
            ]
        );
    }
}
