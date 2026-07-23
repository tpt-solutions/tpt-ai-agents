# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-07-23

### Added

- Initial release: `PgVectorStore` implementing `tpt_vector_store_traits::VectorStore`
- Async search, upsert, delete, and collection info via `sqlx`
- pgvector `vector <=>` distance operator for cosine similarity
- JSONB payload support with basic filter conditions
- `no_std` default with `std`-gated adapter
