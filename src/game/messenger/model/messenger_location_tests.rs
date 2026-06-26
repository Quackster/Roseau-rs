use super::*;

#[test]
fn formats_java_messenger_locations() {
    assert_eq!(MessengerLocation::HotelView.display_text(), "On Hotel View");
    assert_eq!(
        MessengerLocation::PrivateRoom.display_text(),
        "In a user flat"
    );
    assert_eq!(
        MessengerLocation::PublicRoom("Cafe".to_owned()).display_text(),
        "Cafe"
    );
}
