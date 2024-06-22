use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::Packet;

pub fn send_ipv4_packet(packet: Packet, destination: IpAddr) -> std::io::Result<()> {
    match destination {
        IpAddr::V6(_) => {
            panic!("must provide ipv4 address as destination");
        }
        _ => {}
    }

    let packet = packet.serialize_ipv4();
    for byte in packet.iter() {
        print!("{:X} ", byte);
    }
    println!();

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.set_header_included(true).unwrap();
    socket.set_nonblocking(true)?;

    let sockaddr = SocketAddr::new(destination, 0);
    match socket.send_to(&packet, &sockaddr.into()) {
        Ok(bytes_sent) => println!("Sent {} bytes", bytes_sent),
        Err(e) => println!("Failed to send packet: {:?}", e),
    }

    Ok(())
}

pub fn send_ipv6_packet(packet: Packet, destination: Ipv6Addr) -> std::io::Result<()> {
    unimplemented!()
}
