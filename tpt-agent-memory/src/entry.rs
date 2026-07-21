use alloc::string::String;

/// Global counter for generating unique entry IDs.
static mut NEXT_ID: u64 = 0;

fn next_id() -> u64 {
    // SAFETY: This is a simple monotonic counter. In a real implementation,
    // you'd use atomics or a UUID library.
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

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
    /// Create a new memory entry with an auto-generated unique ID.
    pub fn new(content: &str, tags: &[&str]) -> Self {
        Self {
            id: alloc::format!("mem_{}", next_id()),
            content: String::from(content),
            tags: tags.iter().map(|t| String::from(*t)).collect(),
            timestamp: 0,
            access_count: 0,
            score: 1.0,
        }
    }

    /// Create a new memory entry with a caller-specified ID.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_ids() {
        let a = MemoryEntry::new("hello", &[]);
        let b = MemoryEntry::new("hello", &[]);
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn test_with_id() {
        let entry = MemoryEntry::with_id("custom_id", "content", &["tag"]);
        assert_eq!(entry.id, "custom_id");
    }
}
