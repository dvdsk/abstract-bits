Turn combinations of bit and byte fields into a structs, even if they represent
Options and Lists.

# Difference to bit-field crates
Let's jump directly into an example. You want to map a zigbee *Link Status
Command* to a high level rust struct. The frame's shape changes depending 
on some bits. 

This is the spec for the frame:
```txt
Bit: |  0 – 4      |      5      |    6       |    7     |       8 -
     | List length | First frame | Last frame | Reserved | Link status list
```

Each `link status` is:
```txt
Bit: |         0 – 15           |    16-18      |     19   |    20-22      |   23
     | Neighbor network address | Incoming cost | Reserved | Outgoing cost | Reserved
```

Its especially tricky that an earlier *bitfield* is determining the list length. Not even a hacky combination of `serde` and a `bitfield` crate can generate (de)-serialize code for us. 

Which is why we now have `abstract-bits`!
```rust
use abstract_bits::{abstract_bits, AbstractBits};

#[abstract_bits]
struct LinkStatusCommand {
    #[abstract_bits(length_of = link_statuses)]
    reserved: u5,
    is_first_frame: bool,
    is_last_frame: bool,
    reserved: u1,
    link_statuses: Vec<LinkStatus>,
}

#[abstract_bits]
struct LinkStatus {
    neighbor_address: u16,
    incoming_cost: u3,
    reserved: u1,
    outgoing_cost: u3,
    reserved: u1,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = LinkStatusCommand {
        is_first_frame: false,
        is_last_frame: true,
        link_statuses: Vec::new(),
    }.to_abstract_bits()?;
    let link_status_cmd = LinkStatusCommand::from_abstract_bits(&bytes)?;
    print!("number of links: {}", link_status_cmd.link_statuses.len());
    Ok(())
}
```

# Usage
## With a struct
- Add `#[abstract-bits]` above your struct and any *derives*.
- Use `u<n>` (`n` a natural number larger than zero) for numeric fields. In the
  transformed struct these will transform to the smallest rust primitives that
  can represent them. For example an `u7` will become an `u8`.
- Add padding (if needed) in between fields using `reserved = u<n>`.
- For each `Option` field place `#[abstract-bits(controls = <field_name>)]`
  above the `reserved: bool` fields which controls whether the `Option` is
  `Some` or `None`.
- For each `Vec` field place `#[abstract-bits(length_of = <field_name>)]`
  above the `reserved: u<n>` fields which controls the length of the `Vec`.

## With an enum
- Add `#[abstract-bits(bits = <N>)]` above your enum. Replace `N` with the
  number of bits the enum should occupy when serialized. Make sure any *derives*
  follow after.
- Explicitly assign every variant a value.
- Add a `#[repr(<Type>]` attribute, for example `#[repr(u8)]`.

# Complex example
```rust
use abstract_bits::{abstract_bits, AbstractBits, BitReader};

// The size of this is: 
// - 4+1+5+2+2|0+n*18, with n in range 0..u5::MAX 
// so this is at most 14 + 31*18  = 572 bits long
#[abstract_bits]
#[derive(Debug, PartialEq, Eq)] // note: derives follow after
struct Frame {
    header: u4,
    #[abstract_bits(controls = source)]
    reserved: bool,
    #[abstract_bits(length_of = data)]
    reserved: u5,
    frame_type: Type,
    source: Option<u16>,
    data: Vec<Message>,
}

/// This is: 4+3+1+10 = 18 bits long
#[abstract_bits]
#[derive(Debug, PartialEq, Eq)]
struct Message {
    header: u4,
    reserved: u3,
    is_important: bool,
    bits: [bool; 4]
}

#[abstract_bits(bits = 2)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Type {
    #[default]
    System = 0,
    Personal = 1,
    Group = 2,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = Frame {
        header: 12,
        frame_type: Type::default(),
        source: Some(4243),
        data: vec![Message {
            header: 9,
            is_important: false,
            bits: [true, false, true, true]
        }],
    }.to_abstract_bits()?;
    let mut reader = BitReader::from(bytes.as_slice());
    let mut frame = Frame::read_abstract_bits(&mut reader)?;
    if frame.frame_type == Type::default() {
        for message in &mut frame.data {
            message.is_important = true;
        }
    }
    let bytes = frame.to_abstract_bits();
    Ok(())
}
```

# Planned features
- `no-std` & `no-alloc` support (quite trivial)
- Support algebraic data-types other than Option (already supported)

# Possible features
- `HashMap`/`HashSet`/`BtreeMap` support

# Acknowledgements
This crate was inspired by [`bilge`](https://crates.io/crates/bilge) and
[`serde`](https://crates.io/crates/serde).
