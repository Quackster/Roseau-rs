use crate::game::item::Item;
use crate::protocol::{NettyResponse, SerializableObject};
use crate::util::round_to_two_places;

impl SerializableObject for Item {
    fn serialise(&self, response: &mut NettyResponse) {
        let behaviour = self.definition.behaviour();
        if behaviour.is_invisible() {
            return;
        }

        if behaviour.is_passive_object() {
            response.append_new_argument(self.id);
            response.append_argument(self.definition.sprite());
            response.append_argument(self.position.x());
            response.append_argument(self.position.y());
            response.append_argument(self.position.z() as i32);
            response.append_argument(self.position.rotation());
            return;
        }

        if behaviour.is_on_floor() {
            response.append_new_argument(self.padding());
            response.append(self.id);
            response.append_argument_with(self.definition.sprite(), ',');
            response.append_argument(self.position.x());
            response.append_argument(self.position.y());
            response.append_argument(self.definition.length());
            response.append_argument(self.definition.width());
            response.append_argument(self.position.rotation());
            response.append_argument(round_to_two_places(self.position.z()));
            response.append_argument(self.definition.color());
            response.append_argument_with(self.definition.name(), '/');
            response.append_argument_with(self.definition.description(), '/');

            if self.target_teleporter_id > 0 {
                response.append_argument_with("extr=", '/');
                response.append_argument_with(self.target_teleporter_id, '/');
            }

            if self.definition.sprite() == "fireplace_polyfon"
                || (self.custom_data.is_some() && !self.definition.data_class().is_empty())
            {
                response.append_argument_with(self.definition.data_class(), '/');
                response.append_argument_with(self.custom_data.as_deref().unwrap_or_default(), '/');
            }

            return;
        }

        if behaviour.is_on_wall() {
            response.append(self.id);
            response.append_argument_with(self.definition.sprite(), ';');
            response.append_argument_with("Alex", ';');
            response.append_argument_with(self.wall_position.as_deref().unwrap_or_default(), ';');
            response.append_new_argument(self.custom_data.as_deref().unwrap_or_default());
        }
    }
}
