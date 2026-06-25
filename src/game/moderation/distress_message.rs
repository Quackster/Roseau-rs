use crate::game::room::settings::RoomType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistressMessage {
    text: String,
}

impl DistressMessage {
    pub fn from_payload(
        room_type: RoomType,
        room_name: &str,
        room_id: i32,
        body: &str,
        first_tab_argument: &str,
    ) -> Self {
        let text = match room_type {
            RoomType::Private => private_room_message(room_name, room_id, first_tab_argument),
            RoomType::Public => public_room_message(room_name, body),
        };

        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

fn private_room_message(room_name: &str, room_id: i32, first_tab_argument: &str) -> String {
    let without_room = first_tab_argument
        .strip_prefix(&format!("/Private Room: {room_name}"))
        .unwrap_or(first_tab_argument);
    let without_prefix = skip_chars(without_room, 3);
    without_prefix.replace(&format!(";{room_id}"), "")
}

fn public_room_message(room_name: &str, body: &str) -> String {
    let without_room = body.strip_prefix(&format!("/{room_name}")).unwrap_or(body);
    let without_prefix = skip_chars(without_room, 3);
    without_prefix
        .split(';')
        .next()
        .unwrap_or_default()
        .to_owned()
}

fn skip_chars(value: &str, count: usize) -> &str {
    value
        .char_indices()
        .nth(count)
        .map(|(index, _)| &value[index..])
        .unwrap_or_default()
}
