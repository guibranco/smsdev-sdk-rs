//! # smsdev
//!
//! An async Rust SDK for the [SMSDev](https://www.smsdev.com.br/en/) SMS Gateway API.
//!
//! ## Features
//!
//! | Method              | Description                                      |
//! |---------------------|--------------------------------------------------|
//! | `send_sms`          | Send one or many SMS messages (bulk supported)   |
//! | `send_one`          | Convenience wrapper to send a single SMS         |
//! | `cancel`            | Cancel scheduled messages by ID                  |
//! | `inbox`             | Query received (MO) messages                     |
//! | `dlr`               | Query delivery status of sent messages           |
//! | `balance`           | Get account SMS credit balance                   |
//! | `report`            | Fetch a usage summary report by date range       |
//!
//! ## Quick Start
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! smsdev = "0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ### Send an SMS
//!
//! ```rust,no_run
//! use smsdev::{SmsDev, models::SendSmsRequest};
//!
//! #[tokio::main]
//! async fn main() -> smsdev::Result<()> {
//!     let client = SmsDev::new("YOUR_API_KEY");
//!
//!     let result = client
//!         .send_one(SendSmsRequest::new(
//!             "YOUR_API_KEY",
//!             5511988887777_u64,
//!             "Hello from the smsdev Rust SDK!",
//!         ))
//!         .await?;
//!
//!     println!("Message queued with id: {}", result.id);
//!     Ok(())
//! }
//! ```
//!
//! ### Check Account Balance
//!
//! ```rust,no_run
//! use smsdev::SmsDev;
//!
//! #[tokio::main]
//! async fn main() -> smsdev::Result<()> {
//!     let client = SmsDev::new("YOUR_API_KEY");
//!     let balance = client.balance().await?;
//!     println!("SMS credits: {}", balance.sms_balance);
//!     Ok(())
//! }
//! ```
//!
//! ### Bulk Send
//!
//! ```rust,no_run
//! use smsdev::{SmsDev, models::SendSmsRequest};
//!
//! #[tokio::main]
//! async fn main() -> smsdev::Result<()> {
//!     let client = SmsDev::new("YOUR_API_KEY");
//!
//!     let messages = vec![
//!         SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Hi Alice!"),
//!         SendSmsRequest::new("YOUR_API_KEY", 5521977776666, "Hi Bob!")
//!             .refer("campaign-123"),
//!     ];
//!
//!     let results = client.send_sms(messages).await?;
//!     for r in results {
//!         println!("id={} ok={}", r.id, r.is_ok());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! All requests are authenticated via an API key (*Chave Key*) which you can
//! find in your [SMSDev account profile](https://painel.smsdev.com.br/configuracao/conta/perfil).
//! Pass it once when constructing [`SmsDev`]; the client attaches it automatically
//! to every request.

pub mod error;
pub mod models;
mod client;

pub use client::SmsDev;
pub use error::{Result, SmsDevError};
