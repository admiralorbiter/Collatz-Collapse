use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::two_zero_cylinder_characterization::{DoubleZeroStatus, TwoZeroCylinderCharacterization};
use std::collections::HashSet;

/// Grid Cell Aggregated Results Statistics.
#[derive(Debug, Clone, Default)]
pub struct GridCellStats {
    pub prefix_word_count: usize,
    pub distinct_endpoint_count: usize,
    pub endpoint_collision_count: usize,
    pub no_zero_lift_count: usize,
    pub exactly_one_zero_lift_count: usize,
    pub at_least_two_zero_lift_count: usize,
}

/// Phase 7.3S.2C: Single-Pass Streaming Grid Traversal Engine.
pub struct SinglePassGridTraversal {
    pub max_depth: usize,
    pub max_gap: u64,
}

impl SinglePassGridTraversal {
    pub fn new(max_depth: usize, max_gap: u64) -> Self {
        Self { max_depth, max_gap }
    }

    /// Execute single streaming pass over all canonical prefixes up to max_depth and max_gap
    pub fn run_traversal(&self) -> (GridCellStats, Vec<(Vec<u64>, DoubleZeroStatus)>) {
        let char_engine = TwoZeroCylinderCharacterization::new(self.max_gap);
        let mut stats = GridCellStats::default();
        let mut witnesses = Vec::new();
        let mut seen_endpoints = HashSet::new();

        let mut current_words = Vec::new();
        for j in 0..=self.max_gap {
            let word = ExtremalSourceSearchEngine::base_guarded_word(j);
            current_words.push((vec![j], word));
        }

        for _d in 1..=self.max_depth {
            let mut next_words = Vec::new();
            for (seq, word) in current_words {
                stats.prefix_word_count += 1;
                let d_u = word.endpoint.clone();

                if seen_endpoints.contains(&d_u) {
                    stats.endpoint_collision_count += 1;
                } else {
                    seen_endpoints.insert(d_u.clone());
                    stats.distinct_endpoint_count += 1;
                }

                let status = char_engine.evaluate_endpoint_status(&d_u);
                match status {
                    DoubleZeroStatus::NoZeroLift => stats.no_zero_lift_count += 1,
                    DoubleZeroStatus::ExactlyOneZeroLift { .. } => stats.exactly_one_zero_lift_count += 1,
                    DoubleZeroStatus::AtLeastTwoZeroLifts { .. } => {
                        stats.at_least_two_zero_lift_count += 1;
                        witnesses.push((seq.clone(), status));
                    }
                }

                if seq.len() < self.max_depth {
                    for j in 0..=self.max_gap {
                        let child_word = ExtremalSourceSearchEngine::extend_guarded_word(&word, j);
                        let mut child_seq = seq.clone();
                        child_seq.push(j);
                        next_words.push((child_seq, child_word));
                    }
                }
            }
            current_words = next_words;
        }

        (stats, witnesses)
    }
}
