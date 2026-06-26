use super::*;

#[test]
fn exposes_database_result_and_runtime_effects() {
    let report = MySqlApplicationTickExecutionReport::new(
        SqlExecutionBatchResult::new([]),
        [
            GameTickEffect::AwardCredits {
                user_id: 3,
                amount: 10,
                new_balance: 20,
            },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 7 },
        ],
    );

    assert!(report.database_result().results().is_empty());
    assert_eq!(
        report.runtime_effects(),
        &[
            GameTickEffect::AwardCredits {
                user_id: 3,
                amount: 10,
                new_balance: 20,
            },
            GameTickEffect::ResolveServerIp,
            GameTickEffect::KickAfkUser { user_id: 7 },
        ]
    );
    assert_eq!(
        report.runtime_actions(),
        vec![
            GameTickRuntimeEffect::SendCreditBalance {
                user_id: 3,
                new_balance: 20,
            },
            GameTickRuntimeEffect::ResolveServerIp,
            GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
        ]
    );
}
