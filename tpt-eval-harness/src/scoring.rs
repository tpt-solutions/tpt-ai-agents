/// Score for a single evaluation sample.
#[derive(Debug, Clone)]
pub struct Score {
    pub value: f64,
    pub details: alloc::string::String,
}

/// Scorer trait for evaluating model outputs.
pub trait Scorer {
    fn score(&self, predicted: &str, expected: &str) -> Score;
}

/// Exact match scorer.
pub struct ExactMatchScorer;

impl Scorer for ExactMatchScorer {
    fn score(&self, predicted: &str, expected: &str) -> Score {
        let exact = predicted.trim() == expected.trim();
        Score {
            value: if exact { 1.0 } else { 0.0 },
            details: if exact {
                alloc::string::String::from("exact match")
            } else {
                alloc::format!("mismatch: predicted='{predicted}', expected='{expected}'")
            },
        }
    }
}

/// Contains match scorer.
pub struct ContainsScorer;

impl Scorer for ContainsScorer {
    fn score(&self, predicted: &str, expected: &str) -> Score {
        let contains = predicted.contains(expected.trim());
        Score {
            value: if contains { 1.0 } else { 0.0 },
            details: if contains {
                alloc::string::String::from("contains match")
            } else {
                alloc::format!("not contained: '{expected}' not in '{predicted}'")
            },
        }
    }
}
