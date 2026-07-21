use alloc::collections::BTreeMap;
use alloc::string::String;

/// Graph-based memory structure for relationships.
pub struct MemoryGraph {
    nodes: BTreeMap<String, Node>,
    edges: alloc::vec::Vec<Edge>,
}

struct Node {
    id: String,
    label: String,
}

struct Edge {
    from: String,
    to: String,
    relationship: String,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: alloc::vec::Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, label: &str) {
        self.nodes.insert(
            String::from(id),
            Node {
                id: String::from(id),
                label: String::from(label),
            },
        );
    }

    pub fn add_edge(&mut self, from: &str, to: &str, relationship: &str) {
        self.edges.push(Edge {
            from: String::from(from),
            to: String::from(to),
            relationship: String::from(relationship),
        });
    }

    pub fn neighbors(&self, id: &str) -> alloc::vec::Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.from == id)
            .filter_map(|e| self.nodes.get(&e.to).map(|n| n.label.as_str()))
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

impl Default for MemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}
