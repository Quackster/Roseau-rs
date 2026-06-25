use super::player_profile_update_executor::*;
use crate::dao::in_memory::InMemoryPlayerDao;
use crate::dao::{CreatePlayer, PlayerDao};

fn create_player() -> CreatePlayer {
    CreatePlayer::new(
        "alice",
        "secret",
        "alice@example.test",
        "hello",
        "hd=100",
        50,
        "Female",
        "08.08.1997",
    )
}

fn seeded_dao() -> InMemoryPlayerDao {
    let dao = InMemoryPlayerDao::new();
    dao.create_player(&create_player()).unwrap();
    dao
}

#[test]
fn persists_profile_update_and_clears_pool_figure_when_sex_changes() {
    let dao = seeded_dao();
    let mut current = dao.details_by_username("alice").unwrap().unwrap();
    current.set_pool_figure("pool=old");
    dao.update_player(&current).unwrap();
    let action = PasswordAction::hash_profile_update(
        Some(current.id()),
        "changed",
        "new@example.test",
        "hd=200",
        "new mission",
        "Male",
    );

    let outcome = PlayerProfileUpdateExecutor::update_profile(&dao, &current, &action).unwrap();

    let updated = outcome.details().unwrap();
    let persisted = dao.details_by_username("alice").unwrap().unwrap();
    assert_eq!(updated.password(), "changed");
    assert_eq!(persisted.password(), "changed");
    assert_eq!(persisted.email(), "new@example.test");
    assert_eq!(persisted.figure(), "hd=200");
    assert_eq!(persisted.mission(), "new mission");
    assert_eq!(persisted.sex(), "Male");
    assert_eq!(persisted.pool_figure(), "");
}

#[test]
fn ignores_profile_update_when_action_targets_another_user() {
    let dao = seeded_dao();
    let current = dao.details_by_username("alice").unwrap().unwrap();
    let action = PasswordAction::hash_profile_update(
        Some(current.id() + 1),
        "changed",
        "new@example.test",
        "hd=200",
        "new mission",
        "Male",
    );

    let outcome = PlayerProfileUpdateExecutor::update_profile(&dao, &current, &action).unwrap();

    let persisted = dao.details_by_username("alice").unwrap().unwrap();
    assert_eq!(outcome, PlayerProfileUpdateOutcome::Ignored);
    assert_eq!(persisted.password(), "secret");
    assert_eq!(persisted.email(), "alice@example.test");
}

#[test]
fn persists_pool_figure_update() {
    let dao = seeded_dao();
    let current = dao.details_by_username("alice").unwrap().unwrap();

    let outcome =
        PlayerProfileUpdateExecutor::update_pool_figure(&dao, &current, "ph=001").unwrap();

    let updated = outcome.details().unwrap();
    let persisted = dao.details_by_username("alice").unwrap().unwrap();
    assert_eq!(updated.pool_figure(), "ph=001");
    assert_eq!(persisted.pool_figure(), "ph=001");
}
