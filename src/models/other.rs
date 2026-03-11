use serde::{Deserialize, Serialize};

// ──────────────────────────────────────────────
// Cancel (MT)
// ──────────────────────────────────────────────

/// Request body for cancelling one or more scheduled messages.
#[derive(Debug, Clone, Serialize)]
pub struct CancelRequest {
    /// Account authentication key.
    pub key: String,
    /// List of message IDs to cancel.
    pub id: Vec<u64>,
}

impl CancelRequest {
    /// Create a cancel request for the given message IDs.
    pub fn new(key: impl Into<String>, ids: Vec<u64>) -> Self {
        Self { key: key.into(), id: ids }
    }
}

/// Result for a single cancelled message.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelResponse {
    /// `"OK"` on success, `"ERROR"` otherwise.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Numeric status code.
    #[serde(rename = "codigo")]
    pub code: String,

    /// ID of the affected message.
    pub id: String,

    /// Human-readable description.
    #[serde(rename = "descricao")]
    pub description: String,
}

impl CancelResponse {
    /// Returns `true` when the cancellation succeeded.
    pub fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }
}

// ──────────────────────────────────────────────
// SMS Inbox / Receiving (MO)
// ──────────────────────────────────────────────

/// Controls which messages the inbox query returns.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum InboxStatus {
    /// Return only new (unread) responses. **Default.**
    #[default]
    NewOnly = 0,
    /// Return all responses (read and unread).
    All = 1,
}

impl From<InboxStatus> for u8 {
    fn from(s: InboxStatus) -> u8 {
        s as u8
    }
}

/// Parameters for querying received messages.
#[derive(Debug, Clone, Default)]
pub struct InboxRequest {
    /// Account authentication key.
    pub key: String,
    /// Filter by read status.
    pub status: InboxStatus,
    /// Optional start date filter (format: `DD/MM/YYYY`).
    pub date_from: Option<String>,
    /// Optional end date filter (format: `DD/MM/YYYY`).
    pub date_to: Option<String>,
    /// Optional list of sent-message IDs to filter responses by.
    pub id: Option<Vec<u64>>,
}

impl InboxRequest {
    /// Create a basic inbox request that fetches only new messages.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into(), ..Default::default() }
    }

    /// Include all messages, not just unread ones.
    pub fn all(mut self) -> Self {
        self.status = InboxStatus::All;
        self
    }

    /// Filter messages starting from this date (format: `DD/MM/YYYY`).
    pub fn date_from(mut self, date: impl Into<String>) -> Self {
        self.date_from = Some(date.into());
        self
    }

    /// Filter messages up to this date (format: `DD/MM/YYYY`).
    pub fn date_to(mut self, date: impl Into<String>) -> Self {
        self.date_to = Some(date.into());
        self
    }

    /// Limit results to responses for the specified sent-message IDs.
    pub fn filter_ids(mut self, ids: Vec<u64>) -> Self {
        self.id = Some(ids);
        self
    }
}

/// A received (MO) message.
#[derive(Debug, Clone, Deserialize)]
pub struct InboxMessage {
    /// `"OK"` on success, `"ERROR"` on error.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Date and time the response was received (`DD/MM/YYYY HH:MM:SS`).
    pub data_read: String,

    /// Phone number that sent the reply (e.g. `5511988887777`).
    #[serde(rename = "telefone")]
    pub phone: String,

    /// ID of the original sent message (MT) this reply belongs to, if known.
    pub id: String,

    /// Reference identifier from the original send request.
    pub refer: String,

    /// Text of the original sent message.
    pub msg_sent: String,

    /// Unique ID of this received message.
    pub id_sms_read: String,

    /// Text of the received reply.
    #[serde(rename = "descricao")]
    pub description: String,
}

// ──────────────────────────────────────────────
// DLR – Message Status Inquiry
// ──────────────────────────────────────────────

/// Request for querying delivery status of one or more sent messages.
#[derive(Debug, Clone, Serialize)]
pub struct DlrRequest {
    /// Account authentication key.
    pub key: String,
    /// IDs of the sent messages to query.
    pub id: Vec<u64>,
}

impl DlrRequest {
    /// Build a DLR query for the given message IDs.
    pub fn new(key: impl Into<String>, ids: Vec<u64>) -> Self {
        Self { key: key.into(), id: ids }
    }
}

