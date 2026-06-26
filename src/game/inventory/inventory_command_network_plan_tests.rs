use super::*;
use crate::messages::outgoing::{StripInfo, StripItem, StripItemKind};

#[test]
fn maps_refreshed_inventory_to_current_connection_packet() {
    let execution = InventoryCommandExecution::Refreshed {
        strip_info: StripInfo::new([StripItem::new(
            7,
            "chair",
            "Chair",
            "blue",
            StripItemKind::Stuff {
                length: 1,
                width: 2,
                color: "red".to_owned(),
            },
        )]),
    };

    let effects = InventoryCommandNetworkPlan::plan(&execution, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#STRIPINFO\rroseau;7;0;S;0;chair;Chair;blue;1;2;red/##".to_owned(),
        }]
    );
}

#[test]
fn empty_inventory_refresh_has_no_network_effect() {
    assert!(InventoryCommandNetworkPlan::plan(&InventoryCommandExecution::Empty, 42).is_empty());
}
