# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-23

### Added

- `EmbeddingsClient` trait with batch embedding support
- `OpenAiEmbeddings` adapter for the OpenAI `/v1/embeddings` endpoint
- `CohereEmbeddings` adapter for the Cohere `/v1/embed` endpoint
- `EmbeddingRequest`, `EmbeddingResponse`, `EmbeddingResult`, `EmbeddingMeta` types
- `no_std` base with `std` feature for networking
