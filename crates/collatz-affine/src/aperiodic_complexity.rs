use crate::{
    CanonicalCoreSelector, CoreTransitionReport, PrecisionLedger, SelectorOutput, TwoAdicValuation,
    ValuationWord,
};
use num_bigint::BigInt;
use std::collections::{HashMap, HashSet};

/// Recurrence classification for bounded-gap valuation paths (H.3A.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecurrenceType {
    Recurrent,
    UniformlyRecurrent,
    NonRecurrent,
}

/// Multiscale periodic-window coverage metrics C_x(N; p, r), U_x(N; p, r), G_x(N; p, r).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiscaleCoverageMetrics {
    pub total_length: usize,
    pub max_period_p: usize,
    pub min_reps_r: usize,
    pub covered_positions: usize,
    pub uncovered_positions: usize,
    pub max_uncovered_run_length: usize,
    pub coverage_ratio: f64,
    pub distinct_orbit_count: usize,
    pub phase_change_count: usize,
    pub genuine_orbit_switch_count: usize,
    pub resonant_switch_count: usize,
    pub total_bits_consumed: u64,
    pub total_reset_losses: u64,
    pub total_resonance_gain: u64,
    pub net_balance: i64,
}

/// Analyzer for factor complexity p_x(n), recurrence, and multiscale coverage.
pub struct FactorComplexityAnalyzer;

impl FactorComplexityAnalyzer {
    /// Computes factor complexity p_x(n) = number of distinct subwords of length n in word.
    pub fn factor_complexity(word: &ValuationWord, n: usize) -> usize {
        let slice = word.as_slice();
        if n == 0 || n > slice.len() {
            return 0;
        }
        let mut set = HashSet::new();
        for window in slice.windows(n) {
            set.insert(window.to_vec());
        }
        set.len()
    }

