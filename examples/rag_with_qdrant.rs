//! Minimal example: RAG chunking + agent memory backed by a real Qdrant
//! instance, via `tpt-vector-store-qdrant`.
//!
//! Unlike the other examples, this one needs a real, reachable Qdrant
//! instance with an existing collection — there's no mock for a vector
//! database in this workspace. Start one locally with:
//!
//! ```sh
//! docker run -p 6334:6334 qdrant/qdrant
//! ```
//!
//! and create a collection named `example_docs` with a 3-dimensional
//! vector (matching the toy embeddings below) before running this example:
//!
//! ```sh
//! curl -X PUT http://localhost:6333/collections/example_docs \
//!   -H 'Content-Type: application/json' \
//!   -d '{"vectors": {"size": 3, "distance": "Cosine"}}'
//! ```
//!
//! Run with `cargo run --example rag_with_qdrant`.

use tpt_agent_memory::{MemoryEntry, VectorBackedMemoryStore};
use tpt_rag_pipeline::{ChunkConfig, Chunker};
use tpt_vector_store_qdrant::QdrantVectorStore;

/// Stand-in for a real embedding model: maps a chunk to a toy 3-dimensional
/// vector so this example doesn't depend on a network embedding call.
fn toy_embedding(text: &str) -> Vec<f32> {
    let len = text.len() as f32;
    let words = text.split_whitespace().count() as f32;
    vec![len, words, len / words.max(1.0)]
}

#[tokio::main]
async fn main() {
    let document = "Rust is a systems programming language focused on safety, speed, and \
        concurrency. It achieves memory safety without garbage collection through its \
        ownership system. The borrow checker enforces strict rules at compile time.";

    let chunker = Chunker::new(ChunkConfig::new(20, 5));
    let chunks = chunker.chunk(document).expect("chunking failed");
    println!("Chunked document into {} pieces", chunks.len());

    let backend = QdrantVectorStore::new("http://localhost:6334", "example_docs")
        .await
        .expect(
            "failed to connect to Qdrant — is it running? see this file's \
             module doc comment for setup instructions",
        );
    let mut memory = VectorBackedMemoryStore::new(backend);

    for chunk in &chunks {
        let embedding = toy_embedding(&chunk.text);
        memory
            .insert(MemoryEntry::new(&chunk.text, &["document"]).with_embedding(embedding))
            .await
            .expect("insert failed");
    }
    println!("Upserted {} chunks into Qdrant", chunks.len());

    let query_embedding = toy_embedding("ownership system memory safety");
    let results = memory
        .search_semantic(query_embedding, 3)
        .await
        .expect("search failed");
    println!("Top {} semantically similar chunks:", results.len());
    for entry in &results {
        println!("  - {}", &entry.content[..60.min(entry.content.len())]);
    }
}
