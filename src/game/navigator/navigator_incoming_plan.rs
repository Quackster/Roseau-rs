use crate::dao::{DaoError, NavigatorDao, RoomDao};
use crate::game::navigator::{NavigatorCommandExecutor, NavigatorSearchOutcome};
use crate::game::player::PlayerManager;
use crate::game::room::RoomManager;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigatorIncomingPlan;

impl NavigatorIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        navigator_dao: &dyn NavigatorDao,
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        server_ip: &str,
        private_server_port: u16,
    ) -> Result<Vec<NavigatorSearchOutcome>, DaoError> {
        Ok(NavigatorCommandExecutor::execute(
            effect,
            navigator_dao,
            room_dao,
            room_manager,
            player_manager,
            server_ip,
            private_server_port,
        )?
        .into_iter()
        .collect())
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        navigator_dao: &dyn NavigatorDao,
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        server_ip: &str,
        private_server_port: u16,
    ) -> Result<Vec<NavigatorSearchOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(
                effect,
                navigator_dao,
                room_dao,
                room_manager,
                player_manager,
                server_ip,
                private_server_port,
            )?);
        }

        Ok(outcomes)
    }
}
