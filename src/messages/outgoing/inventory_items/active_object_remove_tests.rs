use super::*;

#[test]
fn composes_active_object_remove_packet() {
    let mut response = ActiveObjectRemove::new("i:", 7).compose();

    assert_eq!(response.get(), "#ACTIVEOBJECT_REMOVE\ri:7##");
}
