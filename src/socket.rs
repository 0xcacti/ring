use std::{
    mem::MaybeUninit,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    time::{Duration, Instant},
};

use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::{IPV4Packet, IPV6Packet};

pub fn send_and_receive_ipv4_packet(
    packet: IPV4Packet,
    destination: IpAddr,
    audio: bool,
    timeout: u64,
) -> std::io::Result<()> {
    match destination {
        IpAddr::V6(_) => {
            panic!("must provide ipv4 address as destination");
        }
        _ => {}
    }

    let serialized_packet = packet.serialize();
    for byte in serialized_packet.iter() {
        print!("{:X} ", byte);
    }
    println!();

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.set_nonblocking(true)?;

    if packet.header.is_some() {
        socket.set_header_included(true).unwrap();
    }

    let sockaddr = SocketAddr::new(destination, 0);
    match socket.send_to(&serialized_packet, &sockaddr.into()) {
        Ok(bytes_sent) => println!("Sent {} bytes", bytes_sent),
        Err(e) => println!("Failed to send packet: {:?}", e),
    }

    let mut buf: [MaybeUninit<u8>; 1024] = [const { MaybeUninit::uninit() }; 1024];

    let timeout = Duration::from_millis(timeout); // Timeout for receiving
    let start = Instant::now();

    loop {
        if start.elapsed() > timeout {
            println!("Timeout reached, no response received.");
            break;
        }
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src_addr)) => {
                println!("Received {} bytes from {:?}", number_of_bytes, src_addr);
                let received_data = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const u8, number_of_bytes)
                };

                let packet = IPV4Packet::deserialize(&received_data);
                println!("{:?}", packet);
                break; // Exit after receiving the first response
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available yet, continue to try
                continue;
            }
            Err(e) => return Err(e), // Propagate unexpected errors
        }
    }

    Ok(())
}

pub fn send_and_receive_ipv6_packet(
    packet: IPV6Packet,
    destination: IpAddr,
) -> std::io::Result<()> {
    match destination {
        IpAddr::V4(_) => {
            panic!("must provide ipv6 address as destination");
        }
        _ => {}
    }

    let serialized_packet = packet.serialize();
    for byte in serialized_packet.iter() {
        print!("{:X} ", byte);
    }
    println!();

    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6))?;
    socket.set_nonblocking(true)?;
    socket.set_only_v6(true)?;
    socket.set_recv_tclass_v6(true)?;

    if packet.header.is_some() {
        socket.set_header_included(true).unwrap();
    }

    let sockaddr = SocketAddr::new(destination, 0);
    match socket.send_to(&serialized_packet, &sockaddr.into()) {
        Ok(bytes_sent) => println!("Sent {} bytes", bytes_sent),
        Err(e) => println!("Failed to send packet: {:?}", e),
    }

    let mut buf: [MaybeUninit<u8>; 1024] = [const { MaybeUninit::uninit() }; 1024];
    let timeout = Duration::from_secs(5); // Timeout for receiving
    let start = Instant::now();

    loop {
        if start.elapsed() > timeout {
            println!("Timeout reached, no response received.");
            break;
        }
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src_addr)) => {
                println!("Received {} bytes from {:?}", number_of_bytes, src_addr);
                println!("src addr for received data: {:?}", src_addr);
                let received_data = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const u8, number_of_bytes)
                };
                println!("Raw received data: {:X?}", received_data);
                println!("src addr for received data: {:?}", src_addr);
                let packet = IPV6Packet::deserialize(&received_data);
                println!("{:?}", packet);
                break; // Exit after receiving the first response
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available yet, continue to try
                continue;
            }
            Err(e) => return Err(e), // Propagate unexpected errors
        }
    }

    Ok(())
}
