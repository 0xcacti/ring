use get_if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_machine_ipv4() -> Option<Ipv4Addr> {
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

pub fn get_machine_ipv6() -> Option<Ipv6Addr> {
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
