use crate::ftp::connect::connect;
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

        let entries = ftp::list::root(&mut ftp);
        let libs = ftp::list::libraries(&mut ftp, true);

        ftp::download::file(
            &mut ftp,
            &*libs[5],
            ".",
        ).expect("Download failed");

        ftp.quit().expect("Couldn't close FTP connection");
    }
}
