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
