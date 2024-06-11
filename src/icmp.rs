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
        }
    }

    fn compute_checksum(&mut self) {
        let mut sum: u32 = 0;
        sum += (self.version as u32) << 12
            | (self.ihl as u32) << 8
            | (self.tos as u32) + (self.length as u32);

        println!("Sum after first term: {:X}", sum);
        sum += self.id as u32 + ((self.flags as u32) << 13) | (self.fragment_offset as u32);
        println!(
            "addition term for id: {:X}",
            self.id as u32 + (self.flags as u32) << 13 | (self.fragment_offset as u32)
        );
        println!("Sum after id: {:X}", sum);
        sum += (self.ttl as u32) << 8 | (self.protocol as u32) + 0; // 0 term is header checksum
        println!(
            "addition term for ttl: {:X}",
            (self.ttl as u32) << 8 | (self.protocol as u32)
        );

        println!("Sum after protocol: {:X}", sum);
        let source_term = ((self.source[0] as u32) << 8 | (self.source[1] as u32))
            + ((self.source[2] as u32) << 8 | (self.source[3] as u32));

        sum += source_term;
        println!(
            "self.source_01: {:X}",
            (self.source[0] as u32) << 8 | self.source[1] as u32
        );
        println!(
            "self.source_23: {:X}",
            (self.source[2] as u32) << 8 | self.source[3] as u32
        );

        println!("source_term: {:X}", source_term);

        println!("Sum after source: {:X}", sum);

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
        let source = IpAddr::V4(Ipv4Addr::new(10, 10, 10, 2));
        let destination = IpAddr::V4(Ipv4Addr::new(10, 10, 10, 1));
        let mut header = Header::new_ip_header(source, destination, 0xabcd);
        header.compute_checksum();
        assert_eq!(header.checksum, 0xa6ec);
    }
}
