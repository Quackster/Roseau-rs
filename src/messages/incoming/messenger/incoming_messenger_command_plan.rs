use crate::messages::{IncomingCommand, IncomingExecutionEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct IncomingMessengerCommandPlan;

impl IncomingMessengerCommandPlan {
    pub(crate) fn plan(command: &IncomingCommand) -> Option<Vec<IncomingExecutionEffect>> {
        match command {
            IncomingCommand::MarkMessengerMessageRead { message_id } => {
                Some(vec![IncomingExecutionEffect::MarkMessengerMessageRead {
                    message_id: *message_id,
                }])
            }
            IncomingCommand::AssignPersonalMessage { message } => {
                Some(vec![IncomingExecutionEffect::AssignPersonalMessage {
                    message: message.clone(),
                }])
            }
            IncomingCommand::RequestBuddy { username } => {
                Some(vec![IncomingExecutionEffect::RequestBuddy {
                    username: username.clone(),
                }])
            }
            IncomingCommand::AcceptBuddy { username } => {
                Some(vec![IncomingExecutionEffect::AcceptBuddy {
                    username: username.clone(),
                }])
            }
            IncomingCommand::DeclineBuddy { username } => {
                Some(vec![IncomingExecutionEffect::DeclineBuddy {
                    username: username.clone(),
                }])
            }
            IncomingCommand::RemoveBuddy { username } => {
                Some(vec![IncomingExecutionEffect::RemoveBuddy {
                    username: username.clone(),
                }])
            }
            IncomingCommand::SendMessengerMessage {
                receiver_ids,
                message,
            } => Some(vec![IncomingExecutionEffect::SendMessengerMessage {
                receiver_ids: receiver_ids.clone(),
                message: message.clone(),
            }]),
            IncomingCommand::InitMessenger => Some(vec![IncomingExecutionEffect::InitMessenger]),
            _ => None,
        }
    }
}
