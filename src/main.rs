use clap::{crate_version, Parser};
use ring::{icmp, ip, socket};
use std::{env, net::IpAddr};

use anyhow::Result;

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
struct App {
    /// The ip address or hostname to ping
    host: String,
}

fn main() {
    let args = App::parse();
    let is_macos = std::env::consts::OS == "macos"; // Auto-detect macOS

    let destination_ip = match ip::resolve_host(&args.host) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Couldn't resolve host: {}", e);
            std::process::exit(1);
        }
    };

    match destination_ip {
        IpAddr::V4(ipv4) => {
            let source_ip = ip::get_machine_ipv4(ipv4).unwrap();
            println!("source ip: {}", source_ip);
            let source = IpAddr::V4(source_ip);
            let destination = IpAddr::V4(ipv4);
            let packet = if is_macos {
                icmp::IPV4Packet::new_echo_request(true, source, destination, 0x26f2)
            } else {
                icmp::IPV4Packet::new_echo_request(false, source, destination, 0x26f2)
            };

            socket::send_and_receive_ipv4_packet(packet, destination).unwrap();
        }
        IpAddr::V6(ipv6) => match ip::get_machine_ipv6(ipv6) {
            Some(source_ip) => {
                println!("destination ip: {}", ipv6);
                println!("source ip: {}", source_ip);
                let source = IpAddr::V6(source_ip);
                let destination = IpAddr::V6(ipv6);
                let packet = if is_macos {
                    icmp::IPV6Packet::new_echo_request(true, source, destination)
                } else {
                    icmp::IPV6Packet::new_echo_request(false, source, destination)
                };
                socket::send_and_receive_ipv6_packet(packet, destination).unwrap();
            }
            None => {
                eprintln!("Couldn't find a suitable IPv6 address. Please check your network configuration.");
                std::process::exit(1);
            }
        },
    };
}
