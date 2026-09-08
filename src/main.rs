use crate::scanning::discovery::discover_ps4s;

mod scanning;
mod ftp;

fn main() {
    let ps4s = discover_ps4s()
        .expect("Scanning failed!");

    // Take the first ps4
    let ps4 = &ps4s[0];
    let ip = ps4.ip;
    println!("{:#?}", ps4);
}