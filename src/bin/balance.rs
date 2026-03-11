//! # balance
//!
//! CLI tool for checking your SMSDev account's current SMS credit balance.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin balance -- --key YOUR_KEY
//! ```

use std::process;
use smsdev::SmsDev;

fn parse_args() -> Result<String, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut key = None;
    let mut i = 0;

    while i < raw.len() {
        match raw[i].as_str() {
            "--key" => {
                i += 1;
                key = Some(raw.get(i).ok_or("--key requires a value")?.clone());
            }
            "--help" | "-h" => {
                eprintln!("Usage: balance --key KEY");
                process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
        i += 1;
    }

    key.ok_or("--key is required".into())
}

#[tokio::main]
async fn main() {
    let key = match parse_args() {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Run with --help for usage.");
            process::exit(1);
        }
    };

    let client = SmsDev::new(&key);

    match client.balance().await {
        Ok(bal) => {
            if bal.is_ok() {
                let credits = bal.balance_as_u64().unwrap_or(0);
                println!("SMS credits available: {}", credits);
                println!("Status: {}", bal.description);
            } else {
                eprintln!("API error: {}", bal.description);
                process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
