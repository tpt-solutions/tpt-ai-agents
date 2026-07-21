/// Configuration for the evaluation harness.
#[derive(Debug, Clone)]
pub struct EvalConfig {
    pub max_concurrency: usize,
    pub timeout_ms: u64,
    pub retry_count: u32,
}

impl EvalConfig {
    pub fn new(max_concurrency: usize, timeout_ms: u64, retry_count: u32) -> Self {
        Self {
            max_concurrency,
            timeout_ms,
            retry_count,
        }
    }
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self {
            max_concurrency: 8,
            timeout_ms: 30_000,
            retry_count: 3,
        }
    }
}
