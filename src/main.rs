// https://github.com/kamalmarhubi/rust-intro-ngw
extern crate nix;

use nix::sys::socket::*;
use std::io::{self, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;

    let ip_addr = IpAddr::new_v4(94, 142, 241, 111);
    let port = 23;

    let sockaddr = SockAddr::new_inet(InetAddr::new(ip_addr, port));

    connect(sock, &sockaddr)?;

    let mut buf = [0u8; 1024];

    // recv returns the number of bytes it received and wrote into buf
    loop {
        let len = recv(sock, &mut buf, MsgFlags::empty())?;
        let new_bytes = &buf[..len];
        io::stdout().write(new_bytes);
    }

    // we want just the newly received bytes, we can take a len-long slice of buf:

    println!("Hello, world!");
    Ok(())
}
