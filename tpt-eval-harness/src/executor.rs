use crate::{EvalMetrics, EvalSample};
use alloc::vec::Vec;

/// Parallel execution engine for eval tasks.
pub struct ParallelExecutor {
    max_concurrency: usize,
}

impl ParallelExecutor {
    pub fn new(max_concurrency: usize) -> Self {
        Self { max_concurrency }
    }

    pub fn max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    /// Execute evaluation samples in batches.
    pub fn execute<F>(&self, samples: Vec<EvalSample>, eval_fn: F) -> EvalMetrics
    where
        F: Fn(&EvalSample) -> EvalSample,
    {
        let mut metrics = EvalMetrics::new();
        metrics.total_samples = samples.len();

        // Process in batches (sequential for now; true parallelism needs async/tokio)
        for chunk in samples.chunks(self.max_concurrency) {
            for sample in chunk {
                let result = eval_fn(sample);
                if !result.output.is_empty() && result.output == result.expected {
                    metrics.correct += 1;
                }
                metrics.token_usage.prompt_tokens += result.prompt_tokens;
                metrics.token_usage.completion_tokens += result.completion_tokens;
                metrics.token_usage.total_tokens += result.prompt_tokens + result.completion_tokens;
            }
        }

        metrics
    }
}

impl Default for ParallelExecutor {
    fn default() -> Self {
        Self::new(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_executor() {
        let executor = ParallelExecutor::new(2);
        let samples = vec![
            EvalSample::new("q1", "a", "a"),
            EvalSample::new("q2", "b", "b"),
            EvalSample::new("q3", "c", "wrong"),
        ];
        let metrics = executor.execute(samples, |s| {
            EvalSample {
                input: s.input.clone(),
                expected: s.expected.clone(),
                output: s.expected.clone(), // pretend the model got it right
                prompt_tokens: 10,
                completion_tokens: 5,
            }
        });
        assert_eq!(metrics.correct, 3);
        assert_eq!(metrics.token_usage.total_tokens, 45);
    }
}
