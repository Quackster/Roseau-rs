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
