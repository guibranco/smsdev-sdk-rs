//! # report
//!
//! CLI tool for fetching a total usage report from the SMSDev API,
//! optionally filtered by a date range.
//!
//! ## Usage
//!
//! ```bash
//! # Full account history
//! cargo run --bin report -- --key YOUR_KEY
//!
//! # Filtered by date range
//! cargo run --bin report -- --key YOUR_KEY --from 01/01/2025 --to 31/01/2025
//! ```

use std::process;
use smsdev::{SmsDev, models::ReportRequest};

struct Args {
    key: String,
    date_from: Option<String>,
    date_to: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut key = None;
    let mut date_from = None;
    let mut date_to = None;
    let mut i = 0;

    while i < raw.len() {
        match raw[i].as_str() {
            "--key" => {
                i += 1;
                key = Some(raw.get(i).ok_or("--key requires a value")?.clone());
            }
            "--from" => {
                i += 1;
                date_from = Some(raw.get(i).ok_or("--from requires a value (DD/MM/YYYY)")?.clone());
            }
            "--to" => {
                i += 1;
                date_to = Some(raw.get(i).ok_or("--to requires a value (DD/MM/YYYY)")?.clone());
            }
            "--help" | "-h" => {
                eprintln!("Usage: report --key KEY [--from DD/MM/YYYY] [--to DD/MM/YYYY]");
                process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
        i += 1;
    }

    Ok(Args {
        key: key.ok_or("--key is required")?,
        date_from,
        date_to,
    })
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

    let mut req = ReportRequest::new(&args.key);
    if let Some(from) = args.date_from {
        req = req.date_from(from);
    }
    if let Some(to) = args.date_to {
        req = req.date_to(to);
    }

    match client.report(req).await {
        Ok(r) => {
            if !r.is_ok() {
                eprintln!("API error: {}", r.description);
                process::exit(2);
            }

            let period = match (r.data_inicio.as_deref(), r.data_fim.as_deref()) {
                (Some(from), Some(to)) => format!("{} → {}", from, to),
                (Some(from), None) => format!("from {}", from),
                (None, Some(to)) => format!("up to {}", to),
                (None, None) => "all time".to_string(),
            };

            println!("══════════════════════════════════════");
            println!("  SMSDev Usage Report");
            println!("  Period : {}", period);
            println!("══════════════════════════════════════");
            println!("  Sent       : {:>10}", r.sent);
            println!("  Received   : {:>10}", r.received);
            println!("  Blacklist  : {:>10}", r.blacklist);
            println!("  Cancelled  : {:>10}", r.cancelled);
            println!("──────────────────────────────────────");
            println!("  Credits used: {:>9}", r.credits_used);
            println!("══════════════════════════════════════");
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
