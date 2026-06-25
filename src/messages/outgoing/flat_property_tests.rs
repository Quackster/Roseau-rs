use super::flat_property::*;

#[test]
fn composes_flat_property_packet() {
    let mut response = FlatProperty::new("wallpaper", "101").compose();

    assert_eq!(response.get(), "#FLATPROPERTY\rwallpaper/101##");
}
