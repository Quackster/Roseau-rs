use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitMembers {
    names: Vec<String>,
}

impl UnitMembers {
    pub fn new(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}

impl OutgoingMessage for UnitMembers {
    fn write(&self, response: &mut NettyResponse) {
        response.init("UNITMEMBERS");

        for name in &self.names {
            response.append_new_argument(name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_unit_members_packet() {
        let mut response = UnitMembers::new(["alice", "bob"]).compose();

        assert_eq!(response.get(), "#UNITMEMBERS\ralice\rbob##");
    }
}
