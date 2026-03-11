# smsdev-rs

An async Rust SDK for the [SMSDev](https://www.smsdev.com.br/en/) SMS Gateway API.

[![Crates.io](https://img.shields.io/crates/v/smsdev.svg)](https://crates.io/crates/smsdev)
[![Docs.rs](https://docs.rs/smsdev/badge.svg)](https://docs.rs/smsdev)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## Features

| Method       | Description                                        |
|--------------|----------------------------------------------------|
| `send_sms`   | Send one or many SMS messages (batch supported)    |
| `send_one`   | Convenience wrapper for sending a single SMS       |
| `cancel`     | Cancel scheduled messages by ID                    |
| `inbox`      | Query received (MO) messages                       |
| `dlr`        | Query delivery status (DLR) of sent messages       |
| `balance`    | Get the account SMS credit balance                 |
| `report`     | Fetch a usage summary report by date range         |

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smsdev = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## Authentication

All requests are authenticated with an **API Key** (*Chave Key*).  
You can obtain yours at: <https://painel.smsdev.com.br/configuracao/conta/perfil>

---

## Usage

### Send a Single SMS

```rust
use smsdev::{SmsDev, models::SendSmsRequest};

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");

    let result = client
        .send_one(SendSmsRequest::new(
            "YOUR_API_KEY",
            5511988887777_u64,
            "Hello from Rust!",
        ))
        .await?;

    println!("Queued with id: {}", result.id);
    Ok(())
}
```

### Bulk Send

```rust
use smsdev::{SmsDev, models::SendSmsRequest};

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");

    let messages = vec![
        SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Hi Alice!"),
        SendSmsRequest::new("YOUR_API_KEY", 5521977776666, "Hi Bob!")
            .refer("campaign-abc"),
    ];

    let results = client.send_sms(messages).await?;
    for r in results {
        println!("id={} status={}", r.id, r.status);
    }
    Ok(())
}
```

### Schedule a Message

```rust
use smsdev::{SmsDev, models::SendSmsRequest};

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");

    let result = client
        .send_one(
            SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Reminder!")
                .schedule_date("25/12/2025")
                .schedule_time("09:00"),
        )
        .await?;

    println!("Scheduled message id: {}", result.id);
    Ok(())
}
```

### Cancel a Scheduled Message

```rust
use smsdev::SmsDev;

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");
    let results = client.cancel(vec![637849052, 637849053]).await?;
    for r in results {
        println!("Cancelled id={}: {}", r.id, r.description);
    }
    Ok(())
}
```

### Check Delivery Status (DLR)

```rust
use smsdev::SmsDev;

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");
    let statuses = client.dlr(vec![637849052]).await?;
    for s in statuses {
        if s.is_delivered() {
            println!("Message delivered via {}", s.operator.unwrap_or_default());
        } else {
            println!("Status: {}", s.description);
        }
    }
    Ok(())
}
```

### Read Inbox (MO)

```rust
use smsdev::{SmsDev, models::InboxRequest};

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");

    let messages = client
        .inbox(
            InboxRequest::new("YOUR_API_KEY")
                .all()                          // include already-read messages
                .date_from("01/01/2025")
                .date_to("31/01/2025"),
        )
        .await?;

    for msg in messages {
        println!("[{}] from {}: {}", msg.data_read, msg.phone, msg.description);
    }
    Ok(())
}
```

### Account Balance

```rust
use smsdev::SmsDev;

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");
    let bal = client.balance().await?;
    println!("Credits remaining: {}", bal.balance_as_u64().unwrap_or(0));
    Ok(())
}
```

### Usage Report

```rust
use smsdev::{SmsDev, models::ReportRequest};

#[tokio::main]
async fn main() -> smsdev::Result<()> {
    let client = SmsDev::new("YOUR_API_KEY");
    let report = client
        .report(
            ReportRequest::new("YOUR_API_KEY")
                .date_from("01/01/2025")
                .date_to("31/01/2025"),
        )
        .await?;

    println!(
        "Sent={} Received={} Credits used={}",
        report.sent, report.received, report.credits_used
    );
    Ok(())
}
```

---

## Error Handling

All methods return `smsdev::Result<T>` which wraps [`SmsDevError`]:

```rust
use smsdev::{SmsDev, SmsDevError};

#[tokio::main]
async fn main() {
    let client = SmsDev::new("BAD_KEY");
    match client.balance().await {
        Ok(b) => println!("Balance: {}", b.sms_balance),
        Err(SmsDevError::Api { code, description }) => {
            eprintln!("API rejected the request [{code}]: {description}")
        }
        Err(SmsDevError::Http(e)) => eprintln!("Network error: {e}"),
        Err(e) => eprintln!("Other error: {e}"),
    }
}
```

---

## Running Tests

```bash
cargo test
```

Tests use [mockito](https://crates.io/crates/mockito) to intercept HTTP calls,
so no live API key is required.

---

## License

MIT — see [LICENSE](LICENSE).