/// Delivery status of a single message.
#[derive(Debug, Clone, Deserialize)]
pub struct DlrResponse {
    /// `"OK"` on success, `"ERROR"` otherwise.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Numeric status code.
    #[serde(rename = "codigo")]
    pub code: String,

    /// Timestamp the message was sent (`DD/MM/YYYY HH:MM:SS`).
    pub data_envio: Option<String>,

    /// Mobile operator the message was sent to (e.g. `"OI"`, `"VIVO"`).
    #[serde(rename = "operadora")]
    pub operator: Option<String>,

    /// Delivery status description.
    ///
    /// Possible values:
    /// - `"RECEBIDA"` – delivered to the device
    /// - `"ENVIADA"` – sent to the operator
    /// - `"ERROR"` – validation error
    /// - `"FILA"` – queued / awaiting processing
    /// - `"CANCELADA"` – cancelled by the user
    /// - `"BLACK LIST"` – recipient in the block list
    /// - `"APROVACAO"` – under operator approval
    #[serde(rename = "descricao")]
    pub description: String,
}

impl DlrResponse {
    /// Returns `true` when the API call itself succeeded (not necessarily
    /// that the SMS was delivered — check [`DlrResponse::description`] for that).
    pub fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }

    /// Convenience: returns `true` if the message was delivered to the device.
    pub fn is_delivered(&self) -> bool {
        self.description.eq_ignore_ascii_case("recebida")
    }
}

// ──────────────────────────────────────────────
// Account Balance
// ──────────────────────────────────────────────

/// The current SMS credit balance for the account.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    /// `"OK"` on success, `"ERROR"` otherwise.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Available SMS credits.
    #[serde(rename = "saldo_sms")]
    pub sms_balance: String,

    /// Human-readable description.
    #[serde(rename = "descricao")]
    pub description: String,
}

impl BalanceResponse {
    /// Returns `true` when the API call succeeded.
    pub fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }

    /// Parse the SMS balance as an integer.
    pub fn balance_as_u64(&self) -> Option<u64> {
        self.sms_balance.trim().parse().ok()
    }
}

// ──────────────────────────────────────────────
// Total Report
// ──────────────────────────────────────────────

/// Parameters for a total-usage report query.
#[derive(Debug, Clone, Default)]
pub struct ReportRequest {
    /// Account authentication key.
    pub key: String,
    /// Optional start date filter (format: `DD/MM/YYYY`).
    pub date_from: Option<String>,
    /// Optional end date filter (format: `DD/MM/YYYY`).
    pub date_to: Option<String>,
}

impl ReportRequest {
    /// Build a report request with optional date filters.
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into(), ..Default::default() }
    }

    /// Filter report starting from this date (format: `DD/MM/YYYY`).
    pub fn date_from(mut self, date: impl Into<String>) -> Self {
        self.date_from = Some(date.into());
        self
    }

    /// Filter report up to this date (format: `DD/MM/YYYY`).
    pub fn date_to(mut self, date: impl Into<String>) -> Self {
        self.date_to = Some(date.into());
        self
    }
}

/// Summary usage statistics for a reporting period.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportResponse {
    /// `"OK"` on success, `"ERROR"` otherwise.
    #[serde(rename = "situacao")]
    pub status: String,

    /// Numeric status code.
    #[serde(rename = "codigo")]
    pub code: String,

    /// Start date of the period (`DD/MM/YYYY`).
    pub data_inicio: Option<String>,

    /// End date of the period (`DD/MM/YYYY`).
    pub data_fim: Option<String>,

    /// Count of messages with status *sent* during the period.
    #[serde(rename = "enviada")]
    pub sent: String,

    /// Count of messages with status *received* during the period.
    #[serde(rename = "recebida")]
    pub received: String,

    /// Count of messages rejected by the block list.
    #[serde(rename = "blacklist")]
    pub blacklist: String,

    /// Count of messages cancelled during the period.
    #[serde(rename = "cancelada")]
    pub cancelled: String,

    /// Total credits consumed during the period.
    #[serde(rename = "qtd_credito")]
    pub credits_used: String,

    /// Human-readable description.
    #[serde(rename = "descricao")]
    pub description: String,
}

impl ReportResponse {
    /// Returns `true` when the API call succeeded.
    pub fn is_ok(&self) -> bool {
        self.status.eq_ignore_ascii_case("ok")
    }
}
