use crate::game::moderation::CallForHelp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModerationEffect {
    SendCallForHelp {
        moderator_connection_id: i32,
        call: CallForHelp,
    },
}
