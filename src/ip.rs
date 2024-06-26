use get_if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_machine_ipv4(destination: Ipv4Addr) -> Option<Ipv4Addr> {
    if destination.is_loopback() {
        return Some(Ipv4Addr::LOCALHOST);
    }
    get_if_addrs().ok().and_then(|if_addrs| {
        if_addrs
            .into_iter()
            .filter_map(|if_addr| {
                if let IpAddr::V4(ipv4_addr) = if_addr.addr.ip() {
                    if !ipv4_addr.is_loopback() && !ipv4_addr.is_link_local() {
                        Some(ipv4_addr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .next()
    })
}

pub fn get_machine_ipv6(destination: Ipv6Addr) -> Option<Ipv6Addr> {
    if destination.is_loopback() {
        return Some(Ipv6Addr::LOCALHOST);
    }

    match get_if_addrs() {
        Ok(if_addrs) => {
            let mut suitable_addr = None;
            for if_addr in if_addrs {
                if let IpAddr::V6(ipv6_addr) = if_addr.addr.ip() {
                    let suitable = is_suitable_ipv6(&ipv6_addr);
                    println!(
                        "Interface: {}, IPv6: {}, Suitable: {}",
                        if_addr.name, ipv6_addr, suitable
                    );
                    if suitable && suitable_addr.is_none() {
                        suitable_addr = Some(ipv6_addr);
                    }
                }
            }
            if suitable_addr.is_none() {
                println!("No suitable IPv6 address found");
            }
            suitable_addr
        }
        Err(e) => {
            println!("Error getting interface addresses: {:?}", e);
            None
        }
    }
}

fn is_suitable_ipv6(addr: &Ipv6Addr) -> bool {
    !addr.is_loopback() && !is_link_local(addr) && !addr.is_unspecified()
}

fn is_link_local(addr: &Ipv6Addr) -> bool {
    addr.segments()[0] & 0xffc0 == 0xfe80
}
