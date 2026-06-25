use crate::game::GameTickEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameTickRuntimeEffect {
    SendCreditBalance { user_id: i32, new_balance: i32 },
    ResolveServerIp,
    KickAfkUser { user_id: i32 },
}

impl GameTickRuntimeEffect {
    pub fn from_tick_effect(effect: &GameTickEffect) -> Option<Self> {
        match effect {
            GameTickEffect::AwardCredits {
                user_id,
                new_balance,
                ..
            } => Some(Self::SendCreditBalance {
                user_id: *user_id,
                new_balance: *new_balance,
            }),
            GameTickEffect::ResolveServerIp => Some(Self::ResolveServerIp),
            GameTickEffect::KickAfkUser { user_id } => {
                Some(Self::KickAfkUser { user_id: *user_id })
            }
            GameTickEffect::SavePlayer { .. } => None,
        }
    }

    pub fn collect(effects: &[GameTickEffect]) -> Vec<Self> {
        effects.iter().filter_map(Self::from_tick_effect).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_runtime_only_tick_effects() {
        let effects = [
            GameTickEffect::AwardCredits {
                user_id: 1,
                amount: 10,
                new_balance: 20,
            },
            GameTickEffect::SavePlayer { user_id: 1 },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 7 },
        ];

        assert_eq!(
            GameTickRuntimeEffect::collect(&effects),
            vec![
                GameTickRuntimeEffect::SendCreditBalance {
                    user_id: 1,
                    new_balance: 20,
                },
                GameTickRuntimeEffect::ResolveServerIp,
                GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
            ]
        );
    }
}
