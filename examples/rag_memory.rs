//! Integration example: rag-pipeline + agent-memory
//!
//! Demonstrates chunking a document and storing/retrieving from memory.

use tpt_agent_memory::{MemoryEntry, MemoryStore, SearchQuery};
use tpt_rag_pipeline::{ChunkConfig, Chunker};

fn main() {
    // 1. Chunk a document using the RAG pipeline
    let config = ChunkConfig::new(20, 5);
    let chunker = Chunker::new(config);

    let document = "Rust is a systems programming language focused on safety, speed, and concurrency. \
        It achieves memory safety without garbage collection through its ownership system. \
        The borrow checker enforces strict rules at compile time, preventing data races. \
        This makes Rust ideal for building reliable and efficient software.";

    let chunks = chunker.chunk(document).expect("chunking failed");
    println!("Document chunked into {} pieces:", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  [{i}] ({} tokens) {}", chunk.token_count, &chunk.text[..40.min(chunk.text.len())]);
    }

    // 2. Store chunks in agent memory
    let mut store = MemoryStore::new();
    for chunk in &chunks {
        store.insert(MemoryEntry::new(&chunk.text, &["document", "chunk"]));
    }
    println!("\nStored {} entries in memory", store.len());

    // 3. Search memory
    let query = SearchQuery::new("ownership");
    let results = store.search(&query);
    println!("\nSearch for 'ownership' returned {} results:", results.len());
    for result in &results {
        println!("  - {}", &result.content[..60.min(result.content.len())]);
    }

    // 4. Search for concurrency
    let query = SearchQuery::new("concurrency");
    let results = store.search(&query);
    println!("\nSearch for 'concurrency' returned {} results:", results.len());
    for result in &results {
        println!("  - {}", &result.content[..60.min(result.content.len())]);
    }
}
