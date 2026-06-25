use crate::logging::DateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomSource {
    state: u64,
}

impl RandomSource {
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    pub fn from_clock() -> Self {
        Self::seeded(DateTime::current_time_millis() as u64)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value.max(1);
        value
    }

    pub fn next_i32(&mut self, upper_bound: i32) -> Option<i32> {
        if upper_bound <= 0 {
            return None;
        }

        Some((self.next_u64() % upper_bound as u64) as i32)
    }

    pub fn next_fraction(&mut self) -> f32 {
        let value = self.next_u64() >> 40;
        value as f32 / 16_777_216.0
    }
}

#[cfg(test)]
#[path = "random_source_tests.rs"]
mod tests;
