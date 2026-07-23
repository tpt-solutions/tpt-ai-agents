//! The real [`QdrantVectorStore`] adapter (requires the `std` feature).

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use qdrant_client::qdrant::{
    DeletePointsBuilder, PointId, PointStruct, PointsIdsList, QueryPointsBuilder,
    UpsertPointsBuilder,
};
use qdrant_client::Qdrant;
use tpt_vector_store_traits::{CollectionInfo, CosineDistance, Query, SearchResult, VectorStore};

use crate::Error;

/// [`VectorStore`] backed by a real Qdrant instance.
pub struct QdrantVectorStore {
    client: Qdrant,
    collection: String,
}

impl QdrantVectorStore {
    /// Connect to a Qdrant instance at `url` (its gRPC endpoint, e.g.
    /// `http://localhost:6334`) and target `collection`. The collection
    /// must already exist — this adapter does not create collections.
    pub async fn new(url: &str, collection: &str) -> Result<Self, Error> {
        let client = Qdrant::from_url(url)
            .build()
            .map_err(|e| Error::Client(e.to_string()))?;
        Ok(Self {
            client,
            collection: collection.to_string(),
        })
    }

    /// Wrap an already-constructed [`Qdrant`] client, e.g. one configured
    /// with an API key or custom TLS settings via [`Qdrant::from_url`].
    pub fn with_client(client: Qdrant, collection: &str) -> Self {
        Self {
            client,
            collection: collection.to_string(),
        }
    }
}

#[async_trait]
impl VectorStore for QdrantVectorStore {
    type Error = Error;
    type Distance = CosineDistance;
    type Payload = serde_json::Value;

    async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Error> {
        let mut builder = QueryPointsBuilder::new(&self.collection)
            .query(query.vector.clone())
            .limit(query.limit as u64)
            .with_payload(query.include_payload.unwrap_or(false));
        if let Some(offset) = query.offset {
            builder = builder.offset(offset as u64);
        }

        let response = self
            .client
            .query(builder)
            .await
            .map_err(|e| Error::Client(e.to_string()))?;

        Ok(response
            .result
            .into_iter()
            .map(|point| {
                let id = point.id.map(point_id_to_string).unwrap_or_default();
                let payload = if point.payload.is_empty() {
                    None
                } else {
                    Some(serde_json::Value::Object(
                        point
                            .payload
                            .into_iter()
                            .map(|(k, v)| (k, qdrant_value_to_json(v)))
                            .collect(),
                    ))
                };
                SearchResult {
                    id,
                    score: point.score,
                    vector: None,
                    payload,
                }
            })
            .collect())
    }

