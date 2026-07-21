/// Token usage statistics.
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

/// Evaluation metrics.
#[derive(Debug, Clone)]
pub struct EvalMetrics {
    pub total_samples: usize,
    pub correct: usize,
    pub token_usage: TokenUsage,
    pub latency_ms: u64,
}

impl EvalMetrics {
    pub fn new() -> Self {
        Self {
            total_samples: 0,
            correct: 0,
            token_usage: TokenUsage::default(),
            latency_ms: 0,
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_samples == 0 {
            0.0
        } else {
            self.correct as f64 / self.total_samples as f64
        }
    }
}

impl Default for EvalMetrics {
    fn default() -> Self {
        Self::new()
    }
}
