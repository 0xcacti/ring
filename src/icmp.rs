use crate::{cli::CliArgs, error::ICMPError};
use std::{
    net::{IpAddr, Ipv4Addr},
    process,
};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ICMPHeader {
    pub msg_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub id: u16,
    pub seq_num: u16,
}

#[derive(Debug)]
pub struct ICMPPayload {
    pub data: [u8; 32],
}

#[derive(Debug)]
pub struct IPV4Packet {
    pub header: Option<HeaderIPV4>,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: Option<ICMPPayload>,
}

#[derive(Debug)]
pub struct IPV6Packet {
    pub header: Option<HeaderIPV6>,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: Option<ICMPPayload>,
}

fn get_random_header_id() -> u16 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..u16::MAX)
}

pub fn get_icmp_id(desired_id: Option<u16>) -> u16 {
    match desired_id {
        Some(id) => id,
        None => (process::id() % 0xFFFF) as u16,
    }
}

impl IPV4Packet {
    pub fn new_echo_request(
        is_macos: bool,
        source_ip: IpAddr,
        destination_ip: IpAddr,
        icmp_id: u16,
        ttl: u8,
        include_payload: bool,
        seq_num: u16,
    ) -> IPV4Packet {
        let payload = if include_payload {
            Some(ICMPPayload::new_random_payload())
        } else {
            None
        };

        if is_macos {
            let mut icmp_header = ICMPHeader {
                msg_type: 8, // echo request
                code: 0,
                checksum: 0,
                id: icmp_id,
                seq_num,
            };
            if let Some(ref payload) = payload {
                icmp_header.compute_icmp_checksum(Some(&payload.data));
            } else {
                icmp_header.compute_icmp_checksum(None);
            }

            IPV4Packet {
                header: None,
                icmp_header,
                icmp_payload: payload,
            }
        } else {
            let mut header =
                HeaderIPV4::new_ip_header(source_ip, destination_ip, ttl, include_payload);
            header.compute_checksum();
            let mut icmp_header = ICMPHeader {
                msg_type: 8, // echo request
                code: 0,
                checksum: 0,
                id: icmp_id,
                seq_num,
            };
            if let Some(ref payload) = payload {
                icmp_header.compute_icmp_checksum(Some(&payload.data));
            } else {
                icmp_header.compute_icmp_checksum(None);
            }
            IPV4Packet {
                header: Some(header),
                icmp_header,
                icmp_payload: payload,
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

    pub fn deserialize(data: &[u8]) -> Result<IPV4Packet, ICMPError> {
        if data.len() < 28 {
            return Err(ICMPError::new("Packet too short. Invalid".to_string()));
        }

        let header = HeaderIPV4 {
            version: data[0] >> 4,
            ihl: data[0] & 0x0F,
            tos: data[1],
            length: u16::from_be_bytes([data[2], data[3]]),
            id: u16::from_be_bytes([data[4], data[5]]),
            flags: (data[6] >> 5) as u8,
            fragment_offset: u16::from_be_bytes([data[6] & 0x1F, data[7]]),
            ttl: data[8],
            protocol: data[9],
            checksum: u16::from_be_bytes([data[10], data[11]]),
            source: [data[12], data[13], data[14], data[15]],
            destination: [data[16], data[17], data[18], data[19]],
        };

        let icmp_header = ICMPHeader {
            msg_type: data[20],
            code: data[21],
            checksum: u16::from_be_bytes([data[22], data[23]]),
            id: u16::from_be_bytes([data[24], data[25]]),
            seq_num: u16::from_be_bytes([data[26], data[27]]),
        };

        let icmp_payload = if data.len() > 28 {
            let mut payload_data = [0; 32];
            let payload_len = std::cmp::min(data.len() - 28, 32);
            payload_data[..payload_len].copy_from_slice(&data[28..28 + payload_len]);
            Some(ICMPPayload { data: payload_data })
        } else {
            None
        };

        Ok(IPV4Packet {
            header: Some(header),
            icmp_header,
            icmp_payload,
        })
    }
}

impl IPV6Packet {
    pub fn new_echo_request(
        is_macos: bool,
        source_ip: IpAddr,
        destination_ip: IpAddr,
        icmp_id: u16,
        hop_limit: u8,
        include_payload: bool,
        seq_num: u16,
    ) -> IPV6Packet {
        let payload = if include_payload {
            Some(ICMPPayload::new_random_payload())
        } else {
            None
        };

        if is_macos {
            let mut icmp_header = ICMPHeader {
                msg_type: 128, // echo request
                code: 0,
                checksum: 0,
                id: icmp_id,
                seq_num,
            };
            if let Some(ref payload) = payload {
                icmp_header.compute_icmp_checksum(Some(&payload.data));
            } else {
                icmp_header.compute_icmp_checksum(None);
            }
            IPV6Packet {
                header: None,
                icmp_header,
                icmp_payload: payload,
            }
        } else {
            let header = HeaderIPV6::new_ip_header(source_ip, destination_ip, hop_limit);
            let mut icmp_header = ICMPHeader {
                msg_type: 128, // echo request
                code: 0,
                checksum: 0,
                id: icmp_id,
                seq_num,
            };
            if let Some(ref payload) = payload {
                icmp_header.compute_icmp_checksum(Some(&payload.data));
            } else {
                icmp_header.compute_icmp_checksum(None);
            }
            IPV6Packet {
                header: Some(header),
                icmp_header,
                icmp_payload: payload,
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

    pub fn deserialize(data: &[u8]) -> Result<IPV6Packet, ICMPError> {
        let icmp_header = ICMPHeader {
            msg_type: data[0],
            code: data[1],
            checksum: u16::from_be_bytes([data[2], data[3]]),
            id: u16::from_be_bytes([data[4], data[5]]),
            seq_num: u16::from_be_bytes([data[6], data[7]]),
        };

        let icmp_payload = if data.len() > 8 {
            let mut payload_data = [0u8; 32];
            let payload_len = std::cmp::min(data.len() - 8, 32);
            payload_data[..payload_len].copy_from_slice(&data[8..8 + payload_len]);
            Some(ICMPPayload { data: payload_data })
        } else {
            None
        };

        Ok(IPV6Packet {
            header: None,
            icmp_header,
            icmp_payload,
        })
    }
}

impl HeaderIPV4 {
    fn new_ip_header(
        source_ip: IpAddr,
        destination_ip: IpAddr,
        ttl: u8,
        include_payload: bool,
    ) -> HeaderIPV4 {
        let source = match source_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        let destination = match destination_ip {
            IpAddr::V4(addr) => addr.octets(),
            _ => panic!("Only IPv4 is supported"),
        };

        let header_id = get_random_header_id();

        let length: u16 = if include_payload { 60 } else { 28 };

        HeaderIPV4 {
            version: 4,
            ihl: 5,
            tos: 0,
            // len(Header) + len(ICMPHeader) + 0 (no payload)
            //     bytes: [ihl * 4(bytes)] + 2 * 4(bytes) + 32 * 4 + 0
            length: length as u16,
            id: header_id,
            flags: 0,
            fragment_offset: 0,
            ttl,
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
    fn new_ip_header(source_ip: IpAddr, destination_ip: IpAddr, hop_limit: u8) -> HeaderIPV6 {
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
            traffic_class: 0,
            flow_label: 0,
            payload_length: 8,
            next_header: 58,
            hop_limit,
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

    fn compute_icmp_checksum(&mut self, payload: Option<&[u8]>) {
        let mut sum: u32 = 0;
        sum += (self.msg_type as u32) << 8 | (self.code as u32);
        sum += self.id as u32 + self.seq_num as u32;

        if let Some(data) = payload {
            let mut i = 0;
            while i < data.len() {
                if i + 1 < data.len() {
                    sum += (data[i] as u32) << 8 | (data[i + 1] as u32);
                } else {
                    sum += (data[i] as u32) << 8;
                }
                i += 2;
            }
        }

        while (sum >> 16) > 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        self.checksum = !(sum as u16);
    }
}

impl ICMPPayload {
    pub fn new_random_payload() -> ICMPPayload {
        let mut payload_data = [0u8; 32];
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.fill(&mut payload_data);
        ICMPPayload { data: payload_data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_icmp_checksum() {
        let mut icmp_header = ICMPHeader::new_echo_request_header(8, 0x1234, 0x001);
        icmp_header.compute_icmp_checksum(None);
        assert_eq!(icmp_header.checksum, 0xE5CA);
    }

    #[test]
    fn it_computes_ip_header_checksum() {
        let source = [10, 10, 10, 2];
        let destination = [10, 10, 10, 1];
        let id = 0xabcd;
        let mut header = HeaderIPV4 {
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
        let mut packet =
            IPV4Packet::new_echo_request(false, source, destination, 0x1234, 64, false, 0x001);
        if let Some(ref mut header) = packet.header {
            header.id = id;
            header.compute_checksum();
        }
        packet.icmp_header.compute_icmp_checksum(None);

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
