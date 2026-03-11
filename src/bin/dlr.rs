//! # dlr
//!
//! CLI tool for querying delivery status (DLR) of sent SMS messages.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin dlr -- --key YOUR_KEY --id 637849052
//!
//! # Query multiple messages at once
//! cargo run --bin dlr -- --key YOUR_KEY --id 637849052 --id 637849053
//! ```
//!
//! ## Possible statuses
//!
//! | Status      | Meaning                                      |
//! |-------------|----------------------------------------------|
//! | RECEBIDA    | Message delivered to the recipient's device  |
//! | ENVIADA     | Message sent to the operator                 |
//! | FILA        | Message queued / awaiting processing         |
//! | ERROR       | Message validation error                     |
//! | CANCELADA   | Message cancelled by user                    |
//! | BLACK LIST  | Recipient is in the block list               |
//! | APROVACAO   | Under operator approval                      |

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
                eprintln!("Usage: dlr --key KEY --id ID [--id ID ...]");
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

fn status_icon(description: &str) -> &'static str {
    match description.to_uppercase().as_str() {
        s if s.contains("RECEBIDA") => "✓ DELIVERED",
        s if s.contains("ENVIADA") => "→ SENT",
        s if s.contains("FILA") => "⏳ QUEUED",
        s if s.contains("CANCELADA") => "✗ CANCELLED",
        s if s.contains("BLACK") => "⛔ BLACKLIST",
        s if s.contains("APROVACAO") => "⏳ APPROVAL",
        s if s.contains("ERROR") => "✗ ERROR",
        _ => "? UNKNOWN",
    }
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

    println!("Querying DLR for {} message(s)...\n", ids.len());

    match client.dlr(ids).await {
        Ok(statuses) => {
            println!(
                "{:<14} {:<10} {:<22} {:<12} {}",
                "Status", "Code", "Sent At", "Operator", "Raw"
            );
            println!("{}", "-".repeat(78));
            for s in &statuses {
                println!(
                    "{:<14} {:<10} {:<22} {:<12} {}",
                    status_icon(&s.description),
                    s.code,
                    s.data_envio.as_deref().unwrap_or("-"),
                    s.operator.as_deref().unwrap_or("-"),
                    s.description,
                );
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            process::exit(1);
        }
    }
}
