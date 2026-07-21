use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Graph-based memory structure for relationships.
pub struct MemoryGraph {
    nodes: BTreeMap<String, Node>,
    edges: Vec<Edge>,
}

struct Node {
    #[allow(dead_code)]
    id: String,
    label: String,
}

struct Edge {
    from: String,
    to: String,
    #[allow(dead_code)]
    relationship: String,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: Vec::new(),
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

    pub fn add_edge(&mut self, from: &str, to: &str, relationship: &str) -> bool {
        if !self.nodes.contains_key(from) || !self.nodes.contains_key(to) {
            return false;
        }
        self.edges.push(Edge {
            from: String::from(from),
            to: String::from(to),
            relationship: String::from(relationship),
        });
        true
    }

    /// Returns the IDs of nodes that are direct neighbors of the given node.
    pub fn neighbors(&self, id: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| e.from == id)
            .map(|e| e.to.as_str())
            .collect()
    }

    pub fn has_node(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn node_label(&self, id: &str) -> Option<&str> {
        self.nodes.get(id).map(|n| n.label.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node_and_edge() {
        let mut graph = MemoryGraph::new();
        graph.add_node("a", "Alice");
        graph.add_node("b", "Bob");
        assert!(graph.add_edge("a", "b", "knows"));
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_add_edge_validates_nodes() {
        let mut graph = MemoryGraph::new();
        graph.add_node("a", "Alice");
        assert!(!graph.add_edge("a", "missing", "knows"));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_neighbors_returns_ids() {
        let mut graph = MemoryGraph::new();
        graph.add_node("a", "Alice");
        graph.add_node("b", "Bob");
        graph.add_node("c", "Charlie");
        graph.add_edge("a", "b", "knows");
        graph.add_edge("a", "c", "knows");
        let neighbors = graph.neighbors("a");
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&"b"));
        assert!(neighbors.contains(&"c"));
    }

    #[test]
    fn test_node_label() {
        let mut graph = MemoryGraph::new();
        graph.add_node("a", "Alice");
        assert_eq!(graph.node_label("a"), Some("Alice"));
        assert_eq!(graph.node_label("missing"), None);
    }
}
