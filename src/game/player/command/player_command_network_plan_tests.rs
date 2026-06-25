use super::player_command_network_plan::*;
use crate::game::player::PlayerDetails;

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "hello",
        "hd-100",
        "",
        "alice@example.test",
        1,
        50,
        "F",
        "UK",
        "",
        "1990-01-01",
        1234,
        "welcome",
        9,
    );
    details
}

#[test]
fn maps_retrieve_user_info_to_current_connection_packet() {
    let effects =
        PlayerCommandNetworkPlan::plan(&PlayerCommandOutcome::retrieve_user_info(&details()), 42);

    assert_eq!(effects.len(), 1);
    assert_eq!(
        effects[0],
        PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#USEROBJECT\rname=alice\rfigure=hd-100\remail=alice@example.test\rbirthday=1990-01-01\rphonenumber=+44\rcustomData=hello\rhas_read_agreement=1\rsex=F\rcountry=UK\rhas_special_rights=0\rbadge_type=##".to_owned(),
        }
    );
}

#[test]
fn maps_ticket_count_to_current_connection_packet() {
    let effects =
        PlayerCommandNetworkPlan::plan(&PlayerCommandOutcome::send_tickets(&details()), 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#PH_TICKETS 9##".to_owned(),
        }]
    );
}
