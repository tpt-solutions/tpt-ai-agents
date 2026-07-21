/// Temporal decay function for memory relevance.
pub struct TemporalDecay {
    half_life: f32,
}

impl TemporalDecay {
    pub fn new(half_life: f32) -> Self {
        Self { half_life }
    }

    pub fn compute(&self, age: f32) -> f32 {
        (-0.693 * age / self.half_life).exp()
    }
}

impl Default for TemporalDecay {
    fn default() -> Self {
        Self::new(86400.0) // 1 day in seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay() {
        let decay = TemporalDecay::new(100.0);
        assert!((decay.compute(0.0) - 1.0).abs() < 0.001);
        assert!((decay.compute(100.0) - 0.5).abs() < 0.001);
        assert!((decay.compute(200.0) - 0.25).abs() < 0.001);
    }
}
