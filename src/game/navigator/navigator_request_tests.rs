use super::navigator_request::*;

#[test]
fn exposes_java_navigator_request_variants() {
    let requests = [
        NavigatorRequest::PrivateRooms,
        NavigatorRequest::PopularRooms,
        NavigatorRequest::SearchRooms,
    ];

    assert_eq!(requests.len(), 3);
}
