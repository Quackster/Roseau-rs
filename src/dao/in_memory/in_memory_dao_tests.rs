use super::in_memory_dao::*;

#[test]
fn exposes_player_dao_and_connection_state() {
    let mut dao = InMemoryDao::new(InMemoryPlayerDao::new());

    assert!(dao.is_connected());
    assert!(dao.player().is_empty());
    assert!(dao.catalogue().is_empty());
    assert!(dao.item().is_empty());
    assert!(dao.messenger().is_empty());
    assert!(dao.room().is_empty());
    assert!(Rc::ptr_eq(&dao.item, &dao.inventory().item_dao()));
    assert!(dao.connect());
}
