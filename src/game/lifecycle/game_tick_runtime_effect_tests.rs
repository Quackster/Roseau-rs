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
