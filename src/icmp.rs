use std::net::{IpAddr, Ipv4Addr};

// TODO add encapsulation
pub struct HeaderIPV4 {
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

pub struct HeaderIPV6 {
    pub version: u8,       // 4 bits
    pub traffic_class: u8, // 8 bits
    pub flow_label: u32,   // 20 bits
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source: [u8; 16],
    pub destination: [u8; 16],
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

pub struct IPV4Packet {
    pub header: Option<HeaderIPV4>,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: Option<ICMPPayload>,
}

pub struct IPV6Packet {
    pub header: Option<HeaderIPV6>,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: Option<ICMPPayload>,
}

impl IPV4Packet {
    pub fn new_echo_request(
        is_macos: bool,
        source_ip: IpAddr,
        destination_ip: IpAddr,
        id: u16,
    ) -> IPV4Packet {
        if is_macos {
            let mut icmp_header = ICMPHeader {
                msg_type: 8, // echo request
                code: 0,
                checksum: 0,
                id: 1234,
                seq_num: 1,
            };
            icmp_header.compute_icmp_checksum();
            IPV4Packet {
                header: None,
                icmp_header,
                icmp_payload: None,
            }
        } else {
            let mut header = HeaderIPV4::new_ip_header(source_ip, destination_ip, id);
            header.compute_checksum();
            let mut icmp_header = ICMPHeader {
                msg_type: 8, // echo request
                code: 0,
                checksum: 0,
                id: 1234,
                seq_num: 1,
            };
            icmp_header.compute_icmp_checksum();
            IPV4Packet {
                header: Some(header),
                icmp_header,
                icmp_payload: None,
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_packet = Vec::new();
        if let Some(ref header) = self.header {
            serialized_packet.push(header.version << 4 | header.ihl);
            serialized_packet.push(header.tos);
            serialized_packet.extend_from_slice(&header.length.to_be_bytes());
            serialized_packet.extend_from_slice(&header.id.to_be_bytes());
            serialized_packet.extend_from_slice(
                &(((header.flags as u16) << 13) | header.fragment_offset).to_be_bytes(),
            );
            serialized_packet.push(header.ttl);
            serialized_packet.push(header.protocol);
            serialized_packet.extend_from_slice(&header.checksum.to_be_bytes());
            serialized_packet.extend_from_slice(&header.source);
            serialized_packet.extend_from_slice(&header.destination);
        }

        serialized_packet.push(self.icmp_header.msg_type);
        serialized_packet.push(self.icmp_header.code);
        serialized_packet.extend_from_slice(&self.icmp_header.checksum.to_be_bytes());
        serialized_packet.extend_from_slice(&self.icmp_header.id.to_be_bytes());
        serialized_packet.extend_from_slice(&self.icmp_header.seq_num.to_be_bytes());

        if let Some(payload) = &self.icmp_payload {
            serialized_packet.extend_from_slice(&payload.data);
        }

        serialized_packet
    }
}

impl IPV6Packet {
    pub fn new_echo_request(
        is_macos: bool,
        source_ip: IpAddr,
        destination_ip: IpAddr,
    ) -> IPV6Packet {
        if is_macos {
            let mut icmp_header = ICMPHeader {
                msg_type: 128, // echo request
                code: 0,
                checksum: 0,
                id: 1234,
                seq_num: 1,
            };
            icmp_header.compute_icmp_checksum();
            IPV6Packet {
                header: None,
                icmp_header,
                icmp_payload: None,
            }
        } else {
            let header = HeaderIPV6::new_ip_header(source_ip, destination_ip);
            let mut icmp_header = ICMPHeader {
                msg_type: 128, // echo request
                code: 0,
                checksum: 0,
                id: 1234,
                seq_num: 1,
            };
            icmp_header.compute_icmp_checksum();
            IPV6Packet {
                header: Some(header),
                icmp_header,
                icmp_payload: None,
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_packet = Vec::new();
        if let Some(ref header) = self.header {
            let version_tc_fl = ((header.version as u32) << 28)
                | ((header.traffic_class as u32) << 20)
                | (header.flow_label & 0x000FFFFF); // Mask to ensure only 20 bits
            serialized_packet.extend_from_slice(&version_tc_fl.to_be_bytes());
            serialized_packet.extend_from_slice(&header.payload_length.to_be_bytes());
            serialized_packet.push(header.next_header);
            serialized_packet.push(header.hop_limit);
            serialized_packet.extend_from_slice(&header.source);
            serialized_packet.extend_from_slice(&header.destination);
        }

        serialized_packet.push(self.icmp_header.msg_type);
        serialized_packet.push(self.icmp_header.code);
        serialized_packet.extend_from_slice(&self.icmp_header.checksum.to_be_bytes());
        serialized_packet.extend_from_slice(&self.icmp_header.id.to_be_bytes());
        serialized_packet.extend_from_slice(&self.icmp_header.seq_num.to_be_bytes());

        if let Some(payload) = &self.icmp_payload {
            serialized_packet.extend_from_slice(&payload.data);
        }

        serialized_packet
    }
}

impl HeaderIPV4 {
    fn new_ip_header(source_ip: IpAddr, destination_ip: IpAddr, id: u16) -> HeaderIPV4 {
        // let process_id = process::id() as u16;

        let source = match source_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        let destination = match destination_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        HeaderIPV4 {
            version: 4,
            ihl: 5,
            tos: 0,
            // len(Header) + len(ICMPHeader) + 0 (no payload)
            //     bytes: [ihl * 4(bytes)] + 2 * 4(bytes) + 32 * 4 + 0
            length: 28,
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

impl HeaderIPV6 {
    fn new_ip_header(source_ip: IpAddr, destination_ip: IpAddr) -> HeaderIPV6 {
        let source = match source_ip {
            IpAddr::V6(addr) => addr.octets(),
            _ => panic!("Only IPv6 is supported"),
        };

        let destination = match destination_ip {
            IpAddr::V6(addr) => addr.octets(),
            _ => panic!("Only IPv6 is supported"),
        };

        HeaderIPV6 {
            version: 6,
            traffic_class: 0, // set to 0
            flow_label: 0,    // defaults to 0
            payload_length: 8,
            next_header: 58,
            hop_limit: 64, // normal value
            source,
            destination,
        }
    }
}

impl ICMPHeader {
    pub fn new_echo_request_header(msg_type: u8, id: u16, seq_num: u16) -> ICMPHeader {
        ICMPHeader {
            msg_type,
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
        // ipv4
        let mut icmp_header = ICMPHeader::new_echo_request_header(8, 0x1234, 0x001);
        icmp_header.compute_icmp_checksum();
        print!("{:X}", icmp_header.checksum);
        assert_eq!(icmp_header.checksum, 0xE5CA);

        // ipv6
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
    fn it_serializes_icp4_packet() {
        let source = IpAddr::V4(Ipv4Addr::new(192, 168, 146, 131));
        let destination = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let id = 0xabcd;
        let mut packet = IPV4Packet::new_echo_request(false, source, destination, id);

        match packet {
            Packet::Linux(ref mut packet_data) => {
                // override defaults
                packet_data.icmp_header.id = 0x1234;
                packet_data.icmp_header.seq_num = 0x001;
                packet_data.icmp_header.compute_icmp_checksum();
            }
            _ => panic!("Expected Linux packet"),
        }

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
