use bcrypt::{hash_with_result, verify, BcryptError, Version};

pub const JAVA_BCRYPT_COST: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasswordHasher {
    cost: u32,
}

impl PasswordHasher {
    pub const fn new(cost: u32) -> Self {
        Self { cost }
    }

    pub const fn java_compatible() -> Self {
        Self::new(JAVA_BCRYPT_COST)
    }

    pub const fn cost(&self) -> u32 {
        self.cost
    }

    pub fn hash_password(&self, password: &str) -> Result<String, BcryptError> {
        hash_with_result(password, self.cost).map(|parts| parts.format_for_version(Version::TwoA))
    }

    pub fn verify_password(&self, password: &str, stored_hash: &str) -> Result<bool, BcryptError> {
        verify(password, stored_hash)
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::java_compatible()
    }
}

#[cfg(test)]
mod tests {
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
}
