use clap::{crate_version, Parser, Subcommand};
use std::{env, process};

#[derive(Debug, Parser)]
#[command(name="ring", version=crate_version!(), about="ping in rust", long_about = "rust implementation of the classic util ping", arg_required_else_help(true))]
struct App {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
}

#[tokio::main]
async fn main() {
    let args = App::parse();

    // handle commands
    match &args.command {
        Some(Commands::Ping) => {
            println!("Pinging...")
        }

        None => {
            eprintln!("No command provided");
            process::exit(1);
        }
    }
}
