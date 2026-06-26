use super::*;

#[test]
fn stores_permission_metadata() {
    let permission = Permission::new("room.admin", true, 7);

    assert_eq!(permission.permission(), "room.admin");
    assert!(permission.is_inheritable());
    assert_eq!(permission.rank(), 7);
}
