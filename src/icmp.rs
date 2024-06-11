use get_if_addrs::get_if_addrs;
use std::{
    net::{IpAddr, Ipv4Addr},
    process,
};

// TODO add encapsulation
pub struct Header {
    pub version: u8,
    pub ihl: u8,     // internet header length - in 32-bit words
    pub tos: u8,     // type of service
    pub length: u16, // in bytes
    pub id: u16,
    pub flags: u8,            // limit to 3 bits
    pub fragment_offset: u16, // limit to 13 bits
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub source: [u8; 4],
    pub destination: [u8; 4],
}

pub struct ICMPHeader {
    pub msg_type: u8,
    pub code: u8,
    pub icmp_checksum: u16,
    pub id: u16,
    pub seq_num: u16,
}

pub struct ICMPPayload {
    pub data: [u8; 32], // TODO: determine maximum size
}

pub struct Packet {
    pub header: Header,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: Option<ICMPPayload>,
}

fn get_machine_ipv4_ip() -> Option<Ipv4Addr> {
    match get_if_addrs() {
        Ok(if_addrs) => {
            for if_addr in if_addrs {
                if let IpAddr::V4(ipv4_addr) = if_addr.addr.ip() {
                    if !ipv4_addr.is_loopback() {
                        return Some(ipv4_addr);
                    }
                }
            }
        }
        Err(_) => panic!("Failed to get machine's IP address"), // TODO: should we panic here? or
                                                                // set to localhost?
    }
}

impl Packet {
    pub fn new_ipv4_echo_request(destination_ip: IpAddr) -> Packet {
        let dest = match destination_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        let process_id = process::id() as u16;

        Packet {
            header: Header {
                version: 4,
                ihl: 5,
                tos: 0,

                // len(Header) + len(ICMPHeader) + 0 (no payload)
                //     bytes: [ihl * 4(bytes)] + 2 * 4(bytes) + 32 * 4 + 0
                length: 28,
                id: process_id,
                flags: 0,
                fragment_offset: 0,
                ttl: 64,
                protocol: 1, // ICMP
                checksum: 0,
                source: [127, 0, 0, 1],
                destination: dest,
            },
            icmp_header: ICMPHeader {
                msg_type: 8, // echo request
                code: 0,
                icmp_checksum: 0,
                id: process_id,
                seq_num: 0,
            },
            icmp_payload: None,
        }
    }
}
