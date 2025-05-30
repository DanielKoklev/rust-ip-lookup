use std::fs::File;
use std::io::{self, BufRead};
use std::net::{SocketAddr};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use text_to_ascii_art::to_art;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Domain to scan
    #[clap(short, long)]
    domain: String,

    /// File containing the list of subdomains (optional)
    #[clap(short, long)]
    file: Option<String>,
    
    /// Scan open ports of a domain
    #[clap(short = 's', long = "open-ports")]
    scan_ports: bool,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    match to_art("T0r7uga Services".to_string(), "small", 4, 1, 1) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    let domain = options.domain;

    if domain.is_empty() {
        println!("No domain provided.");
        return;
    }

    // Create a DNS resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // Lookup IP addresses for the main domain
    match resolver.lookup_ip(&domain).await {
        Ok(ip_addresses) => {
            for ip in ip_addresses {
                println!("IP address found {} for domain {}", ip, domain);
                let start_port: u16 = 1;
                let end_port: u16 = 1024;
                scan_ports(ip, start_port, end_port).await;
            }
        }
        Err(e) => {
            println!("Error looking up IP address: {}", e);
            return;
        }
    }

    // Check if a subdomains file is provided
    if let Some(subdomains_file) = options.file {
        let resolver = Arc::new(resolver);
        let mut tasks = vec![];

        if let Ok(lines) = read_lines(&subdomains_file) {
            println!("\nScanning for subdomains...");
            for line in lines {
                if let Ok(subdomain) = line {
                    let resolver = Arc::clone(&resolver);
                    let domain = domain.clone(); // Clone the domain for each task

                    // Spawn an asynchronous task for each subdomain lookup
                    let task = tokio::spawn(async move {
                        let subdomain_domain = format!("{}.{}", subdomain, domain);
                        match resolver.lookup_ip(&subdomain_domain).await {
                            Ok(ip_addresses) => {
                                for ip in ip_addresses {
                                    println!("Subdomain: {} -> IP address: {}", subdomain_domain, ip);
                                }
                            }
                            Err(_) => {
                                println!("No IP address found for subdomain: {}", subdomain_domain);
                            }
                        }
                    });

                    tasks.push(task);
                }
            }

            // Wait for all tasks to complete
            for task in tasks {
                task.await.unwrap();
            }
        } else {
            println!("Could not read subdomains file.");
        }
    }

    println!("IP lookup completed.");
    println!("Exiting program.");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn scan_ports(ip: std::net::IpAddr, start_port: u16, end_port: u16) {
    for port in start_port..=end_port {
        if scan_port(ip, port).await {
            println!("Port {} is open!!", port);
        }
    }
}

async fn scan_port(ip: std::net::IpAddr, port: u16) -> bool {
    let socket_addr = SocketAddr::new(ip, port);
    match tokio::time::timeout(Duration::from_millis(100), tokio::net::TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => true,
        Ok(Err(_)) => false,
        Err(_) => false,
    }
}
