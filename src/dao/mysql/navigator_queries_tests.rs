use super::navigator_queries::*;

#[test]
fn builds_private_room_name_search() {
    let query = NavigatorQueries::rooms_by_like_name("lobby");

    assert_eq!(
        query.sql(),
        "SELECT * FROM rooms WHERE name LIKE ? AND room_type = ?"
    );
    assert_eq!(
        query.parameters(),
        &[
            SqlParameter::Text("%lobby%".to_owned()),
            SqlParameter::Integer(0),
        ]
    );
    assert_eq!(NavigatorQueries::room_table(), "rooms");
}
