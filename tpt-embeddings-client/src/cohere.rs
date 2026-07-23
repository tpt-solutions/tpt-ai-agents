//! Cohere embeddings provider (requires the `std` feature).

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use serde::Deserialize;

use crate::traits::{EmbeddingMeta, EmbeddingRequest, EmbeddingResponse, EmbeddingResult};
use crate::{EmbeddingsClient, Error};

/// Embeddings client for the Cohere API.
///
/// Supports the `/v1/embed` endpoint. Use with models like
/// `embed-english-v3.0` or `embed-multilingual-v3.0`.
pub struct CohereEmbeddings {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl CohereEmbeddings {
    /// Create a new Cohere embeddings client.
    ///
    /// `base_url` is typically `https://api.cohere.com` but can be
    /// overridden for compatible providers.
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// Wrap an existing [`reqwest::Client`].
    pub fn with_client(base_url: &str, api_key: &str, client: reqwest::Client) -> Self {
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CohereResponse {
    embeddings: Vec<Vec<f32>>,
    id: Option<String>,
    meta: Option<CohereMeta>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CohereMeta {
    api_version: Option<CohereApiVersion>,
    billed_units: Option<CohereBilledUnits>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CohereApiVersion {
    version: Option<String>,
}

#[derive(Deserialize)]
struct CohereBilledUnits {
    input_tokens: Option<u32>,
}

#[async_trait]
impl EmbeddingsClient for CohereEmbeddings {
    async fn embed(&self, request: &EmbeddingRequest) -> Result<EmbeddingResponse, Error> {
        if request.inputs.is_empty() {
            return Err(Error::EmptyBatch);
        }

        let body = serde_json::json!({
            "model": &request.model,
            "texts": &request.inputs,
            "input_type": "search_document",
            "embedding_types": ["float"],
        });

        let url = format!("{}/v1/embed", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", alloc::format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
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

        let parsed: CohereResponse = response
            .json()
            .await
            .map_err(|e| Error::Serialization(e.to_string()))?;

        if parsed.embeddings.is_empty() {
            return Err(Error::NoData);
        }

        let embeddings = parsed
            .embeddings
            .into_iter()
            .enumerate()
            .map(|(i, embedding)| EmbeddingResult {
                id: request.inputs.get(i).cloned().unwrap_or_default(),
                embedding,
            })
            .collect();

        let total_tokens = parsed
            .meta
            .and_then(|m| m.billed_units)
            .and_then(|u| u.input_tokens);

        Ok(EmbeddingResponse {
            embeddings,
            meta: EmbeddingMeta {
                total_tokens,
                model: Some(request.model.clone()),
            },
        })
    }
}
