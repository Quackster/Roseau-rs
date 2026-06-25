use crate::game::room::entity::{RoomUser, RoomUserCommandExecutor, RoomUserEffect};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUserIncomingPlan;

impl RoomUserIncomingPlan {
    pub fn plan(user: &mut RoomUser, effect: &IncomingExecutionEffect) -> Vec<RoomUserEffect> {
        RoomUserCommandExecutor::apply(user, effect)
    }

    pub fn plan_all(
        user: &mut RoomUser,
        effects: &[IncomingExecutionEffect],
    ) -> Vec<RoomUserEffect> {
        RoomUserCommandExecutor::apply_all(user, effects)
    }
}
