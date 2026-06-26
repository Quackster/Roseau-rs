use super::*;

#[test]
fn hashes_with_java_bcrypt_shape() {
    let hasher = PasswordHasher::new(4);

    let hash = hasher.hash_password("secret").unwrap();

    assert!(hash.starts_with("$2a$04$"));
    assert!(hasher.verify_password("secret", &hash).unwrap());
    assert!(!hasher.verify_password("wrong", &hash).unwrap());
}

#[test]
fn defaults_to_java_bcrypt_cost() {
    let hasher = PasswordHasher::default();

    assert_eq!(hasher.cost(), JAVA_BCRYPT_COST);
}

#[test]
fn verifies_existing_java_bcrypt_hashes() {
    let hasher = PasswordHasher::default();

    assert!(hasher
        .verify_password(
            "password",
            "$2a$04$UuTkLRZZ6QofpDOlMz32MuuxEHA43WOemOYHPz6.SjsVsyO1tDU96",
        )
        .unwrap());
}
