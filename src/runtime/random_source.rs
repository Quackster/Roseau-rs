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
mod tests {
    use super::*;

    #[test]
    fn produces_repeatable_values_from_seed() {
        let mut left = RandomSource::seeded(42);
        let mut right = RandomSource::seeded(42);

        assert_eq!(left.next_u64(), right.next_u64());
        assert_eq!(left.next_i32(10), right.next_i32(10));
    }

    #[test]
    fn handles_invalid_bounds_and_fraction_range() {
        let mut random = RandomSource::seeded(1);

        assert_eq!(random.next_i32(0), None);
        let fraction = random.next_fraction();
        assert!((0.0..1.0).contains(&fraction));
    }
}
