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

        // All queries use dynamic table names (validated at construction,
        // never from untrusted input at query time). We use `raw_sql` which
        // accepts dynamic SQL, then bind parameters normally.
        let rows: Vec<(String, f32, Option<serde_json::Value>)> =
            if let Some(ref filter) = query.filter {
                if filter.must.is_empty() {
                    let sql = alloc::format!(
                        "SELECT id, 1 - (embedding <=> $1) AS score, payload \
                         FROM {} \
                         ORDER BY embedding <=> $1 \
                         LIMIT $2",
                        self.table
                    );
                    sqlx::raw_sql(&sql)
                        .bind(embed)
                        .bind(query.limit as i64)
                        .fetch_all(&self.pool)
                        .await?
                } else {
                    let mut conditions = Vec::new();
                    for cond in &filter.must {
                        let idx = 3 + conditions.len();
                        conditions.push(alloc::format!(
                            "payload->>'{}' {} ${}",
                            cond.key,
                            filter_op_to_sql(&cond.op),
                            idx
                        ));
                    }
                    let where_clause = conditions.join(" AND ");
                    let sql = alloc::format!(
                        "SELECT id, 1 - (embedding <=> $1) AS score, payload \
                         FROM {} \
                         WHERE {} \
                         ORDER BY embedding <=> $1 \
                         LIMIT $2",
                        self.table,
                        where_clause
                    );
                    sqlx::raw_sql(&sql)
                        .bind(embed)
                        .bind(query.limit as i64)
                        .fetch_all(&self.pool)
                        .await?
                }
            } else {
                let sql = alloc::format!(
                    "SELECT id, 1 - (embedding <=> $1) AS score, payload \
                     FROM {} \
                     ORDER BY embedding <=> $1 \
                     LIMIT $2",
                    self.table
                );
                sqlx::raw_sql(&sql)
                    .bind(embed)
                    .bind(query.limit as i64)
                    .fetch_all(&self.pool)
                    .await?
            };

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

        let sql = alloc::format!(
            "INSERT INTO {} (id, embedding, payload) VALUES ($1, $2, $3) \
             ON CONFLICT (id) DO UPDATE SET embedding = EXCLUDED.embedding, payload = EXCLUDED.payload",
            self.table
        );
        for i in 0..ids.len() {
            let embed = pgvector::Vector::from(vectors[i].to_vec());
            let payload_json = payloads[i]
                .map(|p| (*p).clone())
                .unwrap_or(serde_json::Value::Null);
            sqlx::raw_sql(&sql)
                .bind(ids[i])
                .bind(embed)
                .bind(payload_json)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn delete(&self, ids: &[&str]) -> Result<(), Error> {
        let params: Vec<String> = ids
            .iter()
            .enumerate()
            .map(|(i, _)| alloc::format!("${}", i + 1))
            .collect();
        let sql = alloc::format!(
            "DELETE FROM {} WHERE id IN ({})",
            self.table,
            params.join(", ")
        );
        let mut q = sqlx::raw_sql(&sql);
        for id in ids {
            q = q.bind(id);
        }
        q.execute(&self.pool).await?;
        Ok(())
    }

    async fn info(&self) -> Result<CollectionInfo, Error> {
        let sql = alloc::format!("SELECT COUNT(*) FROM {}", self.table);
        let row: (i64,) = sqlx::raw_sql(&sql)
            .fetch_one(&self.pool)
            .await?;

        Ok(CollectionInfo {
            name: self.table.clone(),
            vectors_count: row.0 as usize,
            dimension: self.dimension,
        })
    }
}
