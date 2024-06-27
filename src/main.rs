use clap::{crate_version, Parser};
use ring::{
    cli::CliArgs,
    icmp::{self, get_icmp_id},
    ip, socket,
};
use std::{
    env,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::Mutex, time::sleep};

use anyhow::Result;

#[tokio::main]
async fn main() {
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
                println!("Ringing {} from {}", destination, source);
                ring_ipv4(source, destination, is_macos, icmp_id, args).await;
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

async fn ring_ipv4(
    source: IpAddr,
    destination: IpAddr,
    is_macos: bool,
    icmp_id: u16,
    args: CliArgs,
) {
    let stats = Arc::new(Mutex::new(Stats::new()));
    let mut tasks = Vec::new();

    for i in 0..args.count.unwrap_or(u16::MAX) {
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
        let stats = stats.clone();
        let destination = destination;
        let task = tokio::spawn(async move {
            let start = Instant::now();
            match socket::send_and_receive_ipv4_packet(
                packet,
                destination,
                args.audio,
                args.timeout,
            ) {
                Ok(_) => {
                    let elapsed = start.elapsed();
                    let mut stats = stats.lock().await;
                    stats.update_success(elapsed);
                }
                Err(e) => {
                    eprintln!("Ping failed: {:?}", e);
                    let mut stats = stats.lock().await;
                    stats.update_failure();
                }
            }
        });
        tasks.push(task);
        sleep(Duration::from_millis(args.interval)).await;
    }

    for task in tasks {
        task.await.unwrap();
    }

    let final_stats = stats.lock().await;
    // Print final statistics here
}

#[derive(Debug)]
struct Stats {
    success: u32,
    failure: u32,
    total_success_time: Duration,
    avg_success_time: Duration,
}

impl Stats {
    fn new() -> Self {
        Self {
            success: 0,
            failure: 0,
            total_success_time: Duration::from_millis(0),
            avg_success_time: Duration::from_millis(0),
        }
    }

    fn update_success(&mut self, time: Duration) {
        self.success += 1;
        self.avg_success_time += time;
    }

    fn update_failure(&mut self) {
        self.failure += 1;
    }

    fn calculate_avg_success_time(&mut self) -> Option<Duration> {
        if self.success > 0 {
            Some(self.avg_success_time / self.success)
        } else {
            None
        }
    }
}