    async fn upsert(
        &self,
        ids: &[&str],
        vectors: &[&[f32]],
        payloads: &[Option<&Self::Payload>],
    ) -> Result<(), Error> {
        if ids.len() != vectors.len() || ids.len() != payloads.len() {
            return Err(tpt_vector_store_traits::Error::DimensionMismatch {
                expected: ids.len(),
                actual: vectors.len().min(payloads.len()),
            }
            .into());
        }

        let points: Vec<PointStruct> = ids
            .iter()
            .zip(vectors.iter())
            .zip(payloads.iter())
            .map(|((id, vector), payload)| {
                let payload = payload
                    .and_then(|p| p.as_object())
                    .cloned()
                    .unwrap_or_default();
                PointStruct::new(id.to_string(), vector.to_vec(), payload)
            })
            .collect();

        self.client
            .upsert_points(UpsertPointsBuilder::new(&self.collection, points))
            .await
            .map_err(|e| Error::Client(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, ids: &[&str]) -> Result<(), Error> {
        let point_ids: Vec<PointId> = ids.iter().map(|id| PointId::from(id.to_string())).collect();
        self.client
            .delete_points(
                DeletePointsBuilder::new(&self.collection).points(PointsIdsList { ids: point_ids }),
            )
            .await
            .map_err(|e| Error::Client(e.to_string()))?;
        Ok(())
    }

    async fn info(&self) -> Result<CollectionInfo, Error> {
        let response = self
            .client
            .collection_info(&self.collection)
            .await
            .map_err(|e| Error::Client(e.to_string()))?;
        let result = response
            .result
            .ok_or_else(|| Error::Client("collection info missing from response".into()))?;
        let dimension = result
            .config
            .as_ref()
            .and_then(|c| c.params.as_ref())
            .and_then(|p| p.vectors_config.as_ref())
            .and_then(|vc| vc.config.as_ref())
            .map(vector_config_dimension)
            .unwrap_or(0);
        Ok(CollectionInfo {
            name: self.collection.clone(),
            vectors_count: result.points_count.unwrap_or(0) as usize,
            dimension,
        })
    }
}

fn point_id_to_string(id: PointId) -> String {
    use qdrant_client::qdrant::point_id::PointIdOptions;
    match id.point_id_options {
        Some(PointIdOptions::Num(n)) => n.to_string(),
        Some(PointIdOptions::Uuid(uuid)) => uuid,
        None => String::new(),
    }
}

fn vector_config_dimension(config: &qdrant_client::qdrant::vectors_config::Config) -> usize {
    use qdrant_client::qdrant::vectors_config::Config;
    match config {
        Config::Params(params) => params.size as usize,
        Config::ParamsMap(_) => 0,
    }
}

fn qdrant_value_to_json(value: qdrant_client::qdrant::Value) -> serde_json::Value {
    use qdrant_client::qdrant::value::Kind;
    match value.kind {
        Some(Kind::NullValue(_)) | None => serde_json::Value::Null,
        Some(Kind::BoolValue(b)) => serde_json::Value::Bool(b),
        Some(Kind::IntegerValue(i)) => serde_json::Value::from(i),
        Some(Kind::DoubleValue(d)) => serde_json::Number::from_f64(d)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Some(Kind::StringValue(s)) => serde_json::Value::String(s),
        Some(Kind::ListValue(list)) => {
            serde_json::Value::Array(list.values.into_iter().map(qdrant_value_to_json).collect())
        }
        Some(Kind::StructValue(s)) => serde_json::Value::Object(
            s.fields
                .into_iter()
                .map(|(k, v)| (k, qdrant_value_to_json(v)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant::point_id::PointIdOptions;
    use qdrant_client::qdrant::value::Kind;

    #[test]
    fn test_point_id_to_string_numeric() {
        let id = PointId {
            point_id_options: Some(PointIdOptions::Num(42)),
        };
        assert_eq!(point_id_to_string(id), "42");
    }

    #[test]
    fn test_point_id_to_string_uuid() {
        let id = PointId {
            point_id_options: Some(PointIdOptions::Uuid("abc-123".into())),
        };
        assert_eq!(point_id_to_string(id), "abc-123");
    }

    #[test]
    fn test_point_id_to_string_missing() {
        let id = PointId {
            point_id_options: None,
        };
        assert_eq!(point_id_to_string(id), "");
    }

    #[test]
    fn test_qdrant_value_to_json_scalars() {
        let bool_val = qdrant_client::qdrant::Value {
            kind: Some(Kind::BoolValue(true)),
        };
        assert_eq!(qdrant_value_to_json(bool_val), serde_json::json!(true));

        let str_val = qdrant_client::qdrant::Value {
            kind: Some(Kind::StringValue("hello".into())),
        };
        assert_eq!(qdrant_value_to_json(str_val), serde_json::json!("hello"));

        let null_val = qdrant_client::qdrant::Value { kind: None };
        assert_eq!(qdrant_value_to_json(null_val), serde_json::Value::Null);
    }

    #[test]
    fn test_qdrant_value_to_json_nested_list() {
        let list_val = qdrant_client::qdrant::Value {
            kind: Some(Kind::ListValue(qdrant_client::qdrant::ListValue {
                values: alloc::vec![
                    qdrant_client::qdrant::Value {
                        kind: Some(Kind::IntegerValue(1)),
                    },
                    qdrant_client::qdrant::Value {
                        kind: Some(Kind::IntegerValue(2)),
                    },
                ],
            })),
        };
        assert_eq!(qdrant_value_to_json(list_val), serde_json::json!([1, 2]));
    }

    #[test]
    fn test_upsert_rejects_mismatched_lengths() {
        // Exercised via a runtime error path, not a live server: the
        // length check happens before any network call.
        let result = tokio_test_block_on(async {
            let store = QdrantVectorStore::with_client(
                Qdrant::from_url("http://127.0.0.1:1").build().unwrap(),
                "test",
            );
            store
                .upsert(&["a", "b"], &[&[1.0, 2.0]], &[None, None])
                .await
        });
        assert!(result.is_err());
    }

    fn tokio_test_block_on<F: core::future::Future>(fut: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(fut)
    }
}
