use super::*;

#[test]
fn composes_height_map_packet() {
    let mut response = HeightMap::new("000\r111").compose();

    assert_eq!(response.get(), "#HEIGHTMAP\r000\r111##");
}
