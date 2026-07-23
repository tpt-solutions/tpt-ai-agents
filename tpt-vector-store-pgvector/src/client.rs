//! The real [`PgVectorStore`] adapter (requires the `std` feature).

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use sqlx::PgPool;
use tpt_vector_store_traits::{
    CollectionInfo, CosineDistance, FilterOp, Query, SearchResult, VectorStore,
};

use crate::Error;

/// [`VectorStore`] backed by a PostgreSQL database with the pgvector extension.
///
/// Expects the table to already exist with the schema:
///
/// ```sql
/// CREATE EXTENSION IF NOT EXISTS vector;
///
/// CREATE TABLE IF NOT EXISTS <table_name> (
///     id TEXT PRIMARY KEY,
///     embedding vector(<dimension>),
///     payload JSONB
/// );
/// ```
///
/// The table is **not** created automatically — manage migrations separately.
pub struct PgVectorStore {
    pool: PgPool,
    table: String,
    dimension: usize,
}

impl PgVectorStore {
    /// Connect to a PostgreSQL database at `database_url` and target `table`
    /// with vectors of the given `dimension`.
    pub async fn new(database_url: &str, table: &str, dimension: usize) -> Result<Self, Error> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| Error::Sql(e.to_string()))?;
        Ok(Self {
            pool,
            table: table.to_string(),
            dimension,
        })
    }

    /// Wrap an already-constructed [`PgPool`].
    pub fn with_pool(pool: PgPool, table: &str, dimension: usize) -> Self {
        Self {
            pool,
            table: table.to_string(),
            dimension,
        }
    }
}

fn filter_op_to_sql(op: &FilterOp) -> &'static str {
    match op {
        FilterOp::Eq => "=",
        FilterOp::Ne => "!=",
        FilterOp::Gt => ">",
        FilterOp::Gte => ">=",
        FilterOp::Lt => "<",
        FilterOp::Lte => "<=",
        FilterOp::Contains => "@>",
    }
}

#[async_trait]
impl VectorStore for PgVectorStore {
    type Error = Error;
    type Distance = CosineDistance;
    type Payload = serde_json::Value;

    async fn search(&self, query: &Query) -> Result<Vec<SearchResult<Self::Payload>>, Error> {
        let embed = pgvector::Vector::from(query.vector.clone());

        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new(alloc::format!(
                "SELECT id, 1 - (embedding <=> $1) AS score, payload FROM {} ORDER BY embedding <=> $1",
                self.table
            ));
        builder.push_bind(embed);

        let mut param_idx = 1;
        let has_filter = query.filter.as_ref().is_some_and(|f| !f.must.is_empty());
        if has_filter {
            builder.push(" WHERE ");
            let mut separated = builder.separated(" AND ");
            for cond in &query.filter.as_ref().unwrap().must {
                separated.push(alloc::format!(
                    "payload->>'{}' {} ",
                    cond.key,
                    filter_op_to_sql(&cond.op),
                ));
                param_idx += 1;
                separated.push_bind_unseparated(alloc::format!("${}", param_idx));
            }
        }

        param_idx += 1;
        builder.push(alloc::format!(" LIMIT ${}", param_idx));
        builder.push_bind(query.limit as i64);

        let rows: Vec<(String, f32, Option<serde_json::Value>)> =
            builder.build_query_as().fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|(id, score, payload)| SearchResult {
                id,
                score,
                vector: None,
                payload,
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

        for i in 0..ids.len() {
            let embed = pgvector::Vector::from(vectors[i].to_vec());
            let payload_json = payloads[i]
                .map(|p| (*p).clone())
                .unwrap_or(serde_json::Value::Null);
            let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
                sqlx::QueryBuilder::new(alloc::format!(
                    "INSERT INTO {} (id, embedding, payload) VALUES ($1, $2, $3) \
                     ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding, payload = EXCLUDED.payload",
                    self.table
                ));
            builder.push_bind(ids[i]);
            builder.push_bind(embed);
            builder.push_bind(payload_json);
            builder.build().execute(&self.pool).await?;
        }
        Ok(())
    }

    async fn delete(&self, ids: &[&str]) -> Result<(), Error> {
        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new(alloc::format!("DELETE FROM {} WHERE id IN (", self.table));
        let mut separated = builder.separated(", ");
        for id in ids {
            separated.push_bind(*id);
        }
        separated.push_unseparated(")");
        builder.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn info(&self) -> Result<CollectionInfo, Error> {
        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new(alloc::format!("SELECT COUNT(*) FROM {}", self.table));
        let row: (i64,) = builder.build_query_scalar().fetch_one(&self.pool).await?;

        Ok(CollectionInfo {
            name: self.table.clone(),
            vectors_count: row.0 as usize,
            dimension: self.dimension,
        })
    }
}
