//! Real HTTP transport for `SseClient` (requires the `std` feature).

use crate::response::{Choice, Delta, StreamChoice, Usage};
use crate::{
    ChatRequest, ChatResponse, Error, Message, Provider, RetryConfig, Role, SseParser, StreamChunk,
};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use futures::Stream;
use serde::Deserialize;
use serde_json::Value;

/// Whether an error is worth retrying: connection-level failures, 429s, and
/// 5xx provider errors. 4xx errors other than 429 (bad request, unauthorized,
/// etc.) are not retried since a retry would fail identically.
fn is_retryable(err: &Error) -> bool {
    match err {
        Error::Network(e) => e.is_timeout() || e.is_connect() || e.is_request(),
        Error::RateLimited => true,
        Error::Provider { code, .. } => *code >= 500,
        _ => false,
    }
}

/// Run `attempt` up to `config.max_attempts` times, retrying on
/// [`is_retryable`] errors with exponential backoff between attempts.
async fn with_retry<T, F, Fut>(config: &RetryConfig, mut attempt: F) -> Result<T, Error>
where
    F: FnMut() -> Fut,
    Fut: core::future::Future<Output = Result<T, Error>>,
{
    let mut delay_ms = config.base_delay_ms;
    for attempt_num in 1..=config.max_attempts {
        match attempt().await {
            Ok(value) => return Ok(value),
            Err(err) if attempt_num < config.max_attempts && is_retryable(&err) => {
                tokio::time::sleep(core::time::Duration::from_millis(delay_ms)).await;
                delay_ms = delay_ms.saturating_mul(2);
            }
            Err(err) => return Err(err),
        }
    }
    unreachable!("loop always returns on the final attempt")
}

/// Build the request URL for a provider.
pub(crate) fn endpoint(base_url: &str, provider: Provider) -> String {
    let base = base_url.trim_end_matches('/');
    match provider {
        Provider::OpenAi => format!("{base}/chat/completions"),
        Provider::Anthropic => format!("{base}/messages"),
        Provider::Ollama => format!("{base}/api/chat"),
    }
}

/// Build the provider-specific JSON request body.
pub(crate) fn build_body(provider: Provider, request: &ChatRequest, stream: bool) -> Value {
    match provider {
        Provider::OpenAi => {
            let mut body = serde_json::to_value(request).unwrap_or_else(|_| serde_json::json!({}));
            body["stream"] = Value::Bool(stream);
            body
        }
        Provider::Ollama => {
            let messages: Vec<Value> = request
                .messages
                .iter()
                .map(|m| serde_json::json!({"role": role_str(m.role), "content": m.content}))
                .collect();
            serde_json::json!({
                "model": request.model,
                "messages": messages,
                "stream": stream,
            })
        }
        Provider::Anthropic => {
            let system: Option<String> = request
                .messages
                .iter()
                .find(|m| m.role == Role::System)
                .map(|m| m.content.clone());
            let messages: Vec<Value> = request
                .messages
                .iter()
                .filter(|m| m.role != Role::System)
                .map(|m| serde_json::json!({"role": role_str(m.role), "content": m.content}))
                .collect();
            let mut body = serde_json::json!({
                "model": request.model,
                "max_tokens": request.max_tokens.unwrap_or(1024),
                "messages": messages,
                "stream": stream,
            });
            if let Some(system) = system {
                body["system"] = Value::String(system);
            }
            if let Some(temperature) = request.temperature {
                body["temperature"] = serde_json::json!(temperature);
            }
            body
        }
    }
}

fn role_str(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
    }
}

fn auth_headers(
    provider: Provider,
    api_key: &str,
    req: reqwest::RequestBuilder,
) -> reqwest::RequestBuilder {
    match provider {
        Provider::OpenAi => req.bearer_auth(api_key),
        Provider::Anthropic => req
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01"),
        Provider::Ollama => req,
    }
}

async fn map_error_response(status: reqwest::StatusCode, body: String) -> Error {
    match status.as_u16() {
        401 => Error::Unauthorized,
        429 => Error::RateLimited,
        code => Error::Provider {
            code,
            message: body,
        },
    }
}

