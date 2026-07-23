use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::global_quotient_theorems::GlobalQuotientTheorems;
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum V2Distance {
    Finite(u64),
    Infinite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotientWitnessRecord {
    pub word: Vec<u64>,
    pub word_length: usize,
    pub first_gap: u64,
    pub d_u: BigUint,
    pub q_u: BigUint,
    pub q_exponent: u64,
    pub n_u: BigInt,
    pub d_plus: BigUint,
    pub min_margin: i64,
    pub forbidden_distances: Vec<(u64, V2Distance, i64)>,
}

#[derive(Debug, Clone)]
pub struct OneZeroQuotientAtlas {
    pub max_depth: usize,
    pub max_gap: u64,
}

impl OneZeroQuotientAtlas {
    pub fn new(max_depth: usize, max_gap: u64) -> Self {
        Self { max_depth, max_gap }
    }

    /// Mine reachable canonical words and extract exact quotient records for one-zero witnesses
    pub fn mine_quotient_records(&self) -> Vec<QuotientWitnessRecord> {
        let mut records = Vec::new();

        // Level 1 words
        let mut current_words = Vec::new();
        for j in 0..=self.max_gap {
            let w = ExtremalSourceSearchEngine::base_guarded_word(j);
            current_words.push(w);
        }

        // Search reachable words up to max_depth
        let mut all_words = current_words.clone();
        for _depth in 2..=self.max_depth {
            let mut next_words = Vec::new();
            for w in &current_words {
                for h in 0..=self.max_gap {
                    let w_next = ExtremalSourceSearchEngine::extend_guarded_word(w, h);
                    next_words.push(w_next.clone());
                    all_words.push(w_next);
                }
            }
            current_words = next_words;
        }

        for w in &all_words {
            // Check if endpoint D_u lands in any branch cylinder [C_j]_{B_j}
            for j in 0..=self.max_gap {
                let p_j = AcceleratedBranchParams::for_gap(j);
                if (&w.endpoint % &p_j.modulus) == p_j.z_source_residue {
                    // One-zero witness found!
                    let d_u_big = BigInt::from(w.endpoint.clone());
                    let c_j_big = BigInt::from(p_j.z_source_residue.clone());
                    let m_j_big = BigInt::from(p_j.modulus.clone());

                    let n_u = (&d_u_big - &c_j_big) / &m_j_big;
                    let d_plus = &p_j.z_endpoint + &p_j.multiplier * n_u.to_biguint().unwrap_or_default();

                    // Calculate q_exponent = 6|u| + 3 * sum(j_i)
                    let sum_j: u64 = w.gap_sequence.iter().sum();
                    let q_exponent = 6 * (w.gap_sequence.len() as u64) + 3 * sum_j;

                    // Evaluate forbidden profile against a_{j,k} for k <= max_gap
                    let mut forbidden_distances = Vec::new();
                    let mut min_margin = i64::MAX;

                    for k in 0..=self.max_gap {
                        let p_k = AcceleratedBranchParams::for_gap(k);
                        let a_jk = GlobalQuotientTheorems::forbidden_quotient_residue(j, k);
                        let diff_n = &n_u - BigInt::from(a_jk);

                        let (v2_dist, margin) = match GlobalQuotientTheorems::v2_val(&diff_n) {
                            Some(v2) => {
                                let m = (p_k.precision as i64) - (v2 as i64);
                                (V2Distance::Finite(v2), m)
                            }
                            None => (V2Distance::Infinite, i64::MIN),
                        };

                        if margin < min_margin {
                            min_margin = margin;
                        }
                        forbidden_distances.push((k, v2_dist, margin));
                    }

                    records.push(QuotientWitnessRecord {
                        word: w.gap_sequence.clone(),
                        word_length: w.gap_sequence.len(),
                        first_gap: j,
                        d_u: w.endpoint.clone(),
                        q_u: w.affine.multiplier.clone(),
                        q_exponent,
                        n_u,
                        d_plus,
                        min_margin,
                        forbidden_distances,
                    });
                }
            }
        }

        records
    }

    /// Verify synthetic controls: n = a_{j,k}, n = a_{j,k} + 2^{B_k}, n = a_{j,k} + 2^t
    pub fn evaluate_synthetic_controls(j: u64, k: u64) -> bool {
        let p_j = AcceleratedBranchParams::for_gap(j);
        let p_k = AcceleratedBranchParams::for_gap(k);

        let a_jk = GlobalQuotientTheorems::forbidden_quotient_residue(j, k);

        // Control 1: n = a_{j,k} => D = C_j + M_j * a_{j,k}
        let _d_c1 = &p_j.z_source_residue + &p_j.modulus * &a_jk;
        let d_plus_c1 = &p_j.z_endpoint + &p_j.multiplier * &a_jk;
        assert_eq!(&d_plus_c1 % &p_k.modulus, p_k.z_source_residue, "Synthetic Control 1 must land in [C_k]");

        // Control 2: n = a_{j,k} + 2^{B_k}
        let n_c2 = BigInt::from(a_jk.clone()) + BigInt::from(p_k.modulus.clone());
        let diff_c2 = &n_c2 - BigInt::from(a_jk.clone());
        assert_eq!(GlobalQuotientTheorems::v2_val(&diff_c2), Some(p_k.precision));

        // Control 3: near miss n = a_{j,k} + 2^t for t < B_k
        for t in 0..p_k.precision {
            let n_c3 = BigInt::from(a_jk.clone()) + (BigInt::from(1u64) << t);
            let diff_c3 = &n_c3 - BigInt::from(a_jk.clone());
            assert_eq!(GlobalQuotientTheorems::v2_val(&diff_c3), Some(t));
        }

        true
    }
}
