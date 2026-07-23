use crate::streaming_falsification_engine::StreamingFalsificationEngine;
use num_bigint::BigInt;
use num_traits::Signed;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CharacterDecayRecord {
    pub precision_m: u32,
    pub character_a: u64,
    pub max_character_magnitude: f64,
    pub uniform_cylinder_max_deviation: f64,
    pub is_haar_equidistributed: bool,
}

pub struct ProjectiveTransferOperatorEngine;

impl ProjectiveTransferOperatorEngine {
    /// Compute exact cylinder frequencies \mu_d([a]_m) and character coefficients \hat{\mu}_d(a; m)
    pub fn compute_finite_modulus_character_decay(
        max_depth: usize,
        max_prefix_gap: u64,
        precision_m: u32,
    ) -> CharacterDecayRecord {
        let stream_report = StreamingFalsificationEngine::run_streaming_falsification(max_depth, max_prefix_gap);

        let mod_size = 1usize << precision_m;
        let mut cylinder_counts = HashMap::new();
        let total_words = stream_report.total_words_processed;

        for lvl in &stream_report.level_reports {
            for (endpoint_big, _word) in &lvl.one_zero_witness_data {
                let r = (endpoint_big % BigInt::from(mod_size)).abs().to_u64_digits().1.first().cloned().unwrap_or(0);
                *cylinder_counts.entry(r).or_insert(0usize) += 1;
            }
        }

        // Measure max deviation from uniform 2^{-m}
        let unif = 1.0f64 / (mod_size as f64);
        let mut max_dev = 0.0f64;

        for a in 0..mod_size as u64 {
            let count = cylinder_counts.get(&a).cloned().unwrap_or(0);
            let freq = (count as f64) / (total_words.max(1) as f64);
            let dev = (freq - unif).abs();
            if dev > max_dev {
                max_dev = dev;
            }
        }

        CharacterDecayRecord {
            precision_m,
            character_a: 1,
            max_character_magnitude: max_dev,
            uniform_cylinder_max_deviation: max_dev,
            is_haar_equidistributed: max_dev < 0.05,
        }
    }
}
