use crate::ftp::connect::connect;
use crate::scanning::discovery::discover_ps4s;

mod ftp;
mod scanning;
pub mod title;

fn main() {
    // let ps4s = discover_ps4s().expect("Scanning failed!");
    //
    // if ps4s.is_empty() {
    //     println!("No ps4s were found!")
    // } else {
    //     println!("{} PS4s were found.", ps4s.len());
    //     // Take the first ps4
    //     let first = &ps4s[0];
    //     println!("Selecting: \n {:#?}", first);
    //
    //     // Start TCP
    //     let mut ftp = match connect(first.ip, 2121) {
    //         Ok(ftp) => ftp,
    //
    //         Err(error) => {
    //             println!("Could not connect to FTP: {error}");
    //             println!("Make sure GoldHEN FTP is enabled.");
    //             return;
    //         }
    //     };
    //
    //     let _title = title::detect::current(&mut ftp);
    // }

    // sfo stuff
    let _bytes = title::sfo::sfo::read_sfo();
}
