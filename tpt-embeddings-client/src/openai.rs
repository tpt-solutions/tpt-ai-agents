//! OpenAI embeddings provider (requires the `std` feature).

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use serde::Deserialize;

use crate::traits::{EmbeddingMeta, EmbeddingRequest, EmbeddingResponse, EmbeddingResult};
use crate::{EmbeddingsClient, Error};

/// Embeddings client for the OpenAI API.
///
/// Supports the `/v1/embeddings` endpoint. Use with any model that supports
/// embeddings (e.g. `text-embedding-3-small`, `text-embedding-3-large`).
pub struct OpenAiEmbeddings {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl OpenAiEmbeddings {
    /// Create a new OpenAI embeddings client.
    ///
    /// `base_url` is typically `https://api.openai.com/v1` but can be
    /// overridden for compatible providers (e.g. Azure OpenAI, local
    /// proxies).
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// Wrap an existing [`reqwest::Client`], e.g. one with custom TLS
    /// or proxy settings.
    pub fn with_client(base_url: &str, api_key: &str, client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct OpenAiResponse {
    data: Vec<OpenAiEmbedding>,
    model: String,
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize)]
struct OpenAiEmbedding {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct OpenAiUsage {
    prompt_tokens: u32,
    total_tokens: u32,
}

#[async_trait]
impl EmbeddingsClient for OpenAiEmbeddings {
    async fn embed(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse, Error> {
        if request.inputs.is_empty() {
            return Err(Error::EmptyBatch);
        }

        let mut body = serde_json::json!({
            "model": &request.model,
            "input": &request.inputs,
        });
        if let Some(dims) = request.dimensions {
            body["dimensions"] = serde_json::json!(dims);
        }

        let url = format!("{}/embeddings", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", alloc::format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let msg = response.text().await.unwrap_or_default();
            return Err(Error::Provider {
                code: status.as_u16(),
                message: msg,
            });
        }

        let parsed: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| Error::Serialization(e.to_string()))?;

        let mut sorted = parsed.data;
        sorted.sort_by_key(|e| e.index);

        let embeddings = sorted
            .into_iter()
            .enumerate()
            .map(|(i, e)| EmbeddingResult {
                id: request.inputs.get(i).cloned().unwrap_or_default(),
                embedding: e.embedding,
            })
            .collect();

        Ok(EmbeddingResponse {
            embeddings,
            meta: EmbeddingMeta {
                total_tokens: parsed.usage.map(|u| u.total_tokens),
                model: Some(parsed.model),
            },
        })
    }
}
