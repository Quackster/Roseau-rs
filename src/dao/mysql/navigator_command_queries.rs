use crate::dao::mysql::{NavigatorQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigatorCommandQueries;

impl NavigatorCommandQueries {
    pub fn plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::SearchFlat { query } => {
                Some(NavigatorQueries::rooms_by_like_name(query).read_plan())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

    #[test]
    fn maps_flat_name_search_to_private_room_lookup() {
        let plan = NavigatorCommandQueries::plan(&IncomingExecutionEffect::SearchFlat {
            query: "cafe".to_owned(),
        })
        .unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::ReadRows);
        assert_eq!(
            plan.sql(),
            "SELECT * FROM rooms WHERE name LIKE ? AND room_type = ?"
        );
        assert_eq!(
            plan.parameters(),
            &[
                SqlParameter::Text("%cafe%".to_owned()),
                SqlParameter::Integer(0),
            ]
        );
    }

    #[test]
    fn ignores_runtime_only_navigator_effects() {
        assert_eq!(
            NavigatorCommandQueries::plan(&IncomingExecutionEffect::SearchBusyFlats {
                multiplier: 2,
            }),
            None
        );
        assert_eq!(
            NavigatorCommandQueries::plan(&IncomingExecutionEffect::EmptySearchBusyFlats),
            None
        );
        assert_eq!(
            NavigatorCommandQueries::plan(&IncomingExecutionEffect::SearchFlatForUser {
                username: "alice".to_owned(),
            }),
            None
        );
    }
}
