//! Real HTTP transport for `SseClient` (requires the `std` feature).

use crate::response::{Choice, Delta, StreamChoice, Usage};
use crate::{ChatRequest, ChatResponse, Error, Message, Provider, Role, SseParser, StreamChunk};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use futures::Stream;
use serde::Deserialize;
use serde_json::Value;

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
        code => Error::Provider { code, message: body },
    }
}

/// Send a non-streaming chat completion request and parse the provider's response
/// into the unified [`ChatResponse`] shape.
pub(crate) async fn send(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    provider: Provider,
    request: &ChatRequest,
) -> Result<ChatResponse, Error> {
    let url = endpoint(base_url, provider);
    let body = build_body(provider, request, false);

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
) -> Result<impl Stream<Item = Result<StreamChunk, Error>>, Error> {
    let url = endpoint(base_url, provider);
    let body = build_body(provider, request, true);

    let req = client.post(&url).json(&body);
    let req = auth_headers(provider, api_key, req);
    let resp = req.send().await?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(map_error_response(status, text).await);
    }

    let byte_stream: core::pin::Pin<Box<dyn Stream<Item = reqwest::Result<bytes::Bytes>> + Send>> =
        Box::pin(resp.bytes_stream());
    let state = (byte_stream, SseParser::new(), String::new(), provider, false);

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
                    loop {
                        if let Some(pos) = line_buf.find('\n') {
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
                                    return Some((Ok(chunk), (byte_stream, parser, line_buf, provider, done)));
                                }
                                Ok(None) => continue,
                                Err(e) => return Some((Err(e), (byte_stream, parser, line_buf, provider, true))),
                            }
                        } else {
                            break;
                        }
                    }
                    match byte_stream.next().await {
                        Some(Ok(bytes)) => {
                            line_buf.push_str(&String::from_utf8_lossy(&bytes));
                            continue;
                        }
                        Some(Err(e)) => {
                            return Some((Err(Error::from(e)), (byte_stream, parser, line_buf, provider, true)))
                        }
                        None => return None,
                    }
                }

                match byte_stream.next().await {
                    Some(Ok(bytes)) => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        if let Some(event) = parser.feed(&text) {
                            match parse_stream_event(provider, event.data_str()) {
                                Ok(Some(chunk)) => {
                                    return Some((Ok(chunk), (byte_stream, parser, line_buf, provider, done)))
                                }
                                Ok(None) => continue,
                                Err(e) => {
                                    return Some((Err(e), (byte_stream, parser, line_buf, provider, true)))
                                }
                            }
                        }
                        continue;
                    }
                    Some(Err(e)) => {
                        return Some((Err(Error::from(e)), (byte_stream, parser, line_buf, provider, true)))
                    }
                    None => {
                        if let Some(event) = parser.flush() {
                            match parse_stream_event(provider, event.data_str()) {
                                Ok(Some(chunk)) => {
                                    return Some((Ok(chunk), (byte_stream, parser, line_buf, provider, true)))
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
                content: if content.is_empty() { None } else { Some(content) },
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
                            delta: Delta { role: None, content: None },
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
