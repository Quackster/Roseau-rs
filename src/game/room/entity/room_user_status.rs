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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_status_keeps_duration_and_ticks() {
        let mut status = RoomUserStatus::new("talk", "", false, 5);

        assert_eq!(status.status(), "talk");
        assert_eq!(status.key(), "talk");
        assert_eq!(status.value(), "");
        assert!(!status.is_infinite());
        assert_eq!(status.duration(), 5);

        status.tick();
        assert_eq!(status.duration(), 4);
    }

    #[test]
    fn infinite_status_uses_java_negative_duration() {
        let mut status = RoomUserStatus::new("flatctrl", "1", true, 99);

        assert!(status.is_infinite());
        assert_eq!(status.duration(), -1);

        status.tick();
        assert_eq!(status.duration(), -2);
    }
}
