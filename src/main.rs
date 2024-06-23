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

    match args.host.parse::<IpAddr>() {
        Ok(destination_ip) => match is_macos {
            true => {
                ring_from_macos(destination_ip).unwrap();
            }
            false => {
                ring_from_linux(destination_ip).unwrap();
            }
        },
        Err(_) => {
            eprintln!("{} is not a valid ip address", args.host);
            std::process::exit(1);
        }
    }
}

fn ring_from_macos(destination_ip: IpAddr) -> Result<()> {
    match destination_ip {
        IpAddr::V4(ipv4) => {
            let source_ip = ip::get_machine_ipv4(ipv4).unwrap();
            println!("source ip: {}", source_ip);
            let source = IpAddr::V4(source_ip);
            let destination = IpAddr::V4(ipv4);
            let packet = icmp::Packet::new_ipv4_echo_request(true, source, destination, 0x26f2);
            socket::send_and_receive_ipv4_packet(packet, destination).unwrap();
        }
        IpAddr::V6(ipv6) => {}
    }
    Ok(())
}

fn ring_from_linux(destination_ip: IpAddr) -> Result<()> {
    match destination_ip {
        IpAddr::V4(ipv4) => {
            let source_ip = ip::get_machine_ipv4(ipv4).unwrap();
            println!("source ip: {}", source_ip);
            let source = IpAddr::V4(source_ip);
            let destination = IpAddr::V4(ipv4);
            let packet = icmp::Packet::new_ipv4_echo_request(false, source, destination, 0x26f2);
            socket::send_and_receive_ipv4_packet(packet, destination).unwrap();
        }
        IpAddr::V6(ipv6) => {}
    }
    Ok(())
}