/// Send a non-streaming chat completion request and parse the provider's response
/// into the unified [`ChatResponse`] shape. Retries per `retry` on transient
/// failures (connection errors, 429, 5xx).
pub(crate) async fn send(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    provider: Provider,
    request: &ChatRequest,
    retry: &RetryConfig,
) -> Result<ChatResponse, Error> {
    let url = endpoint(base_url, provider);
    let body = build_body(provider, request, false);

    with_retry(retry, || async {
        let req = client.post(&url).json(&body);
        let req = auth_headers(provider, api_key, req);
        let resp = req.send().await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(map_error_response(status, text).await);
        }

        let text = resp.text().await?;
        parse_response(provider, &text)
    })
    .await
}

fn parse_response(provider: Provider, text: &str) -> Result<ChatResponse, Error> {
    match provider {
        Provider::OpenAi => Ok(serde_json::from_str::<ChatResponse>(text)?),
        Provider::Ollama => {
            #[derive(Deserialize)]
            struct OllamaMessage {
                #[allow(dead_code)]
                role: String,
                content: String,
            }
            #[derive(Deserialize)]
            struct OllamaResponse {
                message: OllamaMessage,
                #[serde(default)]
                prompt_eval_count: u32,
                #[serde(default)]
                eval_count: u32,
                #[serde(default)]
                done_reason: Option<String>,
            }
            let parsed: OllamaResponse = serde_json::from_str(text)?;
            Ok(ChatResponse {
                id: String::from("ollama"),
                choices: alloc::vec![Choice {
                    index: 0,
                    message: Message {
                        role: Role::Assistant,
                        content: parsed.message.content,
                    },
                    finish_reason: parsed.done_reason,
                }],
                usage: Some(Usage {
                    prompt_tokens: parsed.prompt_eval_count,
                    completion_tokens: parsed.eval_count,
                    total_tokens: parsed.prompt_eval_count + parsed.eval_count,
                }),
            })
        }
        Provider::Anthropic => {
            #[derive(Deserialize)]
            struct ContentBlock {
                #[serde(default)]
                text: String,
            }
            #[derive(Deserialize)]
            struct AnthropicUsage {
                #[serde(default)]
                input_tokens: u32,
                #[serde(default)]
                output_tokens: u32,
            }
            #[derive(Deserialize)]
            struct AnthropicResponse {
                id: String,
                content: Vec<ContentBlock>,
                #[serde(default)]
                stop_reason: Option<String>,
                usage: AnthropicUsage,
            }
            let parsed: AnthropicResponse = serde_json::from_str(text)?;
            let content = parsed
                .content
                .into_iter()
                .map(|b| b.text)
                .collect::<Vec<_>>()
                .join("");
            Ok(ChatResponse {
                id: parsed.id,
                choices: alloc::vec![Choice {
                    index: 0,
                    message: Message {
                        role: Role::Assistant,
                        content,
                    },
                    finish_reason: parsed.stop_reason,
                }],
                usage: Some(Usage {
                    prompt_tokens: parsed.usage.input_tokens,
                    completion_tokens: parsed.usage.output_tokens,
                    total_tokens: parsed.usage.input_tokens + parsed.usage.output_tokens,
                }),
            })
        }
    }
}

