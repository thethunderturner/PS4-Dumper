use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use crate::ftp::connect::connect;
use crate::scanning::discovery::discover_ps4s;

mod scanning;
mod ftp;

fn main() {
    let ps4s = discover_ps4s()
        .expect("Scanning failed!");

    // Take the first ps4
    println!("{} PS4s were found.", ps4s
        .iter()
        .filter(|console| { // This is a double check in case some other Sony device (except for PS4) also responded
            console.host_type == "PS4"
        })
        .count()
    );

    let first = &ps4s[0];
    println!("Selecting: \n {:#?}", first);

    // Start TCP
    connect(first.ip, 2121);
}