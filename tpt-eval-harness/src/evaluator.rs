use crate::{EvalConfig, EvalMetrics, Error};

/// Main evaluation harness.
pub struct EvalHarness {
    config: EvalConfig,
}

impl EvalHarness {
    pub fn new(config: EvalConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, dataset_path: &str) -> Result<EvalMetrics, Error> {
        let _ = dataset_path;
        Ok(EvalMetrics::new())
    }

    pub fn config(&self) -> &EvalConfig {
        &self.config
    }
}
