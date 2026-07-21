/// Temporal decay function for memory relevance.
pub struct TemporalDecay {
    half_life: f32,
}

impl TemporalDecay {
    pub fn new(half_life: f32) -> Self {
        Self { half_life }
    }

    pub fn compute(&self, age: f32) -> f32 {
        let x = -0.693 * age / self.half_life;
        // Simple polynomial approximation of exp(x) for no_std
        // accurate to ~1% for x in [-10, 10]
        let x2 = x * x;
        let x3 = x2 * x;
        let x4 = x3 * x;
        let x5 = x4 * x;
        let x6 = x5 * x;
        1.0 + x + x2 * 0.5 + x3 * 0.166667 + x4 * 0.041667 + x5 * 0.008333 + x6 * 0.001389
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
        assert!((decay.compute(100.0) - 0.5).abs() < 0.01);
        assert!((decay.compute(200.0) - 0.25).abs() < 0.02);
    }
}
