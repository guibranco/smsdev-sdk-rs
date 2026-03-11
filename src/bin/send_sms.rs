//! # send_sms
//!
//! CLI tool for sending one or more SMS messages via the SMSDev API.
//!
//! ## Usage
//!
//! ```bash
//! # Single message
//! cargo run --bin send_sms -- --key YOUR_KEY --number 5511988887777 --msg "Hello!"
//!
//! # With optional scheduling and reference
//! cargo run --bin send_sms -- \
//!   --key YOUR_KEY \
//!   --number 5511988887777 \
//!   --msg "Reminder!" \
//!   --date 25/12/2025 \
//!   --time 09:00 \
//!   --refer order-42
//!
//! # Multiple recipients (repeat --number / --msg pairs)
//! cargo run --bin send_sms -- \
//!   --key YOUR_KEY \
//!   --number 5511988887777 --msg "Hi Alice!" \
//!   --number 5521977776666 --msg "Hi Bob!"
//! ```

use std::process;
use smsdev::{SmsDev, models::SendSmsRequest};

#[derive(Debug)]
struct Args {
    key: String,
    numbers: Vec<u64>,
    messages: Vec<String>,
    refer: Option<String>,
    date: Option<String>,
    time: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut key = None;
    let mut numbers: Vec<u64> = Vec::new();
    let mut messages: Vec<String> = Vec::new();
    let mut refer = None;
    let mut date = None;
    let mut time = None;

    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--key" => {
                i += 1;
                key = Some(raw.get(i).ok_or("--key requires a value")?.clone());
            }
            "--number" => {
                i += 1;
                let n: u64 = raw
                    .get(i)
                    .ok_or("--number requires a value")?
                    .parse()
                    .map_err(|_| "--number must be a numeric phone number")?;
                numbers.push(n);
            }
            "--msg" => {
                i += 1;
                messages.push(raw.get(i).ok_or("--msg requires a value")?.clone());
            }
            "--refer" => {
                i += 1;
                refer = Some(raw.get(i).ok_or("--refer requires a value")?.clone());
            }
            "--date" => {
                i += 1;
                date = Some(raw.get(i).ok_or("--date requires a value")?.clone());
            }
            "--time" => {
                i += 1;
                time = Some(raw.get(i).ok_or("--time requires a value")?.clone());
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: send_sms --key KEY --number NUMBER --msg MSG \
                     [--refer REF] [--date DD/MM/YYYY] [--time HH:MM]"
                );
                process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
        i += 1;
    }

    let key = key.ok_or("--key is required")?;
    if numbers.is_empty() {
        return Err("At least one --number is required".into());
    }
    if messages.is_empty() {
        return Err("At least one --msg is required".into());
    }
    if numbers.len() != messages.len() {
        return Err(format!(
            "Mismatch: {} --number(s) but {} --msg(s)",
            numbers.len(),
            messages.len()
        ));
    }

    Ok(Args { key, numbers, messages, refer, date, time })
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

    let requests: Vec<SendSmsRequest> = args
        .numbers
        .iter()
        .zip(args.messages.iter())
        .map(|(number, msg)| {
            let mut req = SendSmsRequest::new(&args.key, *number, msg);
            if let Some(ref r) = args.refer {
                req = req.refer(r);
            }
            if let Some(ref d) = args.date {
                req = req.schedule_date(d);
            }
            if let Some(ref t) = args.time {
                req = req.schedule_time(t);
            }
            req
        })
        .collect();

    println!("Sending {} message(s)...", requests.len());

    match client.send_sms(requests).await {
        Ok(results) => {
            let mut all_ok = true;
            for (i, r) in results.iter().enumerate() {
                if r.is_ok() {
                    println!(
                        "[{}] ✓  id={} — {}",
                        i + 1,
                        r.id,
                        r.description
                    );
                } else {
                    eprintln!(
                        "[{}] ✗  code={} — {}",
                        i + 1,
                        r.code,
                        r.description
                    );
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
