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
