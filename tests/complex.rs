use abstract_bits::abstract_bits;

#[abstract_bits]
struct Frame {
    header: u4,
    #[abstract_bits(controls = source)]
    reserved: bool,
    #[abstract_bits(length_of = data)]
    reserved: u5,
    ty: Type,
    source: Option<u16>,
    data: Vec<Message>,
}

/// This is: 4+3+1+10 = 18 bits long
#[abstract_bits]
struct Message {
    header: u4,
    reserved: u3,
    is_important: bool,
    bits: [bool; 10],
}

#[abstract_bits(bits = 2)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Type {
    System = 0,
    #[default]
    Personal = 1,
    Group = 2,
}
