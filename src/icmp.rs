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
    pub checksum: u16,
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

fn get_machine_ipv4() -> Option<Ipv4Addr> {
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
                                                                // set to localhost?
    }
}

impl Packet {
    pub fn new_ipv4_echo_request(source_ip: IpAddr, destination_ip: IpAddr, id: u16) -> Packet {
        let icmp_id = rand::random::<u16>();

        let mut header = Header::new_ip_header(source_ip, destination_ip, id);
        header.compute_checksum();

        let mut icmp_header = ICMPHeader {
            msg_type: 8, // echo request
            code: 0,
            checksum: 0,
            id: icmp_id,
            seq_num: 1,
        };
        icmp_header.compute_icmp_checksum();

        Packet {
            header,
            icmp_header,
            icmp_payload: None,
        }
    }

    // pub struct Header {
    //     pub version: u8,
    //     pub ihl: u8,     // internet header length - in 32-bit words
    //     pub tos: u8,     // type of service
    //     pub length: u16, // in bytes
    //     pub id: u16,
    //     pub flags: u8,            // limit to 3 bits
    //     pub fragment_offset: u16, // limit to 13 bits
    //     pub ttl: u8,
    //     pub protocol: u8,
    //     pub checksum: u16,
    //     pub source: [u8; 4],
    //     pub destination: [u8; 4],
    // }
    //
    // pub struct ICMPHeader {
    //     pub msg_type: u8,
    //     pub code: u8,
    //     pub checksum: u16,
    //     pub id: u16,
    //     pub seq_num: u16,
    // }
    //
    // pub struct ICMPPayload {
    //     pub data: [u8; 32], // TODO: determine maximum size
    // }

    pub fn serialize(&self) -> Vec<u32> {
        let mut packet = Vec::new();
        let first_word = (self.header.version as u32) << 28
            | (self.header.ihl as u32) << 24
            | (self.header.tos as u32) << 16
            | self.header.length as u32;
        packet.push(first_word);
        println!("Packet: {:X}", packet[0]);

        let second_word = (self.header.id as u32) << 16
            | (self.header.flags as u32) << 13
            | self.header.fragment_offset as u32;

        packet.push(second_word);
        println!("Packet: {:X}", packet[1]);

        let third_word = (self.header.ttl as u32) << 24
            | (self.header.protocol as u32) << 16
            | self.header.checksum as u32;
        packet.push(third_word);
        println!("Packet: {:X}", packet[2]); // slight difference here

        let fourth_word = (self.header.source[0] as u32) << 24
            | (self.header.source[1] as u32) << 16
            | (self.header.source[2] as u32) << 8
            | self.header.source[3] as u32;
        packet.push(fourth_word);
        println!("Packet: {:X}", packet[3]);

        let fifth_word = (self.header.destination[0] as u32) << 24
            | (self.header.destination[1] as u32) << 16
            | (self.header.destination[2] as u32) << 8
            | self.header.destination[3] as u32;
        packet.push(fifth_word);
        println!("Packet: {:X}", packet[4]);

        let icmp_header_first_word = (self.icmp_header.msg_type as u32) << 24
            | (self.icmp_header.code as u32) << 16
            | self.icmp_header.checksum as u32;
        packet.push(icmp_header_first_word);
        println!("Packet: {:X}", packet[5]);

        let icmp_header_second_word =
            (self.icmp_header.id as u32) << 16 | self.icmp_header.seq_num as u32;
        packet.push(icmp_header_second_word);
        println!("Packet: {:X}", packet[6]);

        packet
    }
}

impl Header {
    fn new_ip_header(source_ip: IpAddr, destination_ip: IpAddr, id: u16) -> Header {
        // let process_id = process::id() as u16;

        let source = match source_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        let destination = match destination_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        Header {
            version: 4,
            ihl: 5,
            tos: 0,

            // len(Header) + len(ICMPHeader) + 0 (no payload)
            //     bytes: [ihl * 4(bytes)] + 2 * 4(bytes) + 32 * 4 + 0
            length: 28, // verify length
            id,
            // TODO: test working separate out for proper
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol: 1, // ICMP - 1
            checksum: 0,
            source,
            destination,
        }
    }

