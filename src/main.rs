use clap::{crate_version, Parser};
use ring::{
    cli::CliArgs,
    icmp::{self, get_icmp_id},
    ip, socket,
};
use std::{env, net::IpAddr};

use anyhow::Result;

fn main() {
    let args = CliArgs::parse();
    let is_macos = std::env::consts::OS == "macos";

    let destination_ip = match ip::resolve_host(&args.host) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Couldn't resolve host: {}", e);
            std::process::exit(1);
        }
    };

    let icmp_id = get_icmp_id(args.id);

    match destination_ip {
        IpAddr::V4(ipv4) => match ip::get_machine_ipv4(ipv4) {
            Some(source_ip) => {
                let source = IpAddr::V4(source_ip);
                let destination = IpAddr::V4(ipv4);
                ring_ipv4(source, destination, is_macos, icmp_id, args);
            }
            None => {
                eprintln!("Couldn't find a suitable IPv4 address. Please check your network configuration.");
                std::process::exit(1);
            }
        },
        IpAddr::V6(ipv6) => match ip::get_machine_ipv6(ipv6) {
            Some(source_ip) => {
                let source = IpAddr::V6(source_ip);
                let destination = IpAddr::V6(ipv6);
                // ADD RING IPV6
            }
            None => {
                eprintln!("Couldn't find a suitable IPv6 address. Please check your network configuration.");
                std::process::exit(1);
            }
        },
    };
}

fn ring_ipv4(source: IpAddr, destination: IpAddr, is_macos: bool, icmp_id: u16, args: CliArgs) {
    let mut i = 0;
    println!("args.count: {:?}", args.count);

    while args.count.map_or(true, |count| i < count) {
        let packet = if is_macos {
            icmp::IPV4Packet::new_echo_request(
                true,
                source,
                destination,
                icmp_id,
                args.ttl,
                args.include_payload,
                i,
            )
        } else {
            icmp::IPV4Packet::new_echo_request(
                false,
                source,
                destination,
                icmp_id,
                args.ttl,
                args.include_payload,
                i,
            )
        };
        socket::send_and_receive_ipv4_packet(packet, destination, args.audio, args.timeout)
            .unwrap();
        i += 1;
    }
}
