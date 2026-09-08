use default_net::Interface;

pub fn find_interfaces() -> Vec<Interface> {

    // Ignore obvious virtual interfaces or interfaces w/o ipv4
    let interfaces: Vec<_> = default_net::get_interfaces()
        .into_iter()
        .filter(|interface| {
            interface.if_type != default_net::interface::InterfaceType::Loopback
                && !interface.ipv4.is_empty()
                && !interface.name.starts_with("docker")
                && !interface.name.starts_with("br-")
                && !interface.name.starts_with("veth")
                && !interface.name.starts_with("zt")
        })
        .collect();

    println!("{:#?}", interfaces);
    return interfaces
}

pub fn discover_ps4s() {
    println!("Scanning...");
    find_interfaces();
}