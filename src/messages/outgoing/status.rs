use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    entities: Vec<StatusEntity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusEntity {
    name: String,
    x: i32,
    y: i32,
    z: String,
    head_rotation: i32,
    rotation: i32,
    statuses: Vec<RoomUserStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomUserStatus {
    status: String,
    value: String,
}

impl Status {
    pub fn new(entities: impl IntoIterator<Item = StatusEntity>) -> Self {
        Self {
            entities: entities.into_iter().collect(),
        }
    }
}

impl StatusEntity {
    pub fn new(
        name: impl Into<String>,
        x: i32,
        y: i32,
        z: impl Into<String>,
        head_rotation: i32,
        rotation: i32,
        statuses: impl IntoIterator<Item = RoomUserStatus>,
    ) -> Self {
        Self {
            name: name.into(),
            x,
            y,
            z: z.into(),
            head_rotation,
            rotation,
            statuses: statuses.into_iter().collect(),
        }
    }
}

impl RoomUserStatus {
    pub fn new(status: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            status: status.into(),
            value: value.into(),
        }
    }
}

impl OutgoingMessage for Status {
    fn write(&self, response: &mut NettyResponse) {
        response.init("STATUS ");

        for entity in &self.entities {
            response.append_new_argument(&entity.name);
            response.append_argument(entity.x);
            response.append_argument_with(entity.y, ',');
            response.append_argument_with(&entity.z, ',');
            response.append_argument_with(entity.head_rotation, ',');
            response.append_argument_with(entity.rotation, ',');

            response.append("/");
            for status in &entity.statuses {
                response.append(&status.status);
                response.append(&status.value);
                response.append("/");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_status_packet() {
        let mut response = Status::new([StatusEntity::new(
            "alice",
            1,
            2,
            "3.0",
            4,
            5,
            [RoomUserStatus::new("sit", " 1.0")],
        )])
        .compose();

        assert_eq!(response.get(), "#STATUS \ralice 1,2,3.0,4,5/sit 1.0/##");
    }
}
