#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    permission: String,
    inheritable: bool,
    rank: i32,
}

impl Permission {
    pub fn new(permission: impl Into<String>, inheritable: bool, rank: i32) -> Self {
        Self {
            permission: permission.into(),
            inheritable,
            rank,
        }
    }

    pub fn permission(&self) -> &str {
        &self.permission
    }

    pub fn is_inheritable(&self) -> bool {
        self.inheritable
    }

    pub fn rank(&self) -> i32 {
        self.rank
    }
}

#[cfg(test)]
#[path = "permission_tests.rs"]
mod tests;
