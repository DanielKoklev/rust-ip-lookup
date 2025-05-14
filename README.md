# Rust IP Lookup and Subdomain Scanner

A Rust application for performing IP lookups and scanning subdomains asynchronously using Tokio and `trust-dns-resolver`.

## Features

- **IP Lookup**: Resolve domain names to IP addresses.
- **Subdomain Scanning**: Scan for subdomains using a provided list.
- **Asynchronous Processing**: Utilize Tokio for asynchronous DNS lookups.
- **Command-Line Interface**: Use `clap` for parsing command-line arguments.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd <repository-directory>

## Usage
Command-Line Arguments

    -d, --domain: The domain to scan (required).
    -f, --file: The file containing the list of subdomains (optional).
