#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomUserStatus {
    key: String,
    value: String,
    infinite: bool,
    duration: i64,
}

impl RoomUserStatus {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        infinite: bool,
        duration: i64,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            infinite,
            duration: if infinite { -1 } else { duration },
        }
    }

    pub fn status(&self) -> &str {
        &self.key
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_infinite(&self) -> bool {
        self.infinite
    }

    pub fn duration(&self) -> i64 {
        self.duration
    }

    pub fn tick(&mut self) {
        self.duration -= 1;
    }
}
