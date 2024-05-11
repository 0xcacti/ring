use clap::{crate_version, Parser};
use std::env;

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
struct App {
    /// The ip address or hostname to ping
    host: String,
}

#[tokio::main]
async fn main() {
    let args = App::parse();

    // handle commands
    println!("Pinging {}", args.host);
}
