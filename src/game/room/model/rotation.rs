pub fn calculate_direction(x: i32, y: i32, to_x: i32, to_y: i32) -> i8 {
    if x > to_x && y > to_y {
        7
    } else if x < to_x && y < to_y {
        3
    } else if x > to_x && y < to_y {
        5
    } else if x < to_x && y > to_y {
        1
    } else if x > to_x {
        6
    } else if x < to_x {
        2
    } else if y < to_y {
        4
    } else if y > to_y {
        0
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_java_room_direction_values() {
        assert_eq!(calculate_direction(5, 5, 4, 4), 7);
        assert_eq!(calculate_direction(5, 5, 6, 6), 3);
        assert_eq!(calculate_direction(5, 5, 4, 6), 5);
        assert_eq!(calculate_direction(5, 5, 6, 4), 1);
        assert_eq!(calculate_direction(5, 5, 4, 5), 6);
        assert_eq!(calculate_direction(5, 5, 6, 5), 2);
        assert_eq!(calculate_direction(5, 5, 5, 6), 4);
        assert_eq!(calculate_direction(5, 5, 5, 4), 0);
    }
}
