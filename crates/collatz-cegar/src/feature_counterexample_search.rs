#[allow(unused_imports)]
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeatureRelationKind {
    StrictDecrease,
    NonIncrease,
    ConstantReset {
        bound: String,
    },
    AffineBound {
        multiplier: String,
        offset: String,
    },
    Incomparable,
    Refuted {
        witness: String,
        minimality_status: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureSearchResultJson {
    pub edge_id: String,
    pub src_feature_id: String,
    pub dst_feature_id: String,
    pub relation: FeatureRelationKind,
    pub smallest_counterexample_found: Option<String>,
}

pub struct FeatureCounterexampleSearchEngine;

impl FeatureCounterexampleSearchEngine {
    /// Generates fixed-point linear form L_p(n) = |(2^A_p - 3^k_p)n - c_p| for a return path p
    pub fn derive_path_linear_form(total_a: u32, odd_steps_k: u32, affine_c: u128) -> (u128, i128) {
        let two_a = 1u128 << total_a;
        let pow3_k = 3u128.pow(odd_steps_k);

        if two_a > pow3_k {
            let alpha = two_a - pow3_k;
            let beta = -(affine_c as i128);
            (alpha, beta)
        } else {
            let alpha = pow3_k - two_a;
            let beta = affine_c as i128;
            (alpha, beta)
        }
    }

    /// Evaluates feature valuation v_2(L_1(F(n))) vs v_2(L_2(n)) across candidate source domain
    pub fn search_relation(
        edge_id: &str,
        src_feat_id: &str,
        dst_feat_id: &str,
        sample_domain: &[u64],
        eval_fn: impl Fn(u64) -> (u32, u32),
    ) -> FeatureSearchResultJson {
        let mut strict = true;
        let mut weak = true;
        let mut smallest_ce: Option<u64> = None;

        for &n in sample_domain {
            let (v_src, v_dst) = eval_fn(n);
            if v_dst >= v_src {
                strict = false;
            }
            if v_dst > v_src {
                weak = false;
                if smallest_ce.is_none() {
                    smallest_ce = Some(n);
                }
            }
        }

        let rel_kind = if strict {
            FeatureRelationKind::StrictDecrease
        } else if weak {
            FeatureRelationKind::NonIncrease
        } else {
            FeatureRelationKind::Refuted {
                witness: smallest_ce.unwrap_or(0).to_string(),
                minimality_status: "smallest_found".to_string(),
            }
        };

        FeatureSearchResultJson {
            edge_id: edge_id.to_string(),
            src_feature_id: src_feat_id.to_string(),
            dst_feature_id: dst_feat_id.to_string(),
            relation: rel_kind,
            smallest_counterexample_found: smallest_ce.map(|v| v.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_linear_form_derivation() {
        // [1, 1, 2] -> A=4, k=3, c=19 => 2^4 - 3^3 = 16 - 27 = -11 => alpha=11, beta=19
        let (a1, b1) = FeatureCounterexampleSearchEngine::derive_path_linear_form(4, 3, 19);
        assert_eq!((a1, b1), (11, 19));

        // [1, 2, 2] -> A=5, k=3, c=23 => 2^5 - 3^3 = 32 - 27 = 5 => alpha=5, beta=-23
        let (a2, b2) = FeatureCounterexampleSearchEngine::derive_path_linear_form(5, 3, 23);
        assert_eq!((a2, b2), (5, -23));
    }
}