    fn compute_checksum(&mut self) {
        let mut sum: u32 = 0;
        sum += (self.version as u32) << 12
            | (self.ihl as u32) << 8
            | (self.tos as u32) + (self.length as u32);

        sum += self.id as u32 + ((self.flags as u32) << 13) | (self.fragment_offset as u32);
        sum += (self.ttl as u32) << 8 | (self.protocol as u32) + 0; // 0 term is header checksum
                                                                    //
        let source_term = ((self.source[0] as u32) << 8 | (self.source[1] as u32))
            + ((self.source[2] as u32) << 8 | (self.source[3] as u32));
        sum += source_term;

        let destination_term = ((self.destination[0] as u32) << 8 | (self.destination[1] as u32))
            + ((self.destination[2] as u32) << 8 | (self.destination[3] as u32));
        sum += destination_term;

        let carry_term = (sum >> 16) as u16;
        let truncated_sum = (sum & 0xFFFF) as u16;
        let pre_negation_cs = truncated_sum + carry_term;
        let checksum = 0xFFFF - pre_negation_cs;
        self.checksum = checksum;
    }
}

impl ICMPHeader {
    pub fn new_echo_request_header(id: u16, seq_num: u16) -> ICMPHeader {
        ICMPHeader {
            msg_type: 8,
            code: 0,
            checksum: 0,
            id,
            seq_num,
        }
    }
    fn compute_icmp_checksum(&mut self) {
        let msg_code_sum = (self.msg_type as u16) << 8 + self.code as u16;
        let id_seq_sum = self.id + self.seq_num;
        let subtotal = msg_code_sum + id_seq_sum;
        let checksum = u16::MAX - subtotal;
        self.checksum = checksum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_icmp_checksum() {
        let mut icmp_header = ICMPHeader::new_echo_request_header(0x1234, 0x001);
        icmp_header.compute_icmp_checksum();
        print!("{:X}", icmp_header.checksum);
        assert_eq!(icmp_header.checksum, 0xE5CA);
    }

    #[test]
    fn it_computes_ip_header_checksum() {
        let source = [10, 10, 10, 2];
        let destination = [10, 10, 10, 1];
        let id = 0xabcd;
        let mut header = Header {
            version: 4,
            ihl: 5,
            tos: 0,

            // len(Header) + len(ICMPHeader) + 0 (no payload)
            //     bytes: [ihl * 4(bytes)] + 2 * 4(bytes) + 32 * 4 + 0
            length: 40, // verify length
            id,
            // TODO: test working separate out for proper
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol: 6, // ICMP - 1
            checksum: 0,
            source,
            destination,
        };

        header.compute_checksum();
        assert_eq!(header.checksum, 0xa6ec);
    }

    #[test]
    fn it_serializes_packet() {
        let source = IpAddr::V4(Ipv4Addr::new(192, 168, 146, 131));
        let destination = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let id = 0xabcd;
        let mut packet = Packet::new_ipv4_echo_request(source, destination, id);
        // override defaults
        packet.icmp_header.id = 0x1234;
        packet.icmp_header.seq_num = 0x001;
        packet.icmp_header.compute_icmp_checksum();

        println!("{:X}", packet.icmp_header.checksum);
        let serialized_packet = packet.serialize();

        assert_eq!(serialized_packet[0], 0x4500001c);
        assert_eq!(serialized_packet[1], 0xabcd0000);
        assert_eq!(serialized_packet[2], 0x40016bd8);
        assert_eq!(serialized_packet[3], 0xc0a89283);
        assert_eq!(serialized_packet[4], 0x08080808);
        assert_eq!(serialized_packet[5], 0x0800e5ca);
        assert_eq!(serialized_packet[6], 0x12340001);
    }
}
