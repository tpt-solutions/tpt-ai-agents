/// A parsed SSE event.
#[derive(Debug, Clone, PartialEq)]
pub struct SseEvent {
    pub event_type: Option<alloc::string::String>,
    pub data: alloc::string::String,
}

impl SseEvent {
    pub fn data_str(&self) -> &str {
        &self.data
    }
}
