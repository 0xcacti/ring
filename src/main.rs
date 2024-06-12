use clap::{crate_version, Parser};
use ring::{icmp, ip, socket};
use std::{env, net::IpAddr};

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
struct App {
    /// The ip address or hostname to ping
    host: String,
}

#[tokio::main]
async fn main() {
    let args = App::parse();

    match args.host.parse::<IpAddr>() {
        Ok(ip) => match ip {
            IpAddr::V4(ipv4) => {
                println!("PING {}", ipv4);
                let source_ip = IpAddr::V4(ip::get_machine_ipv4().unwrap());
                let destination_ip = IpAddr::V4(ipv4);
                let packet = icmp::Packet::new_ipv4_echo_request(source_ip, destination_ip, 0x1234);
                socket::send_ipv4_packet(packet, ipv4).unwrap();
            }
            IpAddr::V6(ipv6) => {
                println!("{} is a valid ip address", ipv6);
            }
        },
        Err(_) => {
            eprintln!("{} is not a valid ip address", args.host);
            std::process::exit(1);
        }
    }
}
