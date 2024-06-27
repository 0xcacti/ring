use clap::{crate_version, Parser};

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
pub struct App {
    #[arg(help = "The ip address or hostname to ping")]
    pub host: String,

    #[arg(
        short = 'a',
        long,
        default_value = "false",
        help = "Audio alert when a packet is received"
    )]
    pub audio: bool,

    #[arg(short = 'c', long, help = "Number of packets to send")]
    pub count: Option<usize>,

    #[arg(
        short = 'i',
        long,
        default_value = "1000",
        help = "Time to wait between sending each packet in milliseconds"
    )]
    pub interval: u64,

    #[arg(
        short = 't',
        long,
        default_value = "1000",
        help = "Time to wait for a response in milliseconds"
    )]
    pub timeout: u64,

    #[arg(short = 'T', long, default_value = "64", help = "TTL for IPV4 packets")]
    pub ttl: u8,

    #[arg(long = "id", help = "Header ID for ICMP packets")]
    pub id: Option<u16>,

    #[arg(
        short = 'H',
        long,
        default_value = "64",
        help = "Hop limit for IPV6 packets"
    )]
    pub ipv6: u8,
}
