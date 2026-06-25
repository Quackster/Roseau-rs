use crate::dao::mysql::{MessengerQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessengerCommandQueries;

impl MessengerCommandQueries {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        sender_id: i32,
        time_sent: i64,
    ) -> Vec<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::MarkMessengerMessageRead { message_id } => {
                vec![MessengerQueries::mark_message_read(*message_id).execute_plan()]
            }
            IncomingExecutionEffect::SendMessengerMessage {
                receiver_ids,
                message,
            } => receiver_ids
                .iter()
                .map(|receiver_id| {
                    MessengerQueries::create_message(sender_id, *receiver_id, time_sent, message)
                        .execute_plan()
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}
