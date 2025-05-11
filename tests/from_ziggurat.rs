use abstract_bits::AbstractBits;
/// These tests where taken and adapted from the `zigpy/ziggurat` project.

/// Zigbee spec 3.4.2 Route Reply Command
#[abstract_bits::abstract_bits]
#[derive(Debug, PartialEq)]
pub struct NwkRouteReplyCommand {
    reserved: u4,
    #[abstract_bits(controls = originator_eui64)]
    reserved: bool,
    #[abstract_bits(controls = responder_eui64)]
    reserved: bool,
    reserved: u2,
    pub route_request_identifier: u8,
    pub originator_nwk: Nwk,
    pub responder_nwk: Nwk,
    pub path_cost: u8,
    pub originator_eui64: Option<Eui64>,
    pub responder_eui64: Option<Eui64>,
}

#[abstract_bits::abstract_bits]
#[derive(Debug, Eq, PartialEq)]
pub struct Eui64(pub [u8; 8]);

impl Eui64 {
    pub fn from_hex(text: &str) -> Self {
        // Strip off colons and a 0x prefix, if present
        let text = text.replace(":", "").replace("0x", "");

        if text.len() != 16 {
            panic!("Invalid Eui64 length");
        }

        let mut eui64 = [0; 8];
        hex::decode_to_slice(text, &mut eui64).expect("Decoding failed");

        eui64.reverse();

        Self(eui64)
    }
}

#[abstract_bits::abstract_bits]
#[derive(Debug, Eq, PartialEq)]
pub struct Nwk(pub u16);

#[test]
fn test_nwk_route_reply_command() {
    let bytes: [u8; 23] = [
        48, 95, 55, 95, 10, 147, 3, 113, 56, 33, 5, 1, 136, 23, 0, 174, 211, 31, 11, 1, 136, 23, 0,
    ];

    dbg!(NwkRouteReplyCommand::MAX_BITS);
    dbg!(bytes.len());
    let command = NwkRouteReplyCommand::from_abstract_bits(&bytes).unwrap();

    assert_eq!(
        command,
        NwkRouteReplyCommand {
            route_request_identifier: 95,
            originator_nwk: Nwk(0x5F37),
            responder_nwk: Nwk(0x930A),
            path_cost: 3,
            originator_eui64: Some(dbg!(Eui64::from_hex("00:17:88:01:05:21:38:71"))),
            responder_eui64: Some(dbg!(Eui64::from_hex("00:17:88:01:0b:1f:d3:ae"))),
        }
    );

    assert_eq!(command.to_abstract_bits().unwrap(), &bytes);
}
