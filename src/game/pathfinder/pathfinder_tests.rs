use super::*;

#[test]
fn finds_straight_path_excluding_start_and_including_end() {
    let path = make_path(
        Position::new(0, 0, 0.0),
        Position::new(2, 0, 0.0),
        4,
        4,
        |_current, _next, _final_move| true,
    );

    assert_eq!(
        path,
        vec![Position::new(1, 0, 0.0), Position::new(2, 0, 0.0)]
    );
}

#[test]
fn passes_final_move_to_step_validator() {
    let mut final_steps = Vec::new();
    let path = make_path(
        Position::new(0, 0, 0.0),
        Position::new(1, 0, 0.0),
        3,
        3,
        |_current, next, final_move| {
            final_steps.push((next, final_move));
            true
        },
    );

    assert_eq!(path, vec![Position::new(1, 0, 0.0)]);
    assert!(final_steps.contains(&(Position::new(1, 0, 0.0), true)));
}

#[test]
fn returns_empty_path_when_blocked() {
    let path = make_path(
        Position::new(0, 0, 0.0),
        Position::new(2, 0, 0.0),
        3,
        1,
        |_current, _next, _final_move| false,
    );

    assert!(path.is_empty());
}
