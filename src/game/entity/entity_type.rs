#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Pet,
    Bot,
}

impl EntityType {
    pub fn rust_type_name(self) -> &'static str {
        match self {
            Self::Player => "Player",
            Self::Pet => "Entity",
            Self::Bot => "Entity",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_java_entity_type_class_mapping_as_names() {
        assert_eq!(EntityType::Player.rust_type_name(), "Player");
        assert_eq!(EntityType::Pet.rust_type_name(), "Entity");
        assert_eq!(EntityType::Bot.rust_type_name(), "Entity");
    }
}
