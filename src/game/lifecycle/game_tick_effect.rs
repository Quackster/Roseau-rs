#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameTickEffect {
    AwardCredits {
        user_id: i32,
        amount: i32,
        new_balance: i32,
    },
    SavePlayer {
        user_id: i32,
    },
    ResolveServerIp,
    KickAfkUser {
        user_id: i32,
    },
}
