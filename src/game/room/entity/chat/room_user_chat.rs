use crate::game::room::entity::{ChatUtility, RoomUser, RoomUserEffect};
use crate::settings::BOT_RESPONSE_DELAY_MS;

impl RoomUser {
    pub fn chat(
        &mut self,
        header: impl Into<String>,
        message: impl Into<String>,
    ) -> RoomUserEffect {
        let header = header.into();
        let message = message.into();

        if header == "WHISPER" {
            let (target_username, message) = Self::parse_whisper(&message);
            return RoomUserEffect::Whisper {
                username: self.username.clone(),
                target_username,
                message,
            };
        }

        if matches!(header.as_str(), "CHAT" | "SHOUT") {
            self.apply_talk_statuses(&message);
        }

        RoomUserEffect::Chat {
            header,
            username: self.username.clone(),
            message,
        }
    }

    pub fn delayed_chat(&self, message: impl Into<String>) -> RoomUserEffect {
        RoomUserEffect::DelayedChat {
            username: self.username.clone(),
            message: message.into(),
            delay_ms: BOT_RESPONSE_DELAY_MS,
        }
    }

    fn apply_talk_statuses(&mut self, message: &str) {
        let talk_duration = Self::talk_duration(message);
        if let Some(emote) = ChatUtility::detect_emote(message.split(' '), false) {
            self.set_status_with_update("gest", format!(" {emote}"), false, 5, true);
        }
        self.set_status_with_update("talk", "", false, talk_duration, true);
    }

    fn talk_duration(message: &str) -> i64 {
        let length = message.encode_utf16().count();
        if length <= 1 {
            1
        } else if length >= 10 {
            5
        } else {
            (length / 2) as i64
        }
    }

    fn parse_whisper(message: &str) -> (Option<String>, String) {
        let mut parts = message.split(' ').collect::<Vec<_>>();
        while parts.last().is_some_and(|part| part.is_empty()) {
            parts.pop();
        }

        if parts.len() > 1 {
            let target_username = parts[0].to_owned();
            let body = message[target_username.len() + 1..].to_owned();
            (Some(target_username), body)
        } else {
            (None, message.to_owned())
        }
    }
}
