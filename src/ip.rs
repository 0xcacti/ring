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

// Helper function to check if an IPv6 address is link-local
fn is_link_local(addr: &Ipv6Addr) -> bool {
    addr.segments()[0] & 0xffc0 == 0xfe80
}

pub fn get_machine_ipv6(destination: Ipv6Addr) -> Option<Ipv6Addr> {
    if destination.is_loopback() {
        return Some(Ipv6Addr::LOCALHOST);
    }
    get_if_addrs().ok().and_then(|if_addrs| {
        if_addrs
            .into_iter()
            .filter_map(|if_addr| {
                if let IpAddr::V6(ipv6_addr) = if_addr.addr.ip() {
                    if !ipv6_addr.is_loopback()
                        && !is_link_local(&ipv6_addr)
                        && ipv6_addr.is_global()
                    {
                        Some(ipv6_addr)
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

