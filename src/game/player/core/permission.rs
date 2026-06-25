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
mod tests {
    use super::*;

    #[test]
    fn stores_permission_metadata() {
        let permission = Permission::new("room.admin", true, 7);

        assert_eq!(permission.permission(), "room.admin");
        assert!(permission.is_inheritable());
        assert_eq!(permission.rank(), 7);
    }
}
