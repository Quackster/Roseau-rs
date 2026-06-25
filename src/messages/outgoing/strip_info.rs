use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripInfo {
    items: Vec<StripItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripItem {
    id: i32,
    sprite: String,
    name: String,
    custom_data: String,
    kind: StripItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StripItemKind {
    Stuff {
        length: i32,
        width: i32,
        color: String,
    },
    Item {
        post_it: bool,
    },
    Other,
}

impl StripInfo {
    pub fn new(items: impl IntoIterator<Item = StripItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl StripItem {
    pub fn new(
        id: i32,
        sprite: impl Into<String>,
        name: impl Into<String>,
        custom_data: impl Into<String>,
        kind: StripItemKind,
    ) -> Self {
        Self {
            id,
            sprite: sprite.into(),
            name: name.into(),
            custom_data: custom_data.into(),
            kind,
        }
    }
}

impl OutgoingMessage for StripInfo {
    fn write(&self, response: &mut NettyResponse) {
        response.init("STRIPINFO");

        for item in &self.items {
            response.append_new_argument("roseau");
            response.append_argument_with(item.id, ';');
            response.append_argument_with("0", ';');

            match &item.kind {
                StripItemKind::Stuff { .. } => response.append_argument_with("S", ';'),
                StripItemKind::Item { .. } => response.append_argument_with("I", ';'),
                StripItemKind::Other => {}
            }

            response.append_argument_with("0", ';');
            response.append_argument_with(&item.sprite, ';');
            response.append_argument_with(&item.name, ';');

            match &item.kind {
                StripItemKind::Stuff {
                    length,
                    width,
                    color,
                } => {
                    response.append_argument_with(&item.custom_data, ';');
                    response.append_argument_with(length, ';');
                    response.append_argument_with(width, ';');
                    response.append_argument_with(color, ';');
                }
                StripItemKind::Item { post_it } => {
                    if *post_it && item.custom_data == "1" {
                        response.append_argument_with("2", ';');
                        response.append_argument_with("2", ';');
                    } else {
                        response.append_argument_with(&item.custom_data, ';');
                        response.append_argument_with(&item.custom_data, ';');
                    }
                }
                StripItemKind::Other => {
                    response.append_argument_with(&item.custom_data, ';');
                    response.append_argument_with(&item.custom_data, ';');
                }
            }

            response.append_argument_with("", '/');
        }
    }
}
