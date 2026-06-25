use super::*;

#[test]
fn maps_order_info_plan_to_current_connection_packet() {
    let effects = CatalogueOrderInfoNetworkPlan::plan(&CatalogueOrderInfoPlan::new("chair", 5), 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#ORDERINFO\rchair\r5##".to_owned(),
        }]
    );
}

#[test]
fn maps_multiple_order_info_plans_in_order() {
    let effects = CatalogueOrderInfoNetworkPlan::plan_all(
        &[
            CatalogueOrderInfoPlan::new("chair", 5),
            CatalogueOrderInfoPlan::new("poster L", 4),
        ],
        42,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ORDERINFO\rchair\r5##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ORDERINFO\rposter L\r4##".to_owned(),
            },
        ]
    );
}
