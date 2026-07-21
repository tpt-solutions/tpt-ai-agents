use alloc::boxed::Box;
use crate::{EvalConfig, EvalMetrics, scoring::{Scorer, ExactMatchScorer}};

/// Main evaluation harness.
pub struct EvalHarness {
    config: EvalConfig,
    scorer: Box<dyn Scorer>,
}

impl EvalHarness {
    pub fn new(config: EvalConfig) -> Self {
        Self {
            config,
            scorer: Box::new(ExactMatchScorer),
        }
    }

    pub fn with_scorer(config: EvalConfig, scorer: Box<dyn Scorer>) -> Self {
        Self { config, scorer }
    }

    pub fn config(&self) -> &EvalConfig {
        &self.config
    }

    /// Evaluate a dataset. Each line should be a JSON object with "input" and "expected" fields.
    pub fn run(&self, dataset: &[EvalSample]) -> EvalMetrics {
        let mut metrics = EvalMetrics::new();
        metrics.total_samples = dataset.len();

        for sample in dataset {
            let score = self.scorer.score(&sample.output, &sample.expected);
            if score.value > 0.5 {
                metrics.correct += 1;
            }
            metrics.token_usage.prompt_tokens += sample.prompt_tokens;
            metrics.token_usage.completion_tokens += sample.completion_tokens;
            metrics.token_usage.total_tokens += sample.prompt_tokens + sample.completion_tokens;
        }

        metrics
    }
}

/// A single evaluation sample with input, expected output, and actual output.
pub struct EvalSample {
    pub input: alloc::string::String,
    pub expected: alloc::string::String,
    pub output: alloc::string::String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

impl EvalSample {
    pub fn new(input: &str, expected: &str, output: &str) -> Self {
        Self {
            input: alloc::string::String::from(input),
            expected: alloc::string::String::from(expected),
            output: alloc::string::String::from(output),
            prompt_tokens: 0,
            completion_tokens: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_harness() {
        let harness = EvalHarness::new(EvalConfig::default());
        let samples = vec![
            EvalSample::new("q1", "yes", "yes"),
            EvalSample::new("q2", "no", "yes"),
            EvalSample::new("q3", "maybe", "maybe"),
        ];
        let metrics = harness.run(&samples);
        assert_eq!(metrics.total_samples, 3);
        assert_eq!(metrics.correct, 2);
    }
}
