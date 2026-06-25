use super::catalogue_command_queries::*;
use crate::dao::mysql::SqlExecutionKind;

#[test]
fn maps_order_info_to_catalogue_table_reads() {
    let plans = CatalogueCommandQueries::read_plans(&IncomingExecutionEffect::GetOrderInfo {
        call_id: "chair".to_owned(),
    });

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plans[0].sql(), "SELECT * FROM catalogue");
    assert_eq!(plans[1].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plans[1].sql(), "SELECT * FROM catalogue_deals");
}

#[test]
fn maps_purchase_to_catalogue_table_reads() {
    let plans = CatalogueCommandQueries::read_plans(&IncomingExecutionEffect::Purchase {
        call_id: "chair".to_owned(),
    });

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].sql(), "SELECT * FROM catalogue");
    assert_eq!(plans[1].sql(), "SELECT * FROM catalogue_deals");
}

#[test]
fn ignores_non_catalogue_effects() {
    assert!(
        CatalogueCommandQueries::read_plans(&IncomingExecutionEffect::RetrieveUserInfo).is_empty()
    );
}
