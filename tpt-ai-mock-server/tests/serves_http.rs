//! End-to-end test that `MockServer` actually serves HTTP responses over a
//! real TCP connection, using `reqwest` (the same HTTP stack
//! `tpt-llm-client-core` uses) as the client.

use tpt_ai_mock_server::{MockServer, RecordedResponse};

#[tokio::test]
async fn serves_json_response_to_a_real_http_client() {
    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(
        r#"{"id":"resp_1","choices":[{"index":0,"message":{"role":"assistant","content":"hi"},"finish_reason":"stop"}],"usage":{"prompt_tokens":1,"completion_tokens":1,"total_tokens":2}}"#,
    ));
    let addr = server.start().await.unwrap();

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/v1/chat/completions"))
        .json(&serde_json::json!({"model": "test", "messages": []}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["choices"][0]["message"]["content"], "hi");
}

#[tokio::test]
async fn serves_queued_responses_in_order() {
    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(r#"{"turn":1}"#));
    server.add_response(RecordedResponse::json_response(r#"{"turn":2}"#));
    let addr = server.start().await.unwrap();

    let client = reqwest::Client::new();
    let url = format!("http://{addr}/v1/chat/completions");
    let body = serde_json::json!({"model": "test", "messages": []});

    let first: serde_json::Value = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let second: serde_json::Value = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(first["turn"], 1);
    assert_eq!(second["turn"], 2);
}

#[tokio::test]
async fn rejects_request_missing_required_fields() {
    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(r#"{"ok":true}"#));
    let addr = server.start().await.unwrap();

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/v1/chat/completions"))
        .json(&serde_json::json!({"messages": []}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn returns_404_for_unmatched_path() {
    let mut server = MockServer::new();
    server.add_response(RecordedResponse::json_response(r#"{"ok":true}"#));
    let addr = server.start().await.unwrap();

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/v1/unknown"))
        .json(&serde_json::json!({"model": "test", "messages": []}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}
