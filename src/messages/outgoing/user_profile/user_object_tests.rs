use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerDetails;

impl SerializableObject for PlayerDetails {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_argument("alice");
    }
}

#[test]
fn composes_user_object_packet() {
    let mut response = UserObject::new(PlayerDetails).compose();

    assert_eq!(response.get(), "#USEROBJECT alice##");
}
