use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeightMap {
    height_map: String,
}

impl HeightMap {
    pub fn new(height_map: impl Into<String>) -> Self {
        Self {
            height_map: height_map.into(),
        }
    }
}

impl OutgoingMessage for HeightMap {
    fn write(&self, response: &mut NettyResponse) {
        response.init("HEIGHTMAP");
        response.append_new_argument(&self.height_map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_height_map_packet() {
        let mut response = HeightMap::new("000\r111").compose();

        assert_eq!(response.get(), "#HEIGHTMAP\r000\r111##");
    }
}
