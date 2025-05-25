use abstract_bits::{AbstractBits, abstract_bits};

#[abstract_bits]
struct LinkStatusCommand {
    #[abstract_bits(length_of = link_statuses)]
    reserved: u5,
    is_first_frame: bool,
    is_last_frame: bool,
    reserved: u1,
    link_statuses: Vec<u8>,
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = LinkStatusCommand {
        is_first_frame: false,
        is_last_frame: true,
        link_statuses: Vec::new(),
    }
    .to_abstract_bits()?;
    let link_status_cmd = LinkStatusCommand::from_abstract_bits(&bytes)?;
    print!("number of links: {}", link_status_cmd.link_statuses.len());
    Ok(())
}