/// Send a streaming chat completion request, returning a stream of unified
/// [`StreamChunk`] items produced by feeding the SSE (or NDJSON, for Ollama)
/// byte stream through the parser and mapping each event to the provider's shape.
pub(crate) async fn stream(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    provider: Provider,
    request: &ChatRequest,
    retry: &RetryConfig,
) -> Result<impl Stream<Item = Result<StreamChunk, Error>>, Error> {
    let url = endpoint(base_url, provider);
    let body = build_body(provider, request, true);

    // Retries cover establishing the stream (the initial request/response
    // handshake) only — once bytes start arriving, a mid-stream failure is
    // surfaced as an item in the stream rather than retried transparently,
    // since partial output may already have been yielded to the caller.
    let resp = with_retry(retry, || async {
        let req = client.post(&url).json(&body);
        let req = auth_headers(provider, api_key, req);
        let resp = req.send().await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(map_error_response(status, text).await);
        }
        Ok(resp)
    })
    .await?;

    let byte_stream: core::pin::Pin<Box<dyn Stream<Item = reqwest::Result<bytes::Bytes>> + Send>> =
        Box::pin(resp.bytes_stream());
    let state = (
        byte_stream,
        SseParser::new(),
        String::new(),
        provider,
        false,
    );

    Ok(Box::pin(futures::stream::unfold(
        state,
        move |(mut byte_stream, mut parser, mut line_buf, provider, mut done)| async move {
            use futures::StreamExt;

            loop {
                if done {
                    return None;
                }

                if provider == Provider::Ollama {
                    // Ollama streams newline-delimited JSON, not SSE.
                    while let Some(pos) = line_buf.find('\n') {
                        let line: String = line_buf.drain(..=pos).collect();
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }
                        match parse_ollama_stream_line(line) {
                            Ok(Some(chunk)) => {
                                if chunk.choices.iter().any(|c| c.finish_reason.is_some()) {
                                    done = true;
                                }
                                return Some((
                                    Ok(chunk),
                                    (byte_stream, parser, line_buf, provider, done),
                                ));
                            }
                            Ok(None) => continue,
                            Err(e) => {
                                return Some((
                                    Err(e),
                                    (byte_stream, parser, line_buf, provider, true),
                                ))
                            }
                        }
                    }
                    match byte_stream.next().await {
                        Some(Ok(bytes)) => {
                            line_buf.push_str(&String::from_utf8_lossy(&bytes));
                            continue;
                        }
                        Some(Err(e)) => {
                            return Some((
                                Err(Error::from(e)),
                                (byte_stream, parser, line_buf, provider, true),
                            ))
                        }
                        None => return None,
                    }
                }

                // A single network read can carry more than one complete SSE
                // event (common for small/local responses); `feed` only
                // returns the first complete event per call and buffers the
                // rest internally. Drain any already-buffered event before
                // blocking on the network again, so trailing events aren't
                // stranded once the server closes the connection.
                if let Some(event) = parser.feed("") {
                    match parse_stream_event(provider, event.data_str()) {
                        Ok(Some(chunk)) => {
                            return Some((
                                Ok(chunk),
                                (byte_stream, parser, line_buf, provider, done),
                            ))
                        }
                        Ok(None) => continue,
                        Err(e) => {
                            return Some((Err(e), (byte_stream, parser, line_buf, provider, true)))
                        }
                    }
                }

                match byte_stream.next().await {
                    Some(Ok(bytes)) => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        if let Some(event) = parser.feed(&text) {
                            match parse_stream_event(provider, event.data_str()) {
                                Ok(Some(chunk)) => {
                                    return Some((
                                        Ok(chunk),
                                        (byte_stream, parser, line_buf, provider, done),
                                    ))
                                }
                                Ok(None) => continue,
                                Err(e) => {
                                    return Some((
                                        Err(e),
                                        (byte_stream, parser, line_buf, provider, true),
                                    ))
                                }
                            }
                        }
                        continue;
                    }
                    Some(Err(e)) => {
                        return Some((
                            Err(Error::from(e)),
                            (byte_stream, parser, line_buf, provider, true),
                        ))
                    }
                    None => {
                        if let Some(event) = parser.flush() {
                            match parse_stream_event(provider, event.data_str()) {
                                Ok(Some(chunk)) => {
                                    return Some((
                                        Ok(chunk),
                                        (byte_stream, parser, line_buf, provider, true),
                                    ))
                                }
                                _ => return None,
                            }
                        }
                        return None;
                    }
                }
            }
        },
    )))
}

