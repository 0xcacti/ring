use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::Packet;

pub fn send_ipv4_packet(
    packet: Packet,
    source: Ipv4Addr,
    destination: Ipv4Addr,
) -> std::io::Result<()> {
    let packet = packet.serialize_ipv4();
    for byte in packet.iter() {
        print!("{:X} ", byte);
    }
    println!();

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.set_header_included(true).unwrap();
    // socket.bind(&SocketAddr::new(IpAddr::V4(source), 0).into())?;
    // socket.set_nonblocking(true)?;

    let sockaddr = SocketAddr::new(IpAddr::V4(destination), 0);
    match socket.send_to(&packet, &sockaddr.into()) {
        Ok(bytes_sent) => println!("Sent {} bytes", bytes_sent),
        Err(e) => println!("Failed to send packet: {:?}", e),
    }

    Ok(())
}

pub fn send_ipv6_packet(packet: Packet, destination: Ipv6Addr) -> std::io::Result<()> {
    unimplemented!()
}
