use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use suppaftp::{FtpResult, FtpStream};

pub fn connect(ip: Ipv4Addr, port: u16) -> FtpResult<FtpStream> {
    let address = SocketAddr::from((ip, port));

    println!("Connecting to FTP at {address}...");

    match FtpStream::connect_timeout(address, Duration::from_secs(5)) {
        Ok(mut ftp) => {
            println!("Connected successfully!");
            ftp.login("", "")?; // Can take empty params
            Ok(ftp)
        }

        Err(error) => {
            println!("Could not connect to {address} within 5 seconds: {error}");
            Err(error)
        }
    }
}
