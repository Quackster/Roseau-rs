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
#[path = "entity_type_tests.rs"]
mod tests;
