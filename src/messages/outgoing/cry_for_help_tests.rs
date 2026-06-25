use super::cry_for_help::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct HelpCall;

impl SerializableObject for HelpCall {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_argument("alice");
    }
}

#[test]
fn composes_cry_for_help_packet() {
    let mut response = CryForHelp::new(HelpCall).compose();

    assert_eq!(response.get(), "#CRYFORHELP alice##");
}
