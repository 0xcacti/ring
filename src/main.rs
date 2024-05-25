use clap::{crate_version, Parser};
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
                println!("{} is a valid ip address", ipv4);
                println!("Constructing a valid ICMP packet");
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
