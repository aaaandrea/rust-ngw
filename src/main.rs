extern crate nix;

use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};

fn main() -> Result<(), Box<std::error::Error>> {
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;
    println!("Hello, world!");
    Ok(())
}
