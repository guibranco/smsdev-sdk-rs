//! # inbox
//!
//! CLI tool for querying received (MO) SMS messages from the SMSDev inbox.
//!
//! ## Usage
//!
//! ```bash
//! # Fetch only new (unread) messages
//! cargo run --bin inbox -- --key YOUR_KEY
//!
//! # Fetch all messages in a date range
//! cargo run --bin inbox -- --key YOUR_KEY --all --from 01/01/2025 --to 31/01/2025
//!
//! # Filter by specific sent-message IDs
//! cargo run --bin inbox -- --key YOUR_KEY --id 637849052 --id 637849053
//! ```

use std::process;
use smsdev::{SmsDev, models::InboxRequest};

#[derive(Default)]
struct Args {
    key: String,
    all: bool,
    date_from: Option<String>,
    date_to: Option<String>,
    ids: Vec<u64>,
}

fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut args = Args::default();
    let mut key = None;
    let mut i = 0;

    while i < raw.len() {
        match raw[i].as_str() {
            "--key" => {
                i += 1;
                key = Some(raw.get(i).ok_or("--key requires a value")?.clone());
            }
            "--all" => {
                args.all = true;
            }
            "--from" => {
                i += 1;
                args.date_from = Some(raw.get(i).ok_or("--from requires a value")?.clone());
            }
            "--to" => {
                i += 1;
                args.date_to = Some(raw.get(i).ok_or("--to requires a value")?.clone());
            }
            "--id" => {
                i += 1;
                let id: u64 = raw
                    .get(i)
                    .ok_or("--id requires a value")?
                    .parse()
                    .map_err(|_| "--id must be numeric")?;
                args.ids.push(id);
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: inbox --key KEY [--all] [--from DD/MM/YYYY] [--to DD/MM/YYYY] [--id ID ...]"
                );
                process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
        i += 1;
    }

    args.key = key.ok_or("--key is required")?;
    Ok(args)
}

#[tokio::main]
async fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Run with --help for usage.");
            process::exit(1);
        }
    };

    let client = SmsDev::new(&args.key);

    let mut req = InboxRequest::new(&args.key);
    if args.all {
        req = req.all();
    }
    if let Some(from) = args.date_from {
        req = req.date_from(from);
    }
    if let Some(to) = args.date_to {
        req = req.date_to(to);
    }
    if !args.ids.is_empty() {
        req = req.filter_ids(args.ids);
    }

    match client.inbox(req).await {
        Ok(messages) => {
            if messages.is_empty() {
                println!("No messages found.");
                return;
            }
            println!("{} message(s) received:\n", messages.len());
            println!(
                "{:<22} {:<18} {:<12} Message",
                "Date", "From", "ID (MO)"
            );
            println!("{}", "-".repeat(80));
            for msg in &messages {
                println!(
                    "{:<22} {:<18} {:<12} {}",
                    msg.data_read,
                    msg.phone,
                    msg.id_sms_read,
                    msg.description
                );
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
