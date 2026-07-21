/// Context window manager for RAG pipelines.
pub struct ContextWindow {
    max_tokens: usize,
    used_tokens: usize,
}

impl ContextWindow {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            used_tokens: 0,
        }
    }

    pub fn can_fit(&self, tokens: usize) -> bool {
        self.used_tokens + tokens <= self.max_tokens
    }

    pub fn allocate(&mut self, tokens: usize) -> bool {
        if self.can_fit(tokens) {
            self.used_tokens += tokens;
            true
        } else {
            false
        }
    }

    pub fn remaining(&self) -> usize {
        self.max_tokens.saturating_sub(self.used_tokens)
    }

    pub fn reset(&mut self) {
        self.used_tokens = 0;
    }
}

impl Default for ContextWindow {
    fn default() -> Self {
        Self::new(4096)
    }
}
