//! Shared token-usage tracking across requests.

use core::fmt;
use std::sync::Mutex;

/// Accumulates token usage across multiple LLM requests.
///
/// Thread-safe via internal `Mutex`. Useful for tracking total cost across
/// a session, multi-turn agent loop, or evaluation run without each
/// consumer re-implementing accounting.
///
/// # Example
///
/// ```
/// use tpt_llm_client_core::usage::UsageTracker;
/// use tpt_llm_client_core::Usage;
///
/// let tracker = UsageTracker::new();
/// tracker.record(&Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 });
/// tracker.record(&Usage { prompt_tokens: 5, completion_tokens: 10, total_tokens: 15 });
/// let totals = tracker.snapshot();
/// assert_eq!(totals.prompt_tokens, 15);
/// assert_eq!(totals.completion_tokens, 30);
/// assert_eq!(totals.total_tokens, 45);
/// ```
pub struct UsageTracker {
    inner: Mutex<UsageTotals>,
}

/// Accumulated usage totals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UsageTotals {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    /// Number of requests recorded.
    pub request_count: u64,
}

impl UsageTracker {
    /// Create a new, empty tracker.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(UsageTotals::default()),
        }
    }

    /// Record usage from a single response. Safe to call from any thread.
    pub fn record(&self, usage: &crate::response::Usage) {
        let mut totals = self.inner.lock().expect("usage tracker lock poisoned");
        totals.prompt_tokens += usage.prompt_tokens as u64;
        totals.completion_tokens += usage.completion_tokens as u64;
        totals.total_tokens += usage.total_tokens as u64;
        totals.request_count += 1;
    }

    /// Take a snapshot of the current accumulated totals.
    pub fn snapshot(&self) -> UsageTotals {
        *self.inner.lock().expect("usage tracker lock poisoned")
    }

    /// Reset all counters to zero.
    pub fn reset(&self) {
        let mut totals = self.inner.lock().expect("usage tracker lock poisoned");
        *totals = UsageTotals::default();
    }
}

impl Default for UsageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for UsageTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let totals = self.snapshot();
        f.debug_struct("UsageTracker")
            .field("totals", &totals)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::Usage;

    #[test]
    fn test_tracker_accumulates() {
        let tracker = UsageTracker::new();
        tracker.record(&Usage {
            prompt_tokens: 10,
            completion_tokens: 20,
            total_tokens: 30,
        });
        tracker.record(&Usage {
            prompt_tokens: 5,
            completion_tokens: 10,
            total_tokens: 15,
        });
        let totals = tracker.snapshot();
        assert_eq!(totals.prompt_tokens, 15);
        assert_eq!(totals.completion_tokens, 30);
        assert_eq!(totals.total_tokens, 45);
        assert_eq!(totals.request_count, 2);
    }

    #[test]
    fn test_tracker_reset() {
        let tracker = UsageTracker::new();
        tracker.record(&Usage {
            prompt_tokens: 100,
            completion_tokens: 200,
            total_tokens: 300,
        });
        tracker.reset();
        let totals = tracker.snapshot();
        assert_eq!(totals.total_tokens, 0);
        assert_eq!(totals.request_count, 0);
    }

    #[test]
    fn test_tracker_default_is_empty() {
        let tracker = UsageTracker::new();
        let totals = tracker.snapshot();
        assert_eq!(totals, UsageTotals::default());
    }

    #[test]
    fn test_tracker_debug() {
        let tracker = UsageTracker::new();
        let debug_str = alloc::format!("{:?}", tracker);
        assert!(debug_str.contains("UsageTracker"));
    }
}
