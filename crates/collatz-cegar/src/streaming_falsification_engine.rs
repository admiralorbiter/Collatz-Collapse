use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::coupled_invariant_miner::CoupledInvariantMiner;
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::spine_quotient_oracle::{ForbiddenShellResult, GlobalGuardResult, SpineQuotientOracle};
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone)]
pub struct ExactLevelReport {
    pub depth: usize,
    pub exact_word_count: usize,
    pub cum_word_count: usize,
    pub no_first_zero_count: usize,
    pub one_zero_count: usize,
    pub cum_one_zero_count: usize,
    pub double_zero_count: usize,
    pub cum_double_zero_count: usize,
    pub one_zero_witness_data: Vec<(BigInt, Vec<u64>)>,
}

#[derive(Debug, Clone)]
pub struct StreamingFalsificationReport {
    pub max_depth: usize,
    pub max_prefix_gap: u64,
    pub total_words_processed: usize,
    pub total_one_zero_count: usize,
    pub total_double_zero_count: usize,
    pub level_reports: Vec<ExactLevelReport>,
}

pub struct StreamingFalsificationEngine;

impl StreamingFalsificationEngine {
    /// Execute streaming falsification search with exact-depth level tracking
    pub fn run_streaming_falsification(
        max_depth: usize,
        max_prefix_gap: u64,
    ) -> StreamingFalsificationReport {
        let mut level_reports = Vec::new();
        let mut total_words_processed = 0;
        let mut total_one_zero_count = 0;
        let mut total_double_zero_count = 0;

        // Level 1 base words
        let mut current_level: Vec<(BigInt, BigUint, Vec<u64>)> = Vec::new();
        for j in 0..=max_prefix_gap {
            let w = ExtremalSourceSearchEngine::base_guarded_word(j);
            current_level.push((BigInt::from(w.endpoint.clone()), w.affine.multiplier.clone(), vec![j]));
        }

        let mut all_levels = vec![current_level.clone()];

        // Expand levels up to max_depth
        for _depth in 2..=max_depth {
            let mut next_level = Vec::new();
            for (d_u, q_u, word) in all_levels.last().unwrap() {
                let d_u_biguint = d_u.to_biguint().unwrap_or_default();
                for h in 0..=max_prefix_gap {
                    let (d_uh, q_uh) = CoupledInvariantMiner::canonical_extension(&d_u_biguint, q_u, h);
                    let mut next_word = word.clone();
                    next_word.push(h);
                    next_level.push((BigInt::from(d_uh), q_uh, next_word));
                }
            }
            all_levels.push(next_level);
        }

        // Process level by level to compute exact-depth vs cumulative statistics
        for (idx, level) in all_levels.iter().enumerate() {
            let depth = idx + 1;
            let exact_word_count = level.len();
            total_words_processed += exact_word_count;

            let mut no_first_zero_count = 0;
            let mut level_one_zero_count = 0;
            let mut level_double_zero_count = 0;
            let mut one_zero_witness_data = Vec::new();

            for (d_u, _q_u, word) in level {
                match SpineQuotientOracle::classify_global_zero_guard(d_u) {
                    GlobalGuardResult::NoFirstZero => {
                        no_first_zero_count += 1;
                    }
                    GlobalGuardResult::FirstZeroGuardFound { gap_j, source_residue, .. } => {
                        level_one_zero_count += 1;
                        total_one_zero_count += 1;
                        one_zero_witness_data.push((d_u.clone(), word.clone()));

                        // Compute n_u = (D_u - C_j) / M_j
                        let p_j = AcceleratedBranchParams::for_gap(gap_j);
                        let m_j_big = BigInt::from(p_j.modulus.clone());
                        let c_j_big = BigInt::from(source_residue);

                        let n_u = (d_u - &c_j_big) / &m_j_big;

                        // Classify quotient globally
                        let _res = SpineQuotientOracle::classify_global_forbidden_quotient(&n_u, gap_j);
                        if let ForbiddenShellResult::ForbiddenMatch { .. } = _res {
                            level_double_zero_count += 1;
                            total_double_zero_count += 1;
                        }
                    }
                }
            }

            level_reports.push(ExactLevelReport {
                depth,
                exact_word_count,
                cum_word_count: total_words_processed,
                no_first_zero_count,
                one_zero_count: level_one_zero_count,
                cum_one_zero_count: total_one_zero_count,
                double_zero_count: level_double_zero_count,
                cum_double_zero_count: total_double_zero_count,
                one_zero_witness_data,
            });
        }

        StreamingFalsificationReport {
            max_depth,
            max_prefix_gap,
            total_words_processed,
            total_one_zero_count,
            total_double_zero_count,
            level_reports,
        }
    }
}
