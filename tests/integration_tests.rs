/// Integration / unit tests using `mockito` to intercept HTTP calls.
///
/// Run with: `cargo test`
#[cfg(test)]
mod tests {
    use mockito::{Matcher, Server};
    use smsdev::{
        SmsDev,
        models::{InboxRequest, ReportRequest, SendSmsRequest},
    };

    fn client(base_url: &str) -> SmsDev {
        SmsDev::new("TEST_KEY").with_base_url(base_url)
    }

    // ── send_sms ──────────────────────────────────────────────────
    #[tokio::test]
    async fn test_send_sms_ok() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"situacao":"OK","codigo":"1","id":"637849052","descricao":"MENSAGEM NA FILA"}]"#,
            )
            .create_async()
            .await;

        let c = client(&server.url());
        let req = SendSmsRequest::new("TEST_KEY", 5511988887777_u64, "Hello");
        let results = c.send_sms(vec![req]).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "637849052");
        assert!(results[0].is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_one_ok() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"situacao":"OK","codigo":"1","id":"111","descricao":"MENSAGEM NA FILA"}]"#,
            )
            .create_async()
            .await;

        let c = client(&server.url());
        let result = c
            .send_one(SendSmsRequest::new("TEST_KEY", 5511988887777_u64, "Hi!"))
            .await
            .unwrap();
        assert_eq!(result.id, "111");
    }

    // ── cancel ───────────────────────────────────────────────────
    #[tokio::test]
    async fn test_cancel_ok() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/cancel")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"situacao":"OK","codigo":"1","id":"9999999","descricao":"MENSAGEM CANCELADA COM SUCESSO"}]"#,
            )
            .create_async()
            .await;

        let c = client(&server.url());
        let results = c.cancel(vec![9999999]).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert_eq!(results[0].id, "9999999");
    }

    // ── inbox ────────────────────────────────────────────────────
    #[tokio::test]
    async fn test_inbox_ok() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/inbox")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[
                {"situacao":"OK","data_read":"01/01/2020 11:35:14","telefone":"5511988887777","id":"","refer":"","msg_sent":"","id_sms_read":"2515973","descricao":"Reply 1"}
            ]"#)
            .create_async()
            .await;

        let c = client(&server.url());
        let msgs = c.inbox(InboxRequest::new("TEST_KEY")).await.unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].description, "Reply 1");
        assert_eq!(msgs[0].phone, "5511988887777");
    }

    // ── dlr ──────────────────────────────────────────────────────
    #[tokio::test]
    async fn test_dlr_single_object() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/dlr")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"situacao":"OK","codigo":"1","data_envio":"21/10/2019 11:08:58","operadora":"OI","descricao":"RECEBIDA"}"#,
            )
            .create_async()
            .await;

        let c = client(&server.url());
        let statuses = c.dlr(vec![123456789]).await.unwrap();
        assert_eq!(statuses.len(), 1);
        assert!(statuses[0].is_delivered());
    }

    #[tokio::test]
    async fn test_dlr_array() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/dlr")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"situacao":"OK","codigo":"1","data_envio":"21/10/2019 11:08:58","operadora":"VIVO","descricao":"ENVIADA"}]"#,
            )
            .create_async()
            .await;

        let c = client(&server.url());
        let statuses = c.dlr(vec![123456789]).await.unwrap();
        assert_eq!(statuses.len(), 1);
        assert!(!statuses[0].is_delivered()); // ENVIADA ≠ RECEBIDA
    }

    // ── balance ───────────────────────────────────────────────────
    #[tokio::test]
    async fn test_balance_ok() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/balance")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"situacao":"OK","saldo_sms":"1200","descricao":"SALDO ATUAL"}"#)
            .create_async()
            .await;

        let c = client(&server.url());
        let bal = c.balance().await.unwrap();
        assert!(bal.is_ok());
        assert_eq!(bal.balance_as_u64(), Some(1200));
    }

    // ── report ───────────────────────────────────────────────────
    #[tokio::test]
    async fn test_report_ok() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/report/total")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "situacao":"OK","codigo":"1",
                "data_inicio":"01/01/2020","data_fim":"30/01/2020",
                "enviada":"100","recebida":"10200","blacklist":"0",
                "cancelada":"0","qtd_credito":"10300",
                "descricao":"CONSULTA REALIZADA"
            }"#)
            .create_async()
            .await;

        let c = client(&server.url());
        let report = c
            .report(
                ReportRequest::new("TEST_KEY")
                    .date_from("01/01/2020")
                    .date_to("30/01/2020"),
            )
            .await
            .unwrap();

        assert!(report.is_ok());
        assert_eq!(report.sent, "100");
        assert_eq!(report.credits_used, "10300");
    }

    // ── SendSmsRequest builder ────────────────────────────────────
    #[test]
    fn test_send_request_builder() {
        let req = SendSmsRequest::new("KEY", 5511999998888_u64, "Test")
            .refer("ref-01")
            .schedule_date("25/12/2025")
            .schedule_time("09:00");

        assert_eq!(req.refer.as_deref(), Some("ref-01"));
        assert_eq!(req.jobdate.as_deref(), Some("25/12/2025"));
        assert_eq!(req.jobtime.as_deref(), Some("09:00"));
        assert_eq!(req.service_type, 9);
    }
}
