use super::messenger_queries::*;

#[test]
fn builds_friend_and_request_reads() {
    let friends = MessengerQueries::friends(7);
    let requests = MessengerQueries::requests(7);

    assert_eq!(
        friends.sql(),
        "SELECT * FROM messenger_friendships WHERE sender = ? OR receiver = ?"
    );
    assert_eq!(
        friends.parameters(),
        &[SqlParameter::Integer(7), SqlParameter::Integer(7)]
    );
    assert_eq!(
        requests.sql(),
        "SELECT * FROM messenger_requests WHERE to_id = ?"
    );
    assert_eq!(requests.parameters(), &[SqlParameter::Integer(7)]);
}

#[test]
fn builds_bidirectional_request_and_friend_mutations() {
    let exists = MessengerQueries::request_exists(1, 2);
    let create_request = MessengerQueries::create_request(1, 2);
    let remove_request = MessengerQueries::remove_request(1, 2);
    let create_friend = MessengerQueries::create_friend(1, 2);
    let remove_friend = MessengerQueries::remove_friend(1, 2);

    assert_eq!(
        exists.sql(),
        "SELECT * FROM messenger_requests WHERE (to_id = ? AND from_id = ?) OR (from_id = ? AND to_id = ?) LIMIT 1"
    );
    assert_eq!(
        exists.parameters(),
        &[
            SqlParameter::Integer(2),
            SqlParameter::Integer(1),
            SqlParameter::Integer(2),
            SqlParameter::Integer(1),
        ]
    );
    assert_eq!(
        create_request.sql(),
        "INSERT INTO messenger_requests (from_id, to_id) VALUES (?, ?)"
    );
    assert_eq!(
        remove_request.sql(),
        "DELETE FROM messenger_requests WHERE from_id = ? AND to_id = ?"
    );
    assert_eq!(
        create_friend.sql(),
        "INSERT INTO messenger_friendships (sender, receiver) VALUES (?, ?)"
    );
    assert_eq!(
        remove_friend.sql(),
        "DELETE FROM messenger_friendships WHERE (sender = ? AND receiver = ?) OR (receiver = ? AND sender = ?)"
    );
}

#[test]
fn builds_message_insert_read_and_update_queries() {
    let create = MessengerQueries::create_message(1, 2, 1234, "hello");
    let unread = MessengerQueries::unread_messages(2);
    let mark_read = MessengerQueries::mark_message_read(9);

    assert_eq!(
        create.sql(),
        "INSERT INTO messenger_messages (from_id, to_id, time_sent, message, unread) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(
        create.parameters(),
        &[
            SqlParameter::Integer(1),
            SqlParameter::Integer(2),
            SqlParameter::Long(1234),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Bool(true),
        ]
    );
    assert_eq!(
        unread.sql(),
        "SELECT * FROM messenger_messages WHERE to_id = ? AND unread = ?"
    );
    assert_eq!(
        mark_read.sql(),
        "UPDATE messenger_messages SET unread = ? WHERE id = ?"
    );
    assert_eq!(
        (
            MessengerQueries::friendship_table(),
            MessengerQueries::message_table(),
            MessengerQueries::request_table()
        ),
        (
            "messenger_friendships",
            "messenger_messages",
            "messenger_requests"
        )
    );
}
