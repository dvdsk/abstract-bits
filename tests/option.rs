use abstract_bits::abstract_bits;

#[abstract_bits]
#[derive(Clone)]
struct Type {
    #[abstract_bits(presence_of = maybe_nothing)]
    is_maybe_nothing_present: bool,
    maybe_nothing: Option<u8>,
}
