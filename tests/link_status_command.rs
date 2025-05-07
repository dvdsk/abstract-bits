use wire_format::abstract_bits;

// #[wire_format::abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
// pub struct Eui64(pub [u8; 8]);

// #[abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
pub struct NwkRouteRecordCommand {
    // #[wire_format(length_of = relays)]
    // reserved: u8,
    pub relays: Vec<Nwk>,
}

#[automatically_derived]
impl ::wire_format::AbstractBits for NwkRouteRecordCommand {
    fn needed_bits(&self) -> usize {
        todo!();
    }
    fn write_abstract_bits(
        &self,
        writer: &mut ::wire_format::BitWriter,
    ) -> Result<(), ::wire_format::ToBytesError> {
        let relays_len: u8 =
            self.relays
                .len()
                .try_into()
                .map_err(|_| ::wire_format::ToBytesError::ListTooLong {
                    ty: core::any::type_name::<Self>(),
                    max: u8::MAX as usize,
                    got: self.relays.len(),
                })?;
        ::wire_format::AbstractBits::write_abstract_bits(relays_len, writer)?;
        for element in &self.relays {
            ::wire_format::AbstractBits::write_abstract_bits(element, writer)?;
        }
        Ok(())
    }
    fn read_abstract_bits(
        reader: &mut ::wire_format::BitReader,
    ) -> Result<Self, ::wire_format::FromBytesError>
    where
        Self: Sized,
    {
        let relays_len = u8::read_abstract_bits(reader)?;
        let relays_len = relays_len as usize;
        let res = (0..relays_len)
            .into_iter()
            .map(|_| ::wire_format::AbstractBits::read_abstract_bits(reader))
            .collect::<Result<_, ::wire_format::FromBytesError>>();
        let relays = res?;
        Ok(Self { relays })
    }
}

// /// Zigbee spec compressed: 3.4.8.3
// #[abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
// pub struct NwkLinkStatusCommand {
//     #[wire_format(length_of = link_statuses)]
//     reserved: u5,
//     pub is_first_frame: bool,
//     pub is_last_frame: bool,
//     reserved: u1,
//     pub link_statuses: Vec<u8>, // edit for testing sake
// }

// /// Zigbee spec 3.4.1
// #[abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
// pub struct NwkRouteRequestCommand {
//     reserved: u3,
//     pub many_to_one: NwkRouteRequestManyToOne,
//     #[wire_format(controls = destination_eui64)]
//     reserved: bool,
//     reserved: u2,
//     pub route_request_identifier: u8,
//     pub destination_address: Nwk,
//     pub path_cost: u8,
//     pub destination_eui64: Option<Eui64>,
//     pub tlvs: Vec<u8>,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq)]
// #[wire_format::abstract_bits(bits=2)]
// #[repr(u8)]
// pub enum NwkRouteRequestManyToOne {
//     NotManyToOne = 0,
//     ManyToOneSenderSupportsRouteRecordTable = 1,
//     ManyToOneSenderDoesntSupportRouteRecordTable = 2,
//     Reserved = 3,
// }

// #[wire_format::abstract_bits]
// struct TestEnum {
//     reserved: u3,
//     thing: NwkRouteRequestManyToOne,
//     reserved: u5,
// }

// #[wire_format::abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
// pub struct NwkRouteReplyCommand {
//     reserved: u3,
//     #[wire_format(controls = originator_eui64)]
//     reserved: bool,
//     #[wire_format(controls = responder_eui64)]
//     reserved: bool,
//     reserved: u3,
//
//     pub multicast: bool,
//     pub route_request_identifier: u8,
//     pub originator_nwk: Nwk,
//     pub responder_nwk: Nwk,
//     pub path_cost: u8,
//     pub originator_eui64: Option<Eui64>,
//     pub responder_eui64: Option<Eui64>,
//     pub tlvs: Vec<u8>,
// }
//
// #[abstract_bits]
// #[derive(Debug, Clone, PartialEq)]
// pub struct Eui64(u64);

// /// Zigbee spec compressed: 3.4.8.3
// #[abstract_bits]
// // #[derive(Debug, Clone, PartialEq)]
// pub struct NwkLinkStatusCommand {
//     reserved: u4,
//     pub is_first_frame: bool,
//     pub is_last_frame: bool,
//     reserved: u1,
//     pub link_statuses: Vec<NwkLinkStatus>,
// }
//
// #[abstract_bits]
// // #[derive(Debug, Clone, PartialEq)]
// pub struct NwkLinkStatus {
//     address: Nwk,
//     incoming_cost: u3,
//     reserved: u1,
//     outgoing_cost: u3,
//     reserved: u1,
// }

#[abstract_bits]
#[derive(Debug, Clone, PartialEq)]
pub struct Nwk(pub u16);

// #[abstract_bits]
// struct Test {
//     list: Vec<u8>,
// }

// #[test]
// fn main() {
//
// }
