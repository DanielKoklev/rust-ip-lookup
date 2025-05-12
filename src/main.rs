use std::io;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use text_to_ascii_art::to_art;

fn main() {

    match to_art("T0r7uga Services".to_string(), "small", 4, 1, 1) {
      Ok(string) => println!("{}", string),
      Err(err) => println!("Error: {}", err),
    }

    println!("Enter a website (e.g., example.org):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let domain = input.trim();

    if domain.is_empty() {
        println!("No domain provided.");
        return;
    }

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .unwrap();

    match resolver.lookup_ip(domain) {
        Ok(ip_addresses) => {
            for ip in ip_addresses.iter() {
                println!("IP address: {}", ip);
            }
        }
        Err(e) => {
            println!("Error looking up IP address: {}", e);
        }
    }
    println!("IP lookup completed.");
    println!("Exiting program.");
}