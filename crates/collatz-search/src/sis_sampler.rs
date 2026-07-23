use collatz_affine::ValuationWord;

/// Sampled valuation path from Sequential Importance Sampling.
#[derive(Debug, Clone)]
pub struct ImportanceSample {
    pub word: ValuationWord,
    pub log_likelihood_weight: f64,
    pub normalized_weight: f64,
}

/// Sequential Importance Sampler (SIS) with Exponential Tilting theta*.
/// Tilts child valuation sampling probabilities to artificially center E_theta*[a_i] = log_2(3),
/// sampling rare-event growth-resistant paths without mode collapse.
pub struct SequentialImportanceSampler {
    pub target_length: usize,
    pub num_samples: usize,
    pub theta_star: f64,
}

impl Default for SequentialImportanceSampler {
    fn default() -> Self {
        Self {
            target_length: 10,
            num_samples: 100,
            // Exponential tilting theta* chosen to shift geometric(1/2) mean 2.0 down to log2(3) approx 1.585
            theta_star: 0.287,
        }
    }
}

impl SequentialImportanceSampler {
    pub fn new(target_length: usize, num_samples: usize) -> Self {
        Self {
            target_length,
            num_samples,
            ..Default::default()
        }
    }

    /// Computes un-tilted geometric probability P(a_i = v) = 2^{-v}.
    pub fn untilted_prob(v: u32) -> f64 {
        0.5f64.powi(v as i32)
    }

    /// Computes tilted probability P_theta*(a_i = v) proportional to e^{-theta* v} * 2^{-v}.
    pub fn tilted_prob(&self, v: u32) -> f64 {
        let raw = (-self.theta_star * (v as f64)).exp() * Self::untilted_prob(v);
        // Normalize over v in 1..=10
        let norm: f64 = (1..=10u32)
            .map(|x| (-self.theta_star * (x as f64)).exp() * Self::untilted_prob(x))
            .sum();
        raw / norm
    }

    /// Generates a set of importance-weighted valuation samples.
    pub fn sample(&self) -> Vec<ImportanceSample> {
        let mut samples = Vec::with_capacity(self.num_samples);

        // Precompute cumulative probabilities for tilted sampling
        let mut cdf = Vec::with_capacity(10);
        let mut cum = 0.0f64;
        for v in 1..=10u32 {
            cum += self.tilted_prob(v);
            cdf.push((v, cum));
        }

        // Use simple deterministic pseudo-random sequence for reproducible sampling
        let mut rng_seed = 0x123456789abcdef0u64;

        for _s in 0..self.num_samples {
            let mut word_vec = Vec::with_capacity(self.target_length);
            let mut log_weight = 0.0f64;

            for _step in 0..self.target_length {
                // Xorshift64 random float in [0, 1)
                rng_seed ^= rng_seed << 13;
                rng_seed ^= rng_seed >> 7;
                rng_seed ^= rng_seed << 17;
                let u = (rng_seed as f64) / (u64::MAX as f64);

                let selected_val = cdf
                    .iter()
                    .find(|&&(_, p)| u <= p)
                    .map(|&(v, _)| v)
                    .unwrap_or(1);
                word_vec.push(selected_val as u8);

                let p_untilted = Self::untilted_prob(selected_val);
                let p_tilted = self.tilted_prob(selected_val);

                log_weight += (p_untilted / p_tilted).ln();
            }

            if let Ok(word) = ValuationWord::new(word_vec) {
                samples.push(ImportanceSample {
                    word,
                    log_likelihood_weight: log_weight,
                    normalized_weight: log_weight.exp(),
                });
            }
        }

        // Normalize weights across sample set
        let total_w: f64 = samples.iter().map(|s| s.normalized_weight).sum();
        if total_w > 0.0 {
            for s in &mut samples {
                s.normalized_weight /= total_w;
            }
        }

        samples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sis_sampler() {
        let sampler = SequentialImportanceSampler::new(5, 20);
        let samples = sampler.sample();

        assert_eq!(samples.len(), 20);
        let norm_sum: f64 = samples.iter().map(|s| s.normalized_weight).sum();
        assert!((norm_sum - 1.0).abs() < 1e-5);
    }
}
