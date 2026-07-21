/// A recorded SSE response for the mock server.
pub struct RecordedResponse {
    pub chunks: alloc::vec::Vec<SseChunk>,
}

/// A single SSE chunk.
#[derive(Debug, Clone)]
pub struct SseChunk {
    pub event_type: Option<alloc::string::String>,
    pub data: alloc::string::String,
}

impl RecordedResponse {
    pub fn sse_stream(text: &str) -> Self {
        let chunks: alloc::vec::Vec<_> = text
            .split_whitespace()
            .map(|word| SseChunk {
                event_type: Some(alloc::string::String::from("message")),
                data: alloc::format!("{{\"text\":\"{word}\"}}"),
            })
            .collect();
        Self { chunks }
    }

    pub fn json_response(body: &str) -> Self {
        Self {
            chunks: alloc::vec![SseChunk {
                event_type: None,
                data: alloc::string::String::from(body),
            }],
        }
    }
}
