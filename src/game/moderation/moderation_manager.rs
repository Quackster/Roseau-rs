use crate::game::moderation::{CallForHelp, ModerationEffect};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModerationManager;

impl ModerationManager {
    pub fn new() -> Self {
        Self
    }

    pub fn call_for_help(
        &self,
        moderator_connections: impl IntoIterator<Item = i32>,
        room_name: &str,
        from_name: &str,
        distress_message: &str,
        time: &str,
    ) -> Vec<ModerationEffect> {
        let sanitized_message = distress_message.replace(';', ",");

        moderator_connections
            .into_iter()
            .map(
                |moderator_connection_id| ModerationEffect::SendCallForHelp {
                    moderator_connection_id,
                    call: CallForHelp::new(room_name, from_name, &sanitized_message, time),
                },
            )
            .collect()
    }
}
