use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatProperty {
    property: String,
    data: String,
}

impl FlatProperty {
    pub fn new(property: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            property: property.into(),
            data: data.into(),
        }
    }
}

impl OutgoingMessage for FlatProperty {
    fn write(&self, response: &mut NettyResponse) {
        response.init("FLATPROPERTY");
        response.append_new_argument(&self.property);
        response.append_part_argument(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_flat_property_packet() {
        let mut response = FlatProperty::new("wallpaper", "101").compose();

        assert_eq!(response.get(), "#FLATPROPERTY\rwallpaper/101##");
    }
}
