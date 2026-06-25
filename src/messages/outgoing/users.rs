use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq)]
pub struct Users {
    entities: Vec<UserEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserEntry {
    name: String,
    figure: String,
    x: i32,
    y: i32,
    z: f64,
    mission: String,
    pool_figure: Option<String>,
}

impl Users {
    pub fn new(entities: impl IntoIterator<Item = UserEntry>) -> Self {
        Self {
            entities: entities.into_iter().collect(),
        }
    }
}

impl UserEntry {
    pub fn new(
        name: impl Into<String>,
        figure: impl Into<String>,
        x: i32,
        y: i32,
        z: f64,
        mission: impl Into<String>,
        pool_figure: Option<impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            figure: figure.into(),
            x,
            y,
            z,
            mission: mission.into(),
            pool_figure: pool_figure.map(Into::into),
        }
    }
}

impl OutgoingMessage for Users {
    fn write(&self, response: &mut NettyResponse) {
        response.init("USERS");

        for entity in &self.entities {
            response.append('\r');
            response.append_argument("");
            response.append_argument(&entity.name);
            response.append_argument(&entity.figure);
            response.append_argument(entity.x);
            response.append_argument(entity.y);
            response.append_argument(entity.z);
            response.append_argument(&entity.mission);

            if let Some(pool_figure) = &entity.pool_figure {
                response.append_argument(pool_figure);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_users_packet() {
        let mut response = Users::new([UserEntry::new(
            "alice",
            "hd-100",
            1,
            2,
            3.5,
            "hello",
            Some("pool"),
        )])
        .compose();

        assert_eq!(
            response.get(),
            "#USERS\r  alice hd-100 1 2 3.5 hello pool##"
        );
    }
}
