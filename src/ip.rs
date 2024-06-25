use get_if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_machine_ipv4(destination: Ipv4Addr) -> Option<Ipv4Addr> {
    if destination.is_loopback() {
        return Some(Ipv4Addr::new(127, 0, 0, 1));
    }

    match get_if_addrs() {
        Ok(if_addrs) => {
            for if_addr in if_addrs {
                if let IpAddr::V4(ipv4_addr) = if_addr.addr.ip() {
                    if !ipv4_addr.is_loopback() {
                        return Some(ipv4_addr);
                    }
                }
            }
            return None;
        }
        Err(_) => panic!("Failed to get machine's IP address"), // TODO: should we panic here? or
    }
}

// TODO: learn how freaking IPv6 works
pub fn get_machine_ipv6(destination: Ipv6Addr) -> Option<Ipv6Addr> {
    if destination.is_loopback() {
        return Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    }

    match get_if_addrs() {
        Ok(if_addrs) => {
            for if_addr in if_addrs {
                if let IpAddr::V6(ipv6_addr) = if_addr.addr.ip() {
                    if !ipv6_addr.is_loopback() {
                        return Some(ipv6_addr);
                    }
                }
            }
            return None;
        }
        Err(_) => panic!("Failed to get machine's IP address"), // TODO: should we panic here? or
    }
}
