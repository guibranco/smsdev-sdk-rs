use reqwest::Client;
use serde_json::json;

use crate::error::{Result, SmsDevError};
use crate::models::{
    BalanceResponse, CancelRequest, CancelResponse, DlrRequest, DlrResponse,
    InboxMessage, InboxRequest, ReportRequest, ReportResponse, SendSmsRequest,
    SendSmsResponse,
};

const BASE_URL: &str = "https://api.smsdev.com.br/v1";

/// The primary entry-point for interacting with the SMSDev API.
///
/// # Example
/// ```rust,no_run
/// use smsdev::{SmsDev, models::SendSmsRequest};
///
/// #[tokio::main]
/// async fn main() -> smsdev::Result<()> {
///     let client = SmsDev::new("YOUR_API_KEY");
///
///     let req = SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Hello from Rust!");
///     let results = client.send_sms(vec![req]).await?;
///
///     for r in &results {
///         println!("Sent! Message ID: {}", r.id);
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SmsDev {
    api_key: String,
    http: Client,
    base_url: String,
}

impl SmsDev {
    /// Create a new `SmsDev` client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            http: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Override the base URL (useful for testing against a mock server).
    #[doc(hidden)]
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    // ──────────────────────────────────────────────────────────────
    // Private helpers
    // ──────────────────────────────────────────────────────────────

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    async fn post_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<T> {
        let url = self.endpoint(path);
        let response = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    // ──────────────────────────────────────────────────────────────
    // Public API methods
    // ──────────────────────────────────────────────────────────────

    /// Send one or more SMS messages.
    ///
    /// Each element in `messages` targets a single recipient. The API
    /// accepts batches, so you can pass as many as needed in one call.
    ///
    /// Returns one [`SendSmsResponse`] per message, in the same order.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::{SmsDev, models::SendSmsRequest};
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    ///
    /// let messages = vec![
    ///     SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Hello Alice!"),
    ///     SendSmsRequest::new("YOUR_API_KEY", 5521977776666, "Hello Bob!"),
    /// ];
    ///
    /// let results = client.send_sms(messages).await?;
    /// for r in results {
    ///     println!("id={} status={}", r.id, r.status);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn send_sms(&self, messages: Vec<SendSmsRequest>) -> Result<Vec<SendSmsResponse>> {
        let url = self.endpoint("send");
        let body = serde_json::to_value(&messages)?;
        let response = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let results: Vec<SendSmsResponse> = response.json().await?;
        Ok(results)
    }

    /// Send a single SMS message (convenience wrapper around [`send_sms`]).
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::{SmsDev, models::SendSmsRequest};
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let req = SendSmsRequest::new("YOUR_API_KEY", 5511988887777, "Hi!")
    ///     .refer("order-42")
    ///     .schedule_date("25/12/2025")
    ///     .schedule_time("09:00");
    /// let result = client.send_one(req).await?;
    /// println!("Queued with id={}", result.id);
    /// # Ok(()) }
    /// ```
    pub async fn send_one(&self, message: SendSmsRequest) -> Result<SendSmsResponse> {
        let mut results = self.send_sms(vec![message]).await?;
        results
            .pop()
            .ok_or_else(|| SmsDevError::UnexpectedResponse("Empty response array".into()))
    }

    /// Cancel one or more scheduled messages.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::SmsDev;
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let results = client.cancel(vec![637849052, 637849053]).await?;
    /// for r in results {
    ///     println!("Cancelled id={}: {}", r.id, r.description);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn cancel(&self, ids: Vec<u64>) -> Result<Vec<CancelResponse>> {
        let req = CancelRequest::new(&self.api_key, ids);
        let body = json!(req);
        self.post_json::<Vec<CancelResponse>>("cancel", &body).await
    }

    /// Query the inbox for received (MO) messages.
    ///
    /// # Example – fetch only new messages
    /// ```rust,no_run
    /// # use smsdev::{SmsDev, models::InboxRequest};
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let inbox = client.inbox(InboxRequest::new("YOUR_API_KEY")).await?;
    /// for msg in inbox {
    ///     println!("[{}] {}: {}", msg.data_read, msg.phone, msg.description);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn inbox(&self, req: InboxRequest) -> Result<Vec<InboxMessage>> {
        let mut body = json!({
            "key": req.key,
            "status": req.status as u8,
        });
        if let Some(df) = req.date_from {
            body["date_from"] = json!(df);
        }
        if let Some(dt) = req.date_to {
            body["date_to"] = json!(dt);
        }
        if let Some(ids) = req.id {
            body["id"] = json!(ids);
        }

        self.post_json::<Vec<InboxMessage>>("inbox", &body).await
    }

    /// Query delivery status (DLR) for one or more sent messages.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::SmsDev;
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let statuses = client.dlr(vec![637849052]).await?;
    /// for s in statuses {
    ///     println!("operator={:?} status={}", s.operator, s.description);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn dlr(&self, ids: Vec<u64>) -> Result<Vec<DlrResponse>> {
        let req = DlrRequest::new(&self.api_key, ids);
        let body = json!(req);
        // The API may return a single object or an array; normalise to Vec.
        let url = self.endpoint("dlr");
        let raw = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        // Try array first, fall back to single object.
        if let Ok(vec) = serde_json::from_str::<Vec<DlrResponse>>(&raw) {
            Ok(vec)
        } else if let Ok(single) = serde_json::from_str::<DlrResponse>(&raw) {
            Ok(vec![single])
        } else {
            Err(SmsDevError::UnexpectedResponse(raw))
        }
    }

    /// Retrieve the account's current SMS credit balance.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::SmsDev;
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let balance = client.balance().await?;
    /// println!("Credits remaining: {}", balance.sms_balance);
    /// # Ok(()) }
    /// ```
    pub async fn balance(&self) -> Result<BalanceResponse> {
        let body = json!({ "key": self.api_key });
        self.post_json::<BalanceResponse>("balance", &body).await
    }

    /// Fetch a total usage report, optionally filtered by date range.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use smsdev::{SmsDev, models::ReportRequest};
    /// # #[tokio::main] async fn main() -> smsdev::Result<()> {
    /// let client = SmsDev::new("YOUR_API_KEY");
    /// let report = client
    ///     .report(
    ///         ReportRequest::new("YOUR_API_KEY")
    ///             .date_from("01/01/2025")
    ///             .date_to("31/01/2025"),
    ///     )
    ///     .await?;
    /// println!(
    ///     "Sent={} Received={} Credits={}",
    ///     report.sent, report.received, report.credits_used
    /// );
    /// # Ok(()) }
    /// ```
    pub async fn report(&self, req: ReportRequest) -> Result<ReportResponse> {
        let mut body = json!({ "key": req.key });
        if let Some(df) = req.date_from {
            body["date_from"] = json!(df);
        }
        if let Some(dt) = req.date_to {
            body["date_to"] = json!(dt);
        }
        self.post_json::<ReportResponse>("report/total", &body).await
    }
}
