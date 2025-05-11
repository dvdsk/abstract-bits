use abstract_bits::abstract_bits;

#[abstract_bits(bits = 2)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Type {
    System = 0,
    #[default]
    Personal = 1,
    Group = 2,
}
