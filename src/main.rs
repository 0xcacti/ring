use clap::{crate_version, Parser};
use ring::{icmp, ip, socket};
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
struct App {
    /// The ip address or hostname to ping
    host: String,
}

fn main() {
    let args = App::parse();

    match args.host.parse::<IpAddr>() {
        Ok(ip) => match ip {
            IpAddr::V4(ipv4) => {
                let ipv4_source = ip::get_machine_ipv4().unwrap();
                println!("source ip: {}", ipv4_source);
                let destination_ip = IpAddr::V4(ipv4);
                let packet = icmp::Packet::new_ipv4_echo_request(
                    IpAddr::V4(ipv4_source),
                    destination_ip,
                    0x26f2,
                );
                println!("ip header checksum: {:X}", packet.header.checksum);
                println!("icmp header checksum: {:X}", packet.icmp_header.checksum);
                println!("total packet length: {}", packet.header.length);
                socket::send_ipv4_packet(packet, ipv4_source, ipv4).unwrap();
            }
            IpAddr::V6(ipv6) => {
                println!("{} is a valid ip address", ipv6);
                let source_ip = IpAddr::V6(ip::get_machine_ipv6().unwrap());
                println!("source ip: {}", source_ip);
                let destination_ip = IpAddr::V6(ipv6);
                let packet = icmp::Packet::new_ipv6_echo_request(source_ip, destination_ip, 0xabcd);
                println!("ip header checksum: {:X}", packet.header.checksum);
                println!("icmp header checksum: {:X}", packet.icmp_header.checksum);
                println!("total packet length: {}", packet.header.length);
                socket::send_ipv6_packet(packet, ipv6).unwrap();
            }
        },
        Err(_) => {
            eprintln!("{} is not a valid ip address", args.host);
            std::process::exit(1);
        }
    }
}
