use default_net::Interface;
use std::io;
use std::net::{SocketAddr, UdpSocket};

pub fn find_interfaces() -> Vec<Interface> {
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

            socket.send_to(
                message.as_bytes(),
                SocketAddr::from((broadcast_ip, 987)),
            )?;
        }
    }

    Ok(())
}