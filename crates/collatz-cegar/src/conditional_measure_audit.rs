use crate::shell_carry_engine::ShellCarryEngine;
use crate::spine_quotient_oracle::SpineQuotientOracle;
use crate::streaming_falsification_engine::StreamingFalsificationEngine;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExactDepthAuditRecord {
    pub depth: usize,
    pub exact_word_count: usize,
    pub cum_word_count: usize,
    pub one_zero_count: usize,
    pub cum_one_zero_count: usize,
    pub haar_expected_one_zero: f64,
    pub double_zero_count: usize,
    pub cum_double_zero_count: usize,
    pub conditional_expected_double_zero: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedTotalVariationRecord {
    pub precision_m: u32,
    pub sample_count: usize,
    pub modulus_size: usize,
    pub occupied_residue_count: usize,
    pub raw_total_variation: f64,
    pub finite_support_lower_bound: f64,
    pub is_sparse_diagnostic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalMeasureAuditReport {
    pub max_depth: usize,
    pub max_prefix_gap: u64,
    pub total_words_processed: usize,
    pub total_one_zero_count: usize,
    pub total_double_zero_count: usize,
    pub exact_depth_records: Vec<ExactDepthAuditRecord>,
    pub pooled_gap_distribution: HashMap<String, usize>,
    pub staged_tv_records: Vec<StagedTotalVariationRecord>,
    pub rejection_layer_counts: HashMap<String, usize>,
}

pub struct ConditionalMeasureAuditEngine;

impl ConditionalMeasureAuditEngine {
    /// Classify rejection layer using CORRECTED v2(D^+) for Layer 2
    pub fn classify_rejection_layer_corrected(
        successor: &BigInt,
        quotient_n: &BigInt,
        first_gap_j: u64,
    ) -> &'static str {
        // Layer 1: SUCCESSOR_ODD
        if (successor % BigInt::from(2u64)) != BigInt::zero() {
            return "SUCCESSOR_ODD";
        }

        // Layer 2: EVEN_SUCCESSOR_ENDPOINT_VALUATION_SAFE (v_2(D^+) NOT in {1, 5, 6})
        let v2_succ = SpineQuotientOracle::v2_val(successor).unwrap_or(0);
        if v2_succ != 1 && v2_succ != 5 && v2_succ != 6 {
            return "EVEN_SUCCESSOR_ENDPOINT_VALUATION_SAFE";
        }

        // Layer 3: SPINE_VALUATION_SAFE (t = v_2(L(D^+)) NOT 1 mod 4)
        let l_succ = ShellCarryEngine::l_val(successor);
        let t_succ = SpineQuotientOracle::v2_val(&l_succ).unwrap_or(0);
        if (t_succ % 4) != 1 {
            return "SPINE_VALUATION_SAFE";
        }

        // Layer 4 / 5: Evaluate Quotient Shell Oracle
        match SpineQuotientOracle::classify_global_forbidden_quotient(quotient_n, first_gap_j) {
            crate::spine_quotient_oracle::ForbiddenShellResult::ForbiddenMatch { .. } => "DOUBLE_ZERO_MATCH",
            _ => "SHELL_SIGNATURE_SAFE",
        }
    }

    /// Execute conditional measure and spectral audit
    pub fn run_conditional_measure_audit(
        max_depth: usize,
        max_prefix_gap: u64,
    ) -> ConditionalMeasureAuditReport {
        let report = StreamingFalsificationEngine::run_streaming_falsification(max_depth, max_prefix_gap);

        let mut exact_depth_records = Vec::new();
        let mut pooled_gap_distribution = HashMap::new();
        let mut rejection_layer_counts = HashMap::new();
        let mut all_w_records: Vec<BigInt> = Vec::new();

        for lvl in &report.level_reports {
            let haar_exp_one_zero = (lvl.exact_word_count as f64) / 480.0;
            let cond_exp_double_zero = (lvl.one_zero_count as f64) / 480.0;

            exact_depth_records.push(ExactDepthAuditRecord {
                depth: lvl.depth,
                exact_word_count: lvl.exact_word_count,
                cum_word_count: lvl.cum_word_count,
                one_zero_count: lvl.one_zero_count,
                cum_one_zero_count: lvl.cum_one_zero_count,
                haar_expected_one_zero: haar_exp_one_zero,
                double_zero_count: lvl.double_zero_count,
                cum_double_zero_count: lvl.cum_double_zero_count,
                conditional_expected_double_zero: cond_exp_double_zero,
            });

            for (endpoint_big, _word) in &lvl.one_zero_witness_data {
                if let crate::spine_quotient_oracle::GlobalGuardResult::FirstZeroGuardFound { gap_j, .. } =
                    SpineQuotientOracle::classify_global_zero_guard(endpoint_big)
                {
                    let bin = match gap_j {
                        0 => "j=0",
                        1 => "j=1",
                        2 => "j=2",
                        3 => "j=3",
                        _ => "j>=4",
                    };
                    *pooled_gap_distribution.entry(bin.to_string()).or_insert(0) += 1;

                    let coords = ShellCarryEngine::compute_shell_coordinates(endpoint_big, gap_j).unwrap();
                    let layer = Self::classify_rejection_layer_corrected(&coords.successor, &coords.quotient_n, gap_j);

                    *rejection_layer_counts.entry(layer.to_string()).or_insert(0) += 1;

                    // W = X - x_{j,\infty} = 2673 * n
                    all_w_records.push(coords.centered_carry_x.clone());
                }
            }
        }

        // Staged Total Variation Audit on W = X - x_{j,\infty} for m = 1..16
        let sample_count = all_w_records.len();
        let mut staged_tv_records = Vec::new();

        for m in 1..=16u32 {
            let mod_size = 1usize << m;
            let mut residue_counts = HashMap::new();

            for w in &all_w_records {
                let r = (w % BigInt::from(mod_size)).abs().to_u64_digits().1.first().cloned().unwrap_or(0);
                *residue_counts.entry(r).or_insert(0usize) += 1;
            }

            let occupied_count = residue_counts.len();
            let mut sum_abs_diff = 0.0f64;
            let unif = 1.0f64 / (mod_size as f64);

            for r in 0..mod_size as u64 {
                let count = residue_counts.get(&r).cloned().unwrap_or(0);
                let p_r = (count as f64) / (sample_count.max(1) as f64);
                sum_abs_diff += (p_r - unif).abs();
            }

            let raw_tv = 0.5f64 * sum_abs_diff;
            let lower_bound = (1.0f64 - (sample_count as f64) / (mod_size as f64)).max(0.0);
            let is_sparse = m > 9;

            staged_tv_records.push(StagedTotalVariationRecord {
                precision_m: m,
                sample_count,
                modulus_size: mod_size,
                occupied_residue_count: occupied_count,
                raw_total_variation: raw_tv,
                finite_support_lower_bound: lower_bound,
                is_sparse_diagnostic: is_sparse,
            });
        }

        ConditionalMeasureAuditReport {
            max_depth,
            max_prefix_gap,
            total_words_processed: report.total_words_processed,
            total_one_zero_count: report.total_one_zero_count,
            total_double_zero_count: report.total_double_zero_count,
            exact_depth_records,
            pooled_gap_distribution,
            staged_tv_records,
            rejection_layer_counts,
        }
    }
}
