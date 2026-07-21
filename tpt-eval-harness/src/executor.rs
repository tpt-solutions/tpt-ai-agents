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
}

impl Default for ParallelExecutor {
    fn default() -> Self {
        Self::new(8)
    }
}
