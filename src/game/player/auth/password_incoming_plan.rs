use crate::game::player::PasswordAction;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PasswordIncomingPlan;

impl PasswordIncomingPlan {
    pub fn plan(effect: &IncomingExecutionEffect) -> Vec<PasswordAction> {
        let IncomingExecutionEffect::Password(action) = effect else {
            return Vec::new();
        };

        vec![action.clone()]
    }

    pub fn plan_all(effects: &[IncomingExecutionEffect]) -> Vec<PasswordAction> {
        effects.iter().flat_map(Self::plan).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_password_actions_from_incoming_effects() {
        let effects = PasswordIncomingPlan::plan_all(&[
            IncomingExecutionEffect::GoAway,
            IncomingExecutionEffect::Password(PasswordAction::verify_login(
                "alice", "secret", true,
            )),
            IncomingExecutionEffect::Password(PasswordAction::hash_registration(
                "bob",
                "door",
                "bob@example.test",
                "hello",
                "hd=100",
                "M",
                "1990-01-01",
            )),
        ]);

        assert_eq!(
            effects,
            vec![
                PasswordAction::verify_login("alice", "secret", true),
                PasswordAction::hash_registration(
                    "bob",
                    "door",
                    "bob@example.test",
                    "hello",
                    "hd=100",
                    "M",
                    "1990-01-01",
                ),
            ]
        );
    }

    #[test]
    fn preserves_profile_update_password_action() {
        let effects = PasswordIncomingPlan::plan(&IncomingExecutionEffect::Password(
            PasswordAction::hash_profile_update(
                Some(7),
                "changed",
                "alice@example.test",
                "hd=200",
                "mission",
                "F",
            ),
        ));

        assert_eq!(
            effects,
            vec![PasswordAction::hash_profile_update(
                Some(7),
                "changed",
                "alice@example.test",
                "hd=200",
                "mission",
                "F",
            )]
        );
    }

    #[test]
    fn ignores_unrelated_incoming_effects() {
        assert!(PasswordIncomingPlan::plan(&IncomingExecutionEffect::ResetAfkTimer).is_empty());
    }
}
