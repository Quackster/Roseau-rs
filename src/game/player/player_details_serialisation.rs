use crate::game::player::PlayerDetails;
use crate::protocol::{NettyResponse, SerializableObject};

impl SerializableObject for PlayerDetails {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_kv_argument("name", &self.username);
        response.append_kv_argument("figure", &self.figure);
        response.append_kv_argument("email", &self.email);
        response.append_kv_argument("birthday", &self.birthday);
        response.append_kv_argument("phonenumber", "+44");
        response.append_kv_argument("customData", &self.mission);
        response.append_kv_argument("has_read_agreement", "1");
        response.append_kv_argument("sex", &self.sex);
        response.append_kv_argument("country", &self.country);
        response.append_kv_argument("has_special_rights", "0");
        response.append_kv_argument("badge_type", &self.badge);
    }
}
