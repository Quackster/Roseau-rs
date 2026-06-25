use crate::dao::{DaoError, PlayerDao};
use crate::game::player::{FindUserOutcome, PlayerCommandOutcome, PlayerDetails};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerIncomingOutcome {
    Command(PlayerCommandOutcome),
    FindUser(FindUserOutcome),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerIncomingPlan;

impl PlayerIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        player_dao: &dyn PlayerDao,
        current_player: &PlayerDetails,
        find_user_last_seen: &str,
        find_user_location: &str,
    ) -> Result<Vec<PlayerIncomingOutcome>, DaoError> {
        match effect {
            IncomingExecutionEffect::RetrieveUserInfo => Ok(vec![PlayerIncomingOutcome::Command(
                PlayerCommandOutcome::retrieve_user_info(current_player),
            )]),
            IncomingExecutionEffect::SendTickets => Ok(vec![PlayerIncomingOutcome::Command(
                PlayerCommandOutcome::send_tickets(current_player),
            )]),
            IncomingExecutionEffect::FindUser { username } => {
                let outcome = if username.is_empty() {
                    FindUserOutcome::Missing
                } else {
                    player_dao
                        .details_by_username(username)?
                        .map(|details| {
                            FindUserOutcome::found(
                                &details,
                                find_user_last_seen,
                                find_user_location,
                            )
                        })
                        .unwrap_or(FindUserOutcome::Missing)
                };

                Ok(vec![PlayerIncomingOutcome::FindUser(outcome)])
            }
            _ => Ok(Vec::new()),
        }
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        player_dao: &dyn PlayerDao,
        current_player: &PlayerDetails,
        find_user_last_seen: &str,
        find_user_location: &str,
    ) -> Result<Vec<PlayerIncomingOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(
                effect,
                player_dao,
                current_player,
                find_user_last_seen,
                find_user_location,
            )?);
        }

        Ok(outcomes)
    }
}
