use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use suppaftp::{FtpResult, FtpStream};

pub fn connect(ip: Ipv4Addr, port: u16) {
    let address = SocketAddr::from((ip, port));

    println!("Connecting to FTP at {address}...");

    match TcpStream::connect_timeout(
        &address,
        Duration::from_secs(5),
    ) {
        Ok(stream) => {
            println!("Connected successfully!");

            if let Err(error) = stream.shutdown(Shutdown::Both) {
                println!("Failed to close connection cleanly: {error}");
            } else {
                println!("Connection closed.");
            }
        }

        Err(error) => {
            println!(
                "Could not connect to {address} within 5 seconds: {error}"
            );
        }
    }
}