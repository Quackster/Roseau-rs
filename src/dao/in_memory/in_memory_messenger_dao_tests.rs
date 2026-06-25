use super::in_memory_messenger_dao::*;

#[test]
fn stores_requests_and_checks_bidirectionally() {
    let dao = InMemoryMessengerDao::new();

    assert!(dao.new_request(1, 2).unwrap());
    assert!(!dao.new_request(2, 1).unwrap());
    assert!(dao.request_exists(2, 1).unwrap());
    assert_eq!(dao.requests(2).unwrap()[0].user_id(), 1);

    dao.remove_request(1, 2).unwrap();
    assert!(!dao.request_exists(1, 2).unwrap());
}

#[test]
fn stores_friendships_bidirectionally() {
    let dao = InMemoryMessengerDao::new();

    assert!(dao.new_friend(1, 2).unwrap());
    assert!(!dao.new_friend(2, 1).unwrap());
    assert_eq!(dao.friends(1).unwrap()[0].user_id(), 2);
    assert_eq!(dao.friends(2).unwrap()[0].user_id(), 1);

    dao.remove_friend(1, 2).unwrap();
    assert!(dao.friends(1).unwrap().is_empty());
}

#[test]
fn stores_unread_messages_and_marks_read() {
    let dao = InMemoryMessengerDao::new().with_current_time(1234);

    let id = dao.new_message(1, 2, "hello").unwrap();
    dao.new_message(1, 3, "other").unwrap();

    let unread = dao.unread_messages(2).unwrap();
    assert_eq!(unread.len(), 1);
    assert_eq!(unread[0].id(), id);
    assert_eq!(unread[0].time_sent(), 1234);

    dao.mark_message_read(id).unwrap();
    assert!(dao.unread_messages(2).unwrap().is_empty());
}
