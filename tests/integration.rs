use serde_json::json;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use claim::assert_ok;
use apiclient_boilerplate_rust::client::ApiClient;

#[tokio::test]
async fn integration_test_apiclient_boilerplate_rust() {
    // Arrange       
    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("GET"))
        .and(path("/posts/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "userId": 1,
            "id": 1,
            "title": "Test Post",
            "body": "This is a test post."
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let base_url = mock_server.uri();
    let api_client = ApiClient::new(&base_url);

    // Act
    let outcome = api_client.get_post(1).await;

    // Assert
    assert_ok!(outcome);
}