fn parse_ollama_stream_line(line: &str) -> Result<Option<StreamChunk>, Error> {
    #[derive(Deserialize)]
    struct OllamaMessage {
        #[serde(default)]
        content: String,
    }
    #[derive(Deserialize)]
    struct OllamaStreamResponse {
        #[serde(default)]
        message: Option<OllamaMessage>,
        #[serde(default)]
        done: bool,
        #[serde(default)]
        done_reason: Option<String>,
    }
    let parsed: OllamaStreamResponse = serde_json::from_str(line)?;
    let content = parsed.message.map(|m| m.content).unwrap_or_default();
    Ok(Some(StreamChunk {
        id: String::from("ollama"),
        choices: alloc::vec![StreamChoice {
            index: 0,
            delta: Delta {
                role: Some(String::from("assistant")),
                content: if content.is_empty() {
                    None
                } else {
                    Some(content)
                },
            },
            finish_reason: if parsed.done {
                Some(parsed.done_reason.unwrap_or_else(|| String::from("stop")))
            } else {
                None
            },
        }],
    }))
}

fn parse_stream_event(provider: Provider, data: &str) -> Result<Option<StreamChunk>, Error> {
    if data == "[DONE]" {
        return Ok(None);
    }
    match provider {
        Provider::OpenAi => Ok(Some(serde_json::from_str::<StreamChunk>(data)?)),
        Provider::Anthropic => {
            let value: Value = serde_json::from_str(data)?;
            let event_type = value.get("type").and_then(Value::as_str).unwrap_or("");
            match event_type {
                "content_block_delta" => {
                    let text = value
                        .get("delta")
                        .and_then(|d| d.get("text"))
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    Ok(Some(StreamChunk {
                        id: String::from("anthropic"),
                        choices: alloc::vec![StreamChoice {
                            index: 0,
                            delta: Delta {
                                role: None,
                                content: Some(String::from(text)),
                            },
                            finish_reason: None,
                        }],
                    }))
                }
                "message_delta" => {
                    let stop_reason = value
                        .get("delta")
                        .and_then(|d| d.get("stop_reason"))
                        .and_then(Value::as_str)
                        .map(String::from);
                    Ok(Some(StreamChunk {
                        id: String::from("anthropic"),
                        choices: alloc::vec![StreamChoice {
                            index: 0,
                            delta: Delta {
                                role: None,
                                content: None
                            },
                            finish_reason: stop_reason,
                        }],
                    }))
                }
                _ => Ok(None),
            }
        }
        Provider::Ollama => unreachable!("Ollama streaming is handled via NDJSON, not SSE"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{ChatRequest, Message, Role};
    use alloc::vec;

    fn sample_request() -> ChatRequest {
        ChatRequest {
            model: String::from("test-model"),
            messages: vec![
                Message {
                    role: Role::System,
                    content: String::from("be nice"),
                },
                Message {
                    role: Role::User,
                    content: String::from("hello"),
                },
            ],
            temperature: Some(0.5),
            max_tokens: Some(128),
            stream: None,
        }
    }

    #[test]
    fn test_endpoint_per_provider() {
        assert_eq!(
            endpoint("https://api.openai.com/v1", Provider::OpenAi),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            endpoint("https://api.anthropic.com/v1/", Provider::Anthropic),
            "https://api.anthropic.com/v1/messages"
        );
        assert_eq!(
            endpoint("http://localhost:11434", Provider::Ollama),
            "http://localhost:11434/api/chat"
        );
    }

    #[test]
    fn test_build_body_openai_passes_through() {
        let body = build_body(Provider::OpenAi, &sample_request(), true);
        assert_eq!(body["model"], "test-model");
        assert_eq!(body["stream"], true);
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][1]["content"], "hello");
    }

    #[test]
    fn test_build_body_anthropic_splits_system_message() {
        let body = build_body(Provider::Anthropic, &sample_request(), false);
        assert_eq!(body["system"], "be nice");
        assert_eq!(body["messages"].as_array().unwrap().len(), 1);
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["max_tokens"], 128);
    }

    #[test]
    fn test_build_body_ollama_native_shape() {
        let body = build_body(Provider::Ollama, &sample_request(), true);
        assert_eq!(body["model"], "test-model");
        assert_eq!(body["messages"].as_array().unwrap().len(), 2);
        assert_eq!(body["stream"], true);
    }

    #[test]
    fn test_parse_response_openai() {
        let text = r#"{"id":"abc","choices":[{"index":0,"message":{"role":"assistant","content":"hi"},"finish_reason":"stop"}],"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":3}}"#;
        let resp = parse_response(Provider::OpenAi, text).unwrap();
        assert_eq!(resp.choices[0].message.content, "hi");
        assert_eq!(resp.usage.unwrap().total_tokens, 3);
    }

    #[test]
    fn test_parse_response_anthropic() {
        let text = r#"{"id":"msg_1","content":[{"type":"text","text":"hi there"}],"stop_reason":"end_turn","usage":{"input_tokens":10,"output_tokens":5}}"#;
        let resp = parse_response(Provider::Anthropic, text).unwrap();
        assert_eq!(resp.choices[0].message.content, "hi there");
        assert_eq!(resp.choices[0].finish_reason.as_deref(), Some("end_turn"));
        assert_eq!(resp.usage.unwrap().total_tokens, 15);
    }

    #[test]
    fn test_parse_response_ollama() {
        let text = r#"{"model":"llama3","message":{"role":"assistant","content":"hi"},"done":true,"done_reason":"stop","prompt_eval_count":4,"eval_count":6}"#;
        let resp = parse_response(Provider::Ollama, text).unwrap();
        assert_eq!(resp.choices[0].message.content, "hi");
        assert_eq!(resp.usage.unwrap().total_tokens, 10);
    }

    #[test]
    fn test_parse_stream_event_openai_done_sentinel() {
        assert!(parse_stream_event(Provider::OpenAi, "[DONE]")
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_parse_stream_event_anthropic_content_delta() {
        let data = r#"{"type":"content_block_delta","delta":{"text":"chunk"}}"#;
        let chunk = parse_stream_event(Provider::Anthropic, data)
            .unwrap()
            .unwrap();
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("chunk"));
    }

    #[test]
    fn test_parse_ollama_stream_line_done() {
        let line = r#"{"message":{"content":"done text"},"done":true,"done_reason":"stop"}"#;
        let chunk = parse_ollama_stream_line(line).unwrap().unwrap();
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("done text"));
        assert_eq!(chunk.choices[0].finish_reason.as_deref(), Some("stop"));
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(&Error::RateLimited));
        assert!(is_retryable(&Error::Provider {
            code: 503,
            message: String::new()
        }));
        assert!(!is_retryable(&Error::Provider {
            code: 400,
            message: String::new()
        }));
        assert!(!is_retryable(&Error::Unauthorized));
        assert!(!is_retryable(&Error::InvalidRequest(String::new())));
    }

    #[tokio::test]
    async fn test_with_retry_succeeds_after_transient_failures() {
        let attempts = core::cell::Cell::new(0);
        let config = RetryConfig {
            max_attempts: 3,
            base_delay_ms: 1,
        };
        let result: Result<&str, Error> = with_retry(&config, || {
            attempts.set(attempts.get() + 1);
            async {
                if attempts.get() < 3 {
                    Err(Error::RateLimited)
                } else {
                    Ok("ok")
                }
            }
        })
        .await;

        assert_eq!(result.unwrap(), "ok");
        assert_eq!(attempts.get(), 3);
    }

    #[tokio::test]
    async fn test_with_retry_gives_up_after_max_attempts() {
        let attempts = core::cell::Cell::new(0);
        let config = RetryConfig {
            max_attempts: 2,
            base_delay_ms: 1,
        };
        let result: Result<&str, Error> = with_retry(&config, || {
            attempts.set(attempts.get() + 1);
            async { Err(Error::RateLimited) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.get(), 2);
    }

    #[tokio::test]
    async fn test_with_retry_does_not_retry_non_retryable_errors() {
        let attempts = core::cell::Cell::new(0);
        let config = RetryConfig {
            max_attempts: 5,
            base_delay_ms: 1,
        };
        let result: Result<&str, Error> = with_retry(&config, || {
            attempts.set(attempts.get() + 1);
            async { Err(Error::Unauthorized) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.get(), 1);
    }
}
