use std::net::{IpAddr, Ipv4Addr};

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

    pub fn serialize(&self) -> Vec<u8> {
        let mut packet = Vec::new();
        // TODO: add asserts
        packet.push(self.header.version << 4 | self.header.ihl);
        packet.push(self.header.tos);
        packet.extend_from_slice(&self.header.length.to_be_bytes());
        packet.extend_from_slice(&self.header.id.to_be_bytes());
        packet.extend_from_slice(
            &(((self.header.flags as u16) << 13) | self.header.fragment_offset).to_be_bytes(),
        );
        packet.push(self.header.ttl);
        packet.push(self.header.protocol);
        packet.extend_from_slice(&self.header.checksum.to_be_bytes());
        packet.extend_from_slice(&self.header.source);
        packet.extend_from_slice(&self.header.destination);
        packet.push(self.icmp_header.msg_type);
        packet.push(self.icmp_header.code);
        packet.extend_from_slice(&self.icmp_header.checksum.to_be_bytes());
        packet.extend_from_slice(&self.icmp_header.id.to_be_bytes());
        packet.extend_from_slice(&self.icmp_header.seq_num.to_be_bytes());

        if let Some(payload) = &self.icmp_payload {
            packet.extend_from_slice(&payload.data);
        }

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
            length: 40,
            id,
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol: 6,
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

        let correct_packet_str = "4500001cabcd000040016bd8c0a89283080808080800e5ca12340001";
        let correct_packet: Vec<u8> = correct_packet_str
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let byte_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(byte_str, 16).unwrap()
            })
            .collect();

        let serialized_packet = packet.serialize();
        assert_eq!(correct_packet, serialized_packet);
    }
}
