use default_net::Interface;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::Duration,
};

#[derive(Debug)]
pub struct PS4 {
    pub ip: Ipv4Addr, // e.g. 10.42.0.1
    pub host_id: String, // e.g. 20C6711DH141
    pub host_type: String, // e.g. PS4
    pub host_name: String, // e.g. PS4-900
    pub host_request_port: u16, // e.g. 987
    pub device_discovery_protocol_version: String, // e.g. 00020020
    pub system_version: String, // e.g. 09510001
}

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

fn get_header(response: &str, name: &str) -> Option<String> {
    response.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        if key.eq_ignore_ascii_case(name) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn parse_sony_device(response: &str, ip: Ipv4Addr) -> Option<PS4> {
    Some(PS4 {
        ip,
        host_id: get_header(response, "host-id")?,
        host_type: get_header(response, "host-type")?,
        host_name: get_header(response, "host-name")?,
        host_request_port: get_header(response, "host-request-port")?
            .parse()
            .ok()?,
        device_discovery_protocol_version: get_header(response, "device-discovery-protocol-version")?,
        system_version: get_header(response, "system-version")?,
    })
}

// Read https://www.psdevwiki.com/ps4/PlayStation_4_Discovery_and_Wake-up_Utility
pub fn discover_ps4s() -> io::Result<Vec<PS4>> {
    let interfaces = find_interfaces();

    let message =
        "SRCH * HTTP/1.1\n\
        device-discovery-protocol-version:00020020\n";

    let mut ps4s = Vec::new();

    for interface in interfaces {
        for ipv4 in interface.ipv4 {
            let local_ip = ipv4.addr;
            let broadcast_ip = ipv4.broadcast();

            let socket = UdpSocket::bind((local_ip, 0))?;

            socket.set_broadcast(true)?;
            socket.set_read_timeout(Some(Duration::from_secs(2)))?;

            let destination =
                SocketAddr::from((broadcast_ip, 987));

            socket.send_to(
                message.as_bytes(),
                destination,
            )?;

            let mut buffer = [0u8; 2048];

            loop {
                match socket.recv_from(&mut buffer) {
                    Ok((size, sender)) => {
                        let response =
                            String::from_utf8_lossy(
                                &buffer[..size]
                            );

                        if let IpAddr::V4(ip) = sender.ip() {
                            if let Some(ps4) =
                                parse_sony_device(&response, ip)
                            {
                                if ps4.host_type == "PS4" {
                                    ps4s.push(ps4);
                                }
                            }
                        }
                    }

                    Err(error)
                    if error.kind()
                        == io::ErrorKind::WouldBlock
                        || error.kind()
                        == io::ErrorKind::TimedOut =>
                        {
                            break;
                        }

                    Err(error) => return Err(error),
                }
            }
        }
    }

    Ok(ps4s)
}