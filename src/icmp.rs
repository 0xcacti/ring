// TODO add encapsulation
pub struct Header {
    pub version: u8,
    pub tos: u8, // type of service
    pub length: u16,
    pub id: u16,
    pub flags_and_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub source: u32,
    pub destination: u32,
}

pub struct ICMPHeader {
    pub msg_type: u8,
    pub code: u8,
    pub icmp_checksum: u16,
    pub header_data: u32,
}

pub struct ICMPPayload {
    pub data: [u8; 32], // TODO: determine maximum size 
}

pub struct Packet {
    pub header: Header,
    pub icmp_header: ICMPHeader,
    pub icmp_payload: ICMPPayload,
}

impl Packet {
    pub fn new_echo_request() -> Packet {
        Packet {
            header: Header {
                version: 1, 
                tos: 0,


            }
        }
    }
}
