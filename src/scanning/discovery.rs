use default_net::Interface;
use std::{
    io,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

pub fn find_interfaces() -> Vec<Interface> {

    // Ignore loopback, obvious virtual interfaces and interfaces w/o ipv4
    default_net::get_interfaces()
        .into_iter()
        .filter(|interface| {
            interface.if_type != default_net::interface::InterfaceType::Loopback
                && !interface.ipv4.is_empty()
                && !interface.name.starts_with("docker")
                && !interface.name.starts_with("br-")
                && !interface.name.starts_with("veth")
                && !interface.name.starts_with("zt")
        })
        .collect()
}

// Read https://www.psdevwiki.com/ps4/PlayStation_4_Discovery_and_Wake-up_Utility
pub fn discover_ps4s() -> io::Result<()> {
    let interfaces = find_interfaces();

    let message =
        "SRCH * HTTP/1.1\n\
        device-discovery-protocol-version:00020020\n";

    for interface in interfaces {
        for ipv4 in interface.ipv4 {
            let local_ip = ipv4.addr;
            let broadcast_ip = ipv4.broadcast();

            println!(
                "Scanning {}: {} -> {}:987",
                interface.name,
                local_ip,
                broadcast_ip
            );

            let socket = UdpSocket::bind((local_ip, 0))?;

            socket.set_broadcast(true)?;

            socket.set_read_timeout(Some(
                Duration::from_secs(2)
            ))?;

            let destination =
                SocketAddr::from((broadcast_ip, 987));

            socket.send_to(
                message.as_bytes(),
                destination,
            )?;

            let mut buffer = [0u8; 2048];

            match socket.recv_from(&mut buffer) {
                Ok((size, sender)) => {
                    let response =
                        String::from_utf8_lossy(&buffer[..size]);

                    println!("Response from {sender}:");
                    println!("{response}");
                }

                Err(error)
                if error.kind() == io::ErrorKind::WouldBlock
                    || error.kind() == io::ErrorKind::TimedOut =>
                    {
                        println!(
                            "No response on {}",
                            interface.name
                        );
                    }

                Err(error) => return Err(error),
            }
        }
    }

    Ok(())
}