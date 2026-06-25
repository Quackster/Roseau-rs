use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallForHelp {
    room_name: String,
    from_name: String,
    message: String,
    time: String,
}

impl CallForHelp {
    pub fn new(
        room_name: impl Into<String>,
        from_name: impl Into<String>,
        message: impl Into<String>,
        time: impl Into<String>,
    ) -> Self {
        Self {
            room_name: room_name.into(),
            from_name: from_name.into(),
            message: message.into(),
            time: time.into(),
        }
    }

    pub fn room_name(&self) -> &str {
        &self.room_name
    }

    pub fn from_name(&self) -> &str {
        &self.from_name
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn time(&self) -> &str {
        &self.time
    }
}

impl SerializableObject for CallForHelp {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_new_argument(format!("Private Room: {} @ {}", self.room_name, self.time));
        response.append_new_argument("url");
        response.append_new_argument(format!(
            "From: {};0;Message: {}",
            self.from_name, self.message
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialises_call_for_help_like_java_object() {
        let call = CallForHelp::new("Lobby", "alice", "help", "2026-06-24 10:00");
        let mut response = NettyResponse::with_header("CRYFORHELP");
        response.append_object(&call);

        assert_eq!(
            response.get(),
            "#CRYFORHELP\rPrivate Room: Lobby @ 2026-06-24 10:00\rurl\rFrom: alice;0;Message: help##"
        );
    }
}
