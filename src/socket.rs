use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::Packet;

pub fn send_ipv4_packet(packet: Packet, destination: Ipv4Addr) -> std::io::Result<()> {
    let packet = packet.serialize();

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.set_nonblocking(true)?;

    let sockaddr = SocketAddr::new(IpAddr::V4(destination), 0);
    socket.send_to(&packet, &sockaddr.into())?;

    Ok(())
}
