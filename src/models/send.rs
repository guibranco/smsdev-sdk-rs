use serde::{Deserialize, Serialize};

// ──────────────────────────────────────────────
// Send SMS (MT)
// ──────────────────────────────────────────────

/// A single SMS message to be sent.
///
/// Build with [`SendSmsRequest::new`] and optionally chain the scheduling
/// helpers (`.schedule_date()`, `.schedule_time()`, `.refer()`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSmsRequest {
    /// Account authentication key.
    pub key: String,

    /// Service type. Always `9` for SMS.
    #[serde(rename = "type")]
    pub service_type: u8,

    /// Destination phone number, e.g. `5511988887777` or `11988887777`.
    pub number: u64,

    /// Message text (up to 160 chars; longer messages consume extra credits).
    pub msg: String,

    /// Optional user reference for message identification (max 100 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer: Option<String>,

    /// Optional scheduling date, format `DD/MM/YYYY`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobdate: Option<String>,

    /// Optional scheduling time, format `HH:MM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobtime: Option<String>,
}

impl SendSmsRequest {
    /// Create a new SMS send request.
    pub fn new(key: impl Into<String>, number: u64, msg: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            service_type: 9,
            number,
            msg: msg.into(),
            refer: None,
            jobdate: None,
            jobtime: None,
        }
    }

    /// Set a user-defined reference identifier (max 100 chars).
    pub fn refer(mut self, refer: impl Into<String>) -> Self {
        self.refer = Some(refer.into());
        self
    }

    /// Schedule the message for a specific date (format: `DD/MM/YYYY`).
    pub fn schedule_date(mut self, date: impl Into<String>) -> Self {
        self.jobdate = Some(date.into());
        self
    }

    /// Schedule the message for a specific time (format: `HH:MM`).
    pub fn schedule_time(mut self, time: impl Into<String>) -> Self {
        self.jobtime = Some(time.into());
        self
    }
}

/// The result returned for each message in a send operation.
#[derive(Debug, Clone, Deserialize)]
pub struct SendSmsResponse {
    /// `"OK"` on success, `"ERROR"` otherwise.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Numeric status code. Consult the SMSDev error code table for details.
    #[serde(rename = "codigo")]
    pub code: String,

    /// Unique ID of the queued / sent message. Keep this for DLR queries.
    pub id: String,

    /// Human-readable description, e.g. `"MENSAGEM NA FILA"`.
    #[serde(rename = "descricao")]
    pub description: String,
}

impl SendSmsResponse {
    /// Returns `true` when the API accepted the message without error.
    pub fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }
}
