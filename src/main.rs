use crate::ftp::connect::connect;
use crate::scanning::discovery::discover_ps4s;

mod ftp;
mod scanning;

fn main() {
    let ps4s = discover_ps4s().expect("Scanning failed!");

    if ps4s.is_empty() {
        println!("No ps4s were found!")
    } else {
        println!("{} PS4s were found.", ps4s.iter().count());
        // Take the first ps4
        let first = &ps4s[0];
        println!("Selecting: \n {:#?}", first);

        // Start TCP
        connect(first.ip, 2121);
    }
}
