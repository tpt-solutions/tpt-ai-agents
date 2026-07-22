#[cfg(feature = "std")]
use crate::Error;
use crate::{
    scoring::{ExactMatchScorer, Scorer},
    EvalConfig, EvalMetrics,
};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

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

    /// Load a dataset from a JSONL file (one JSON-encoded [`EvalSample`] per
    /// line, blank lines ignored) and evaluate it.
    #[cfg(feature = "std")]
    pub fn run_file(&self, path: &str) -> crate::Result<EvalMetrics> {
        let dataset = EvalSample::load_jsonl(path)?;
        Ok(self.run(&dataset))
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSample {
    pub input: alloc::string::String,
    pub expected: alloc::string::String,
    pub output: alloc::string::String,
    #[serde(default)]
    pub prompt_tokens: u64,
    #[serde(default)]
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

    /// Load a dataset from a JSONL file: one JSON-encoded `EvalSample` object
    /// per line, with blank lines ignored.
    #[cfg(feature = "std")]
    pub fn load_jsonl(path: &str) -> crate::Result<alloc::vec::Vec<Self>> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| Error::Dataset(alloc::format!("failed to read '{path}': {e}")))?;

        let mut samples = alloc::vec::Vec::new();
        for (line_no, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let sample: Self = serde_json::from_str(line).map_err(|e| {
                Error::Dataset(alloc::format!("invalid JSON on line {}: {e}", line_no + 1))
            })?;
            samples.push(sample);
        }
        Ok(samples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

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

    #[test]
    #[cfg(feature = "std")]
    fn test_load_jsonl_and_run_file() {
        let path = std::env::temp_dir().join(alloc::format!(
            "tpt_eval_harness_test_{}.jsonl",
            std::process::id()
        ));
        std::fs::write(
            &path,
            concat!(
                "{\"input\":\"q1\",\"expected\":\"yes\",\"output\":\"yes\"}\n",
                "\n",
                "{\"input\":\"q2\",\"expected\":\"no\",\"output\":\"yes\",\"prompt_tokens\":5,\"completion_tokens\":2}\n",
            ),
        )
        .unwrap();

        let samples = EvalSample::load_jsonl(path.to_str().unwrap()).unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[1].prompt_tokens, 5);

        let harness = EvalHarness::new(EvalConfig::default());
        let metrics = harness.run_file(path.to_str().unwrap()).unwrap();
        assert_eq!(metrics.total_samples, 2);
        assert_eq!(metrics.correct, 1);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_load_jsonl_missing_file_errors() {
        assert!(EvalSample::load_jsonl("this_file_does_not_exist.jsonl").is_err());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_load_jsonl_invalid_json_errors() {
        let path = std::env::temp_dir().join(alloc::format!(
            "tpt_eval_harness_bad_test_{}.jsonl",
            std::process::id()
        ));
        std::fs::write(&path, "not json\n").unwrap();
        assert!(EvalSample::load_jsonl(path.to_str().unwrap()).is_err());
        std::fs::remove_file(&path).ok();
    }
}
