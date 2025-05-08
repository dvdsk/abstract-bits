use abstract_bits::abstract_bits;

#[abstract_bits]
struct Register {
    device: u4,
    reserved: u1,
    on: bool,
    count: u2,
}
