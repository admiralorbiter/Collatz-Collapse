use crate::extremal_source_search::{CanonicalGuardedWord, ExtremalSourceSearchEngine};
use crate::zero_tail_stress_audit::ZeroTailProfile;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Outcome status for large-gap stress scan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanStatusLabel {
    NoCheapPrecisionObservedThroughJ32,
    CheapPrecisionTrendObserved,
    BoundaryMaximumRequiresLargerJ,
    LargeGapScanInconclusive,
}

impl std::fmt::Display for ScanStatusLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCheapPrecisionObservedThroughJ32 => {
                write!(f, "NO_CHEAP_PRECISION_OBSERVED_THROUGH_J32")
            }
            Self::CheapPrecisionTrendObserved => write!(f, "CHEAP_PRECISION_TREND_OBSERVED"),
            Self::BoundaryMaximumRequiresLargerJ => {
                write!(f, "BOUNDARY_MAXIMUM_REQUIRES_LARGER_J")
            }
            Self::LargeGapScanInconclusive => write!(f, "LARGE_GAP_SCAN_INCONCLUSIVE"),
        }
    }
}

/// Execution mode tag distinguishing exhaustive search vs beam search heuristic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionModeTag {
    ExhaustiveWithinDeclaredDepthAndGap,
    BeamSearchHeuristic,
}

/// Summary report for large-gap stress scan up to J_stress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeGapScanReport {
    pub max_gap_j_scanned: u64,
    pub zeta_max_1_gap: f64,
    pub zeta_max_2_gap: f64,
    pub best_1_gap_sequence: Vec<u64>,
    pub best_2_gap_sequence: Vec<u64>,
    pub status_label: ScanStatusLabel,
    pub execution_mode: ExecutionModeTag,
    pub best_1_gap_profile: ZeroTailProfile,
    pub best_2_gap_profile: ZeroTailProfile,
}

/// Adaptive Stress Engine: Exhaustive 1/2-gap scans and adaptive multi-gap beam search.
pub struct AdaptiveStressEngine;

impl AdaptiveStressEngine {
    /// Run exhaustive large-gap scan for 1-gap and 2-gap words up to j_max (e.g. j_max = 32).
    pub fn run_large_gap_scan(j_max: u64) -> LargeGapScanReport {
        let mut best_1_word = ExtremalSourceSearchEngine::base_guarded_word(0);
        let mut best_1_profile = ZeroTailProfile::from_canonical_word(&best_1_word);

        for j in 0..=j_max {
            let word = ExtremalSourceSearchEngine::base_guarded_word(j);
            let profile = ZeroTailProfile::from_canonical_word(&word);
            if profile.normalized_zero_tail > best_1_profile.normalized_zero_tail {
                best_1_profile = profile;
                best_1_word = word;
            }
        }

        let mut best_2_word = ExtremalSourceSearchEngine::extend_guarded_word(
            &ExtremalSourceSearchEngine::base_guarded_word(0),
            0,
        );
        let mut best_2_profile = ZeroTailProfile::from_canonical_word(&best_2_word);

        for i in 0..=j_max {
            let base_i = ExtremalSourceSearchEngine::base_guarded_word(i);
            for j in 0..=j_max {
                let word = ExtremalSourceSearchEngine::extend_guarded_word(&base_i, j);
                let profile = ZeroTailProfile::from_canonical_word(&word);
                if profile.normalized_zero_tail > best_2_profile.normalized_zero_tail {
                    best_2_profile = profile;
                    best_2_word = word;
                }
            }
        }

        let zeta_max_1_gap = best_1_profile.normalized_zero_tail;
        let zeta_max_2_gap = best_2_profile.normalized_zero_tail;

        let status_label = if zeta_max_1_gap < 0.5 && zeta_max_2_gap < 0.5 {
            ScanStatusLabel::NoCheapPrecisionObservedThroughJ32
        } else if zeta_max_2_gap > 0.8 {
            ScanStatusLabel::CheapPrecisionTrendObserved
        } else {
            ScanStatusLabel::LargeGapScanInconclusive
        };

        LargeGapScanReport {
            max_gap_j_scanned: j_max,
            zeta_max_1_gap,
            zeta_max_2_gap,
            best_1_gap_sequence: best_1_word.gap_sequence,
            best_2_gap_sequence: best_2_word.gap_sequence,
            status_label,
            execution_mode: ExecutionModeTag::ExhaustiveWithinDeclaredDepthAndGap,
            best_1_gap_profile: best_1_profile,
            best_2_gap_profile: best_2_profile,
        }
    }

    /// Run adaptive 3-to-5-gap stress search using beam search (K = beam_width).
    /// Performs both right extensions (w + j) and left extensions (j + w).
    pub fn run_adaptive_beam_search(
        j_max: u64,
        beam_width: usize,
        max_depth: usize,
    ) -> Vec<(CanonicalGuardedWord, ZeroTailProfile)> {
        let mut beam: Vec<(CanonicalGuardedWord, ZeroTailProfile)> = Vec::new();

        // Initialize beam with 1-gap and 2-gap words
        for i in 0..=j_max {
            let w1 = ExtremalSourceSearchEngine::base_guarded_word(i);
            let p1 = ZeroTailProfile::from_canonical_word(&w1);
            beam.push((w1.clone(), p1));

            for j in 0..=j_max {
                let w2 = ExtremalSourceSearchEngine::extend_guarded_word(&w1, j);
                let p2 = ZeroTailProfile::from_canonical_word(&w2);
                beam.push((w2, p2));
            }
        }

        // Sort by normalized_zero_tail descending
        Self::prune_beam(&mut beam, beam_width);

        // Extend through max_depth
        for current_depth in 3..=max_depth {
            let mut candidates = Vec::new();

            for (parent_word, _) in &beam {
                if parent_word.accelerated_depth + 1 == current_depth {
                    // Right extensions
                    for j in 0..=j_max.min(8) {
                        let ext_right = ExtremalSourceSearchEngine::extend_guarded_word(parent_word, j);
                        let p_right = ZeroTailProfile::from_canonical_word(&ext_right);
                        candidates.push((ext_right, p_right));
                    }

                    // Left extensions (prefixing gap j)
                    for j in 0..=j_max.min(8) {
                        let mut new_gaps = vec![j];
                        new_gaps.extend_from_slice(&parent_word.gap_sequence);
                        let ext_left = ExtremalSourceSearchEngine::sequence_guarded_word(&new_gaps);
                        let p_left = ZeroTailProfile::from_canonical_word(&ext_left);
                        candidates.push((ext_left, p_left));
                    }
                }
            }

            beam.extend(candidates);
            Self::prune_beam(&mut beam, beam_width);
        }

        beam
    }

    fn prune_beam(beam: &mut Vec<(CanonicalGuardedWord, ZeroTailProfile)>, width: usize) {
        beam.sort_by(|a, b| {
            b.1.normalized_zero_tail
                .partial_cmp(&a.1.normalized_zero_tail)
                .unwrap_or(Ordering::Equal)
        });
        beam.dedup_by(|a, b| a.0.gap_sequence == b.0.gap_sequence);
        if beam.len() > width {
            beam.truncate(width);
        }
    }
}
