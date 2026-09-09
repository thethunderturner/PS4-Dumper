use crate::ftp::connect::connect;
use crate::ftp::list::{list_all, list_libraries};
use crate::scanning::discovery::discover_ps4s;

mod ftp;
mod scanning;

fn main() {
    let ps4s = discover_ps4s().expect("Scanning failed!");

    if ps4s.is_empty() {
        println!("No ps4s were found!")
    } else {
        println!("{} PS4s were found.", ps4s.len());
        // Take the first ps4
        let first = &ps4s[0];
        println!("Selecting: \n {:#?}", first);

        // Start TCP
        let mut ftp = connect(first.ip, 2121).expect("Couldn't connect");

        let entries = list_all(&mut ftp);
        let libs = list_libraries(&mut ftp);

        for entry in entries {
            println!("{entry}");
        }

        for lib in libs {
            println!("{lib}");
        }

        ftp.quit().expect("Couldn't close FTP connection");
    }
}
