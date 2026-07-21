use alloc::string::String;

/// A single memory entry.
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub tags: alloc::vec::Vec<String>,
    pub timestamp: u64,
    pub access_count: u64,
    pub score: f32,
}

impl MemoryEntry {
    pub fn new(content: &str, tags: &[&str]) -> Self {
        Self {
            id: alloc::format!("mem_{}", alloc::string::String::from(content).len()),
            content: String::from(content),
            tags: tags.iter().map(|t| String::from(*t)).collect(),
            timestamp: 0,
            access_count: 0,
            score: 1.0,
        }
    }

    pub fn with_id(id: &str, content: &str, tags: &[&str]) -> Self {
        Self {
            id: String::from(id),
            content: String::from(content),
            tags: tags.iter().map(|t| String::from(*t)).collect(),
            timestamp: 0,
            access_count: 0,
            score: 1.0,
        }
    }
}
