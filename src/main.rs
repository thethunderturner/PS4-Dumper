use crate::ftp::connect::connect;
use crate::scanning::discovery::discover_ps4s;

mod ftp;
mod scanning;
pub mod title;

fn main() {
    let ps4s = discover_ps4s().expect("Scanning failed!");

    if ps4s.is_empty() {
        println!("No PS4s were found!");
        return;
    }

    println!("{} PS4s were found.", ps4s.len());

    // Take the first PS4
    let first = &ps4s[0];
    println!("Selecting:\n{:#?}", first);

    // Start FTP
    let mut ftp = match connect(first.ip, 2121) {
        Ok(ftp) => ftp,

        Err(error) => {
            println!("Could not connect to FTP: {error}");
            println!("Make sure GoldHEN FTP is enabled.");
            return;
        }
    };

    // Detect titles
    let titles = title::detect::current(&mut ftp);
    if titles.is_empty() {
        println!("No running titles were detected.");
        return;
    }

    println!("{} title(s) detected:", titles.len());
    println!("{titles:#?}");

    // SFO stuff
    // let _bytes = title::sfo::sfo::read_sfo();
}
