use clap::Parser;
use ring::{
    cli::CliArgs,
    icmp::{self, get_icmp_id},
    ip, socket,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{signal, sync::Mutex, time::sleep};

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
                println!("Ringing {} from {}", destination, source);
                ring_ipv6(source, destination, is_macos, icmp_id, args).await;
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
    let running = Arc::new(AtomicBool::new(true));
    let mut tasks = Vec::new();

    let running_clone = running.clone();
    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c event");
        running_clone.store(false, Ordering::SeqCst);
        println!("\nInterrupted. Finishing current pings and collecting stats...");
    });

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
        let running_task = running.clone();
        let task = tokio::spawn(async move {
            let start = Instant::now();
            match socket::send_and_receive_ipv4_packet(
                packet,
                destination,
                args.audio,
                args.timeout,
                &running_task,
            ) {
                Ok(_) => {
                    let elapsed = start.elapsed();
                    let mut stats = stats.lock().await;
                    stats.update_success(elapsed);
                }
                Err(_e) => {
                    let mut stats = stats.lock().await;
                    stats.update_failure();
                }
            }
        });
        tasks.push(task);
        let running_loop = running.clone();
        if running_loop.load(Ordering::SeqCst) == false {
            break;
        }

        sleep(Duration::from_millis(args.interval)).await;
    }

    for task in tasks {
        task.await.unwrap();
    }

    let mut final_stats = stats.lock().await;
    let avg_success_time = final_stats.calculate_avg_success_time();
    println!(
        "Success: {} Failure: {} - Avg Success Time: {}ms",
        final_stats.success,
        final_stats.failure,
        match avg_success_time {
            Some(duration) => duration.as_millis().to_string(),
            None => "N/A".to_string(),
        }
    );
}

async fn ring_ipv6(
    source: IpAddr,
    destination: IpAddr,
    is_macos: bool,
    icmp_id: u16,
    args: CliArgs,
) {
    let stats = Arc::new(Mutex::new(Stats::new()));
    let running = Arc::new(AtomicBool::new(true));
    let mut tasks = Vec::new();

    let running_clone = running.clone();
    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c event");
        running_clone.store(false, Ordering::SeqCst);
        println!("\nInterrupted. Finishing current pings and collecting stats...");
    });

    for i in 0..args.count.unwrap_or(u16::MAX) {
        let packet = if is_macos {
            icmp::IPV6Packet::new_echo_request(
                true,
                source,
                destination,
                icmp_id,
                args.ttl,
                args.include_payload,
                i,
            )
        } else {
            icmp::IPV6Packet::new_echo_request(
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
        let running_task = running.clone();
        let task = tokio::spawn(async move {
            let start = Instant::now();
            match socket::send_and_receive_ipv6_packet(
                packet,
                destination,
                args.audio,
                args.timeout,
                &running_task,
            ) {
                Ok(_) => {
                    let elapsed = start.elapsed();
                    let mut stats = stats.lock().await;
                    stats.update_success(elapsed);
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::Interrupted => {}
                    _ => {
                        let mut stats = stats.lock().await;
                        stats.update_failure();
                    }
                },
            }
        });
        tasks.push(task);
        let running_loop = running.clone();
        if running_loop.load(Ordering::SeqCst) == false {
            break;
        }

        sleep(Duration::from_millis(args.interval)).await;
    }

    for task in tasks {
        task.await.unwrap();
    }

    let mut final_stats = stats.lock().await;
    let avg_success_time = final_stats.calculate_avg_success_time();
    println!(
        "Success: {} Failure: {} - Avg Success Time: {}ms",
        final_stats.success,
        final_stats.failure,
        match avg_success_time {
            Some(duration) => duration.as_millis().to_string(),
            None => "N/A".to_string(),
        }
    );
}

#[derive(Debug)]
struct Stats {
    success: u32,
    failure: u32,
    total_success_time: Duration,
}

impl Stats {
    fn new() -> Self {
        Self {
            success: 0,
            failure: 0,
            total_success_time: Duration::from_millis(0),
        }
    }

    fn update_success(&mut self, time: Duration) {
        self.success += 1;
        self.total_success_time += time;
    }

    fn update_failure(&mut self) {
        self.failure += 1;
    }

    fn calculate_avg_success_time(&mut self) -> Option<Duration> {
        if self.success > 0 {
            Some(self.total_success_time / self.success)
        } else {
            None
        }
    }
}
