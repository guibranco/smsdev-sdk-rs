//! # cancel_sms
//!
//! CLI tool for cancelling one or more scheduled SMS messages by their IDs.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin cancel_sms -- --key YOUR_KEY --id 637849052
//!
//! # Cancel multiple messages at once
//! cargo run --bin cancel_sms -- --key YOUR_KEY --id 637849052 --id 637849053
//! ```

use std::process;
use smsdev::SmsDev;

fn parse_args() -> Result<(String, Vec<u64>), String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut key = None;
    let mut ids: Vec<u64> = Vec::new();
    let mut i = 0;

    while i < raw.len() {
        match raw[i].as_str() {
            "--key" => {
                i += 1;
                key = Some(raw.get(i).ok_or("--key requires a value")?.clone());
            }
            "--id" => {
                i += 1;
                let id: u64 = raw
                    .get(i)
                    .ok_or("--id requires a value")?
                    .parse()
                    .map_err(|_| "--id must be a numeric message ID")?;
                ids.push(id);
            }
            "--help" | "-h" => {
                eprintln!("Usage: cancel_sms --key KEY --id ID [--id ID ...]");
                process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
        i += 1;
    }

    let key = key.ok_or("--key is required")?;
    if ids.is_empty() {
        return Err("At least one --id is required".into());
    }

    Ok((key, ids))
}

#[tokio::main]
async fn main() {
    let (key, ids) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("Run with --help for usage.");
            process::exit(1);
        }
    };

    let client = SmsDev::new(&key);

    println!("Cancelling {} message(s)...", ids.len());

    match client.cancel(ids).await {
        Ok(results) => {
            let mut all_ok = true;
            for r in &results {
                if r.is_ok() {
                    println!("✓  id={} — {}", r.id, r.description);
                } else {
                    eprintln!("✗  id={} code={} — {}", r.id, r.code, r.description);
                    all_ok = false;
                }
            }
            if !all_ok {
                process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
