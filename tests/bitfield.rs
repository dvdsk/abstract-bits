use abstract_bits::{AbstractBits, abstract_bits};

#[abstract_bits]
struct Register {
    device: u4,
    reserved: u1,
    on: bool,
    count: u2,
}

// #[abstract_bits]
// struct NormalStruct {
//     list: [bool; 5],
// }

// #[abstract_bits]
// struct UnitStruct([bool; 5]);

#[test]
fn main() {
    assert_eq!(Register::MIN_BITS, Register::MAX_BITS);
    assert_eq!(Register::MIN_BITS, 8);

    // assert_eq!(UnitStruct::MIN_BITS, UnitStruct::MAX_BITS);
    // assert_eq!(UnitStruct::MIN_BITS, 5);

    // assert_eq!(NormalStruct::MIN_BITS, NormalStruct::MAX_BITS);
    // assert_eq!(NormalStruct::MIN_BITS, 5);
}
