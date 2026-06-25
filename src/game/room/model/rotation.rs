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
#[path = "rotation_tests.rs"]
mod tests;
