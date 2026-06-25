use super::call_for_help::*;

#[test]
fn serialises_call_for_help_like_java_object() {
    let call = CallForHelp::new("Lobby", "alice", "help", "2026-06-24 10:00");
    let mut response = NettyResponse::with_header("CRYFORHELP");
    response.append_object(&call);

    assert_eq!(
        response.get(),
        "#CRYFORHELP\rPrivate Room: Lobby @ 2026-06-24 10:00\rurl\rFrom: alice;0;Message: help##"
    );
}
