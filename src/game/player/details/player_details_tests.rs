use crate::game::player::PlayerDetails;
use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[test]
fn fills_full_player_details_and_serialises_user_object_fields() {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "hello",
        "hd-180-1",
        "pool",
        "alice@example.test",
        4,
        55,
        "F",
        "UK",
        "ADM",
        "1990-01-01",
        1234,
        "welcome",
        8,
    );

    let mut response = NettyResponse::new();
    details.serialise(&mut response);

    assert_eq!(details.id(), 7);
    assert_eq!(details.rank(), 4);
    assert_eq!(details.pool_figure(), "pool");
    assert_eq!(details.personal_greeting(), "welcome");
    assert_eq!(
        response.get(),
        "\rname=alice\rfigure=hd-180-1\remail=alice@example.test\rbirthday=1990-01-01\rphonenumber=+44\rcustomData=hello\rhas_read_agreement=1\rsex=F\rcountry=UK\rhas_special_rights=0\rbadge_type=ADM##"
    );
}

#[test]
fn creates_credit_and_ticket_packets_from_current_totals() {
    let mut details = PlayerDetails::new();
    details.set_credits(42);
    details.set_tickets(9);

    let mut credits = details.wallet_balance().compose();
    let mut tickets = details.ph_tickets().compose();

    assert_eq!(credits.get(), "#WALLETBALANCE\r42##");
    assert_eq!(tickets.get(), "#PH_TICKETS 9##");
}

#[test]
fn assigns_credit_and_ticket_totals_like_java_setters() {
    let mut details = PlayerDetails::new();
    details.set_credits(100);
    details.set_tickets(25);

    details.set_credits(i32::MAX);
    details.set_tickets(i32::MAX);

    assert_eq!(details.credits(), i32::MAX);
    assert_eq!(details.tickets(), i32::MAX);
}

#[test]
fn keeps_fuse_lookup_as_unimplemented_false() {
    assert!(!PlayerDetails::new().has_fuse("room_admin"));
}
