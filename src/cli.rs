use clap::{crate_version, Parser};

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
pub struct CliArgs {
    #[arg(help = "The ip address or hostname to ping")]
    pub host: String,

    #[arg(
        short = 'a',
        long,
        default_value = "false",
        help = "Audio alert when a packet is received",
        action = clap::ArgAction::Set
    )]
    pub audio: bool,

    #[arg(short = 'c', long, help = "Number of packets to send")]
    pub count: Option<u16>,

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
    pub hop_limit: u8,

    #[arg(
        short = 'p',
        long,
        default_value = "true",
        help = "Include payload in ICMP packets",
        action = clap::ArgAction::Set

    )]
    pub include_payload: bool,
}
