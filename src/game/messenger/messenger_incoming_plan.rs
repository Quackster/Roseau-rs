use crate::dao::{DaoError, MessengerDao, PlayerDao};
use crate::game::messenger::{Messenger, MessengerCommandExecutor, MessengerCommandOutcome};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerIncomingPlan;

impl MessengerIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
    ) -> Result<Vec<MessengerCommandOutcome>, DaoError> {
        let outcome = match effect {
            IncomingExecutionEffect::RequestBuddy { username } => {
                MessengerCommandExecutor::request_buddy(
                    player_dao,
                    messenger_dao,
                    messenger,
                    username,
                )?
            }
            IncomingExecutionEffect::AcceptBuddy { username } => {
                MessengerCommandExecutor::accept_buddy(
                    player_dao,
                    messenger_dao,
                    messenger,
                    username,
                )?
            }
            IncomingExecutionEffect::DeclineBuddy { username } => {
                MessengerCommandExecutor::decline_buddy(
                    player_dao,
                    messenger_dao,
                    messenger,
                    username,
                )?
            }
            IncomingExecutionEffect::RemoveBuddy { username } => {
                MessengerCommandExecutor::remove_buddy(
                    player_dao,
                    messenger_dao,
                    messenger,
                    username,
                )?
            }
            IncomingExecutionEffect::SendMessengerMessage {
                receiver_ids,
                message,
            } => MessengerCommandExecutor::send_message(
                messenger_dao,
                messenger,
                receiver_ids,
                message,
            )?,
            IncomingExecutionEffect::MarkMessengerMessageRead { message_id } => {
                MessengerCommandExecutor::mark_read(
                    messenger_dao,
                    messenger.user_id(),
                    *message_id,
                )?
            }
            _ => return Ok(Vec::new()),
        };

        Ok(vec![outcome])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
    ) -> Result<Vec<MessengerCommandOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(effect, player_dao, messenger_dao, messenger)?);
        }

        Ok(outcomes)
    }
}

#[cfg(test)]
#[path = "messenger_incoming_plan_tests.rs"]
mod tests;
