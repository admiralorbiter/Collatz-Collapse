use crate::shell_carry_engine::ShellCarryEngine;
use crate::spine_quotient_oracle::{ForbiddenShellResult, SpineQuotientOracle};
use crate::streaming_falsification_engine::StreamingFalsificationEngine;
use num_bigint::BigInt;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RejectionLayer {
    SuccessorOdd,
    EvenSuccessorSourceValuationSafe,
    SpineValuationSafe,
    ShellSignatureMismatch,
    DoubleZeroMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalAuditReport {
    pub max_depth: usize,
    pub max_prefix_gap: u64,
    pub word_count: usize,
    pub one_zero_count: usize,
    pub double_zero_count: usize,
    pub conditional_expected_hits: f64,
    pub rejection_layer_distribution: HashMap<RejectionLayer, usize>,
    pub first_gap_distribution: HashMap<u64, usize>,
}

pub struct ConditionalAuditEngine;

impl ConditionalAuditEngine {
    /// Classify one-zero witness into 5-layer ordered mutually exclusive rejection hierarchy
    pub fn classify_rejection_layer(
        successor: &BigInt,
        quotient_n: &BigInt,
        first_gap_j: u64,
    ) -> RejectionLayer {
        // Layer 1: Parity Rejection (Successor Odd)
        if (successor % BigInt::from(2u64)) != BigInt::zero() {
            return RejectionLayer::SuccessorOdd;
        }

        // Layer 2: Even Successor Source Valuation Safe (v_2(L(D^+)) not in {1, 5, 6})
        let l_succ = ShellCarryEngine::l_val(successor);
        let t_succ = SpineQuotientOracle::v2_val(&l_succ).unwrap_or(0);
        if t_succ != 1 && t_succ != 5 && t_succ != 6 {
            return RejectionLayer::EvenSuccessorSourceValuationSafe;
        }

        // Layer 3: Spine Valuation Safe (t_succ not 1 mod 4)
        if (t_succ % 4) != 1 {
            return RejectionLayer::SpineValuationSafe;
        }

        // Layer 4 / 5: Evaluate Quotient Shell Oracle
        match SpineQuotientOracle::classify_global_forbidden_quotient(quotient_n, first_gap_j) {
            ForbiddenShellResult::ForbiddenMatch { .. } => RejectionLayer::DoubleZeroMatch,
            _ => RejectionLayer::ShellSignatureMismatch,
        }
    }

    /// Execute conditional audit and rejection layer analysis
    pub fn run_conditional_audit(max_depth: usize, max_prefix_gap: u64) -> ConditionalAuditReport {
        let report = StreamingFalsificationEngine::run_streaming_falsification(max_depth, max_prefix_gap);

        let mut rejection_layer_distribution = HashMap::new();
        let mut first_gap_distribution = HashMap::new();
        let mut double_zero_count = 0;

        for lvl in &report.level_reports {
            for (endpoint_big, _word) in &lvl.one_zero_witness_data {
                if let crate::spine_quotient_oracle::GlobalGuardResult::FirstZeroGuardFound { gap_j, .. } =
                    SpineQuotientOracle::classify_global_zero_guard(endpoint_big)
                {
                    *first_gap_distribution.entry(gap_j).or_insert(0) += 1;

                    let coords = ShellCarryEngine::compute_shell_coordinates(endpoint_big, gap_j).unwrap();
                    let layer = Self::classify_rejection_layer(&coords.successor, &coords.quotient_n, gap_j);

                    if layer == RejectionLayer::DoubleZeroMatch {
                        double_zero_count += 1;
                    }

                    *rejection_layer_distribution.entry(layer).or_insert(0) += 1;
                }
            }
        }

        let conditional_expected_hits = (report.total_one_zero_count as f64) / 480.0;

        ConditionalAuditReport {
            max_depth,
            max_prefix_gap,
            word_count: report.total_words_processed,
            one_zero_count: report.total_one_zero_count,
            double_zero_count,
            conditional_expected_hits,
            rejection_layer_distribution,
            first_gap_distribution,
        }
    }
}