    /// Evaluates multiscale periodic-window coverage C_x(N; p, r), U_x(N; p, r), G_x(N; p, r).
    pub fn evaluate_multiscale_coverage(
        selector: &CanonicalCoreSelector,
        path: &ValuationWord,
        window_size: usize,
    ) -> MultiscaleCoverageMetrics {
        let slice = path.as_slice();
        let total_length = slice.len();
        if total_length < window_size {
            return MultiscaleCoverageMetrics {
                total_length,
                max_period_p: 32,
                min_reps_r: 2,
                covered_positions: 0,
                uncovered_positions: total_length,
                max_uncovered_run_length: total_length,
                coverage_ratio: 0.0,
                distinct_orbit_count: 0,
                phase_change_count: 0,
                genuine_orbit_switch_count: 0,
                resonant_switch_count: 0,
                total_bits_consumed: 0,
                total_reset_losses: 0,
                total_resonance_gain: 0,
                net_balance: 0,
            };
        }

        let mut covered_flags = vec![false; total_length];
        let mut prev_out = SelectorOutput::NoStructuredCore;

        let mut distinct_orbits = HashSet::new();
        let mut phase_change_count = 0;
        let mut genuine_orbit_switch_count = 0;
        let mut resonant_switch_count = 0;

        let mut ledger = PrecisionLedger::new(TwoAdicValuation::Finite(16));

        for i in window_size..=total_length {
            let window = ValuationWord::from_slice(&slice[i - window_size..i]);
            let curr_out = selector.select_core(&window);

            if let SelectorOutput::StructuredCore(ref curr_sel) = curr_out {
                distinct_orbits.insert(curr_sel.orbit_id);
                for pos in (i - window_size)..i {
                    covered_flags[pos] = true;
                }

                if let SelectorOutput::StructuredCore(ref prev_sel) = prev_out {
                    let transition = CanonicalCoreSelector::report_transition(&prev_out, &curr_out);
                    match transition {
                        CoreTransitionReport::AdvancedPhase { .. } => phase_change_count += 1,
                        CoreTransitionReport::SwitchedCore { .. } => {
                            genuine_orbit_switch_count += 1;
                            let a_v = BigInt::from(64u32);
                            let switch_res =
                                ledger.record_switch(&prev_sel.core, &curr_sel.core, &a_v, 1);
                            if switch_res.switch_type == crate::CoreSwitchType::Resonant {
                                resonant_switch_count += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }

            prev_out = curr_out;
        }

        let covered_positions = covered_flags.iter().filter(|&&b| b).count();
        let uncovered_positions = total_length - covered_positions;
        let coverage_ratio = (covered_positions as f64) / (total_length as f64);

        // Compute maximum uncovered run length G_x(N; p, r)
        let mut max_uncovered_run_length = 0;
        let mut curr_run = 0;
        for &c in &covered_flags {
            if !c {
                curr_run += 1;
                if curr_run > max_uncovered_run_length {
                    max_uncovered_run_length = curr_run;
                }
            } else {
                curr_run = 0;
            }
        }

        let total_bits_consumed = ledger.total_bits_consumed;
        let total_reset_losses = ledger.total_reset_losses;
        let total_resonance_gain = ledger.total_resonance_gain;
        let net_balance = (total_resonance_gain as i64) - ((total_bits_consumed + total_reset_losses) as i64);

        MultiscaleCoverageMetrics {
            total_length,
            max_period_p: 32,
            min_reps_r: 2,
            covered_positions,
            uncovered_positions,
            max_uncovered_run_length,
            coverage_ratio,
            distinct_orbit_count: distinct_orbits.len(),
            phase_change_count,
            genuine_orbit_switch_count,
            resonant_switch_count,
            total_bits_consumed,
            total_reset_losses,
            total_resonance_gain,
            net_balance,
        }
    }
}

/// Analyzer for Sturmian cube occurrences (H.3B).
pub struct SturmianCubeAnalyzer;

impl SturmianCubeAnalyzer {
    /// Detects cube occurrences u^3 = uuu in a word and measures max distance between cube ends.
    pub fn max_cube_gap_distance(word: &ValuationWord, max_pattern_len: usize) -> Option<usize> {
        let slice = word.as_slice();
        let len = slice.len();
        let mut cube_end_positions = Vec::new();

        for i in 0..len {
            for p in 1..=max_pattern_len {
                if i + 3 * p <= len {
                    let u = &slice[i..i + p];
                    let u2 = &slice[i + p..i + 2 * p];
                    let u3 = &slice[i + 2 * p..i + 3 * p];
                    if u == u2 && u == u3 {
                        cube_end_positions.push(i + 3 * p);
                    }
                }
            }
        }

        if cube_end_positions.len() <= 1 {
            return None;
        }

        cube_end_positions.sort();
        cube_end_positions.dedup();

        let mut max_gap = 0;
        for window in cube_end_positions.windows(2) {
            let gap = window[1] - window[0];
            if gap > max_gap {
                max_gap = gap;
            }
        }
        Some(max_gap)
    }
}

/// Verification engine for Lean-friendly potential function \Phi in substitutive graphs (H.3C).
pub struct SubstitutivePotentialFunction;

impl SubstitutivePotentialFunction {
    /// Verifies if a node potential assignment \Phi satisfies w(e) + \Phi(t) - \Phi(s) <= -\epsilon.
    pub fn verify_potential_certificate(
        _nodes: &[usize],
        edges: &[(usize, usize, i64)], // (source, target, net_weight)
        potential: &HashMap<usize, i64>,
        epsilon: i64,
    ) -> bool {
        for &(s, t, w) in edges {
            let phi_s = match potential.get(&s) {
                Some(&p) => p,
                None => return false,
            };
            let phi_t = match potential.get(&t) {
                Some(&p) => p,
                None => return false,
            };
            if w + phi_t - phi_s > -epsilon {
                return false;
            }
        }
        true
    }
}

/// Deterministic benchmark word generators for Phase H.3.
pub struct DeterministicBenchmarkGenerators;

impl DeterministicBenchmarkGenerators {
    /// Generates Fibonacci word (Sturmian, golden ratio slope) mapped to valuations {1, 2}.
    pub fn fibonacci_word(length: usize) -> ValuationWord {
        let mut s0 = vec![1u8];
        let mut s1 = vec![1u8, 2u8];

        while s1.len() < length {
            let mut s_next = s1.clone();
            s_next.extend_from_slice(&s0);
            s0 = s1;
            s1 = s_next;
        }

        ValuationWord::from_slice(&s1[0..length])
    }

    /// Generates Silver Ratio Sturmian word (slope \sqrt{2} - 1, substitution 0 -> 001, 1 -> 0).
    pub fn silver_ratio_word(length: usize) -> ValuationWord {
        let mut s0 = vec![1u8];
        let mut s1 = vec![1u8, 1u8, 2u8];

        while s1.len() < length {
            let mut s_next = s1.clone();
            s_next.extend_from_slice(&s1);
            s_next.extend_from_slice(&s0);
            s0 = s1;
            s1 = s_next;
        }

        ValuationWord::from_slice(&s1[0..length])
    }

    /// Generates Thue-Morse sequence (Automatic, p(n) = O(n)) mapped to valuations {1, 2}.
    pub fn thue_morse_word(length: usize) -> ValuationWord {
        let mut seq = vec![1u8];
        while seq.len() < length {
            let mut next = Vec::with_capacity(seq.len() * 2);
            for &val in &seq {
                if val == 1 {
                    next.push(1);
                    next.push(2);
                } else {
                    next.push(2);
                    next.push(1);
                }
            }
            seq = next;
        }
        ValuationWord::from_slice(&seq[0..length])
    }

    /// Generates Period-Doubling sequence mapped to valuations {1, 2}.
    pub fn period_doubling_word(length: usize) -> ValuationWord {
        let mut seq = vec![1u8];
        while seq.len() < length {
            let mut next = Vec::with_capacity(seq.len() * 2);
            for &val in &seq {
                if val == 1 {
                    next.push(1);
                    next.push(2);
                } else {
                    next.push(1);
                    next.push(1);
                }
            }
            seq = next;
        }
        ValuationWord::from_slice(&seq[0..length])
    }

    /// Generates Tribonacci sequence (3-letter substitution 0 -> 01, 1 -> 02, 2 -> 0).
    pub fn tribonacci_word(length: usize) -> ValuationWord {
        let mut seq = vec![1u8];
        while seq.len() < length {
            let mut next = Vec::new();
            for &val in &seq {
                match val {
                    1 => {
                        next.push(1);
                        next.push(2);
                    }
                    2 => {
                        next.push(1);
                        next.push(3);
                    }
                    3 => {
                        next.push(1);
                    }
                    _ => {}
                }
            }
            seq = next;
        }
        ValuationWord::from_slice(&seq[0..length])
    }

    /// Generates Adversarial Periodic sequence with a single defect to test tie-breaking.
    pub fn adversarial_periodic_defect_word(length: usize) -> ValuationWord {
        let mut seq = vec![1u8, 2u8, 1u8, 2u8];
        while seq.len() < length {
            seq.extend_from_slice(&[1u8, 2u8, 1u8, 2u8]);
        }
        if seq.len() > 16 {
            seq[16] = 5; // Insert sparse defect at position 16
        }
        ValuationWord::from_slice(&seq[0..length])
    }
}
