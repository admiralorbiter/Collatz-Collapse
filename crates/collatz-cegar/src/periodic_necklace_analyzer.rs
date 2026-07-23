use collatz_affine::{MacrostepData, SymbolicWordData, ValuationWord, Q1_EXPONENT, Q1_RESIDUE};
use num_bigint::{BigInt, BigUint, Sign};
use std::collections::HashSet;

/// Periodic 2-adic orbit data for a primitive necklace root w and its specific rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeriodicOrbitData {
    pub primitive_root: ValuationWord,
    pub current_rotation: ValuationWord,
    pub period_length: usize,
    pub a_w: BigUint,
    pub total_valuation: u64,
    pub eta_w: BigUint,
    pub fixed_point_num: BigInt,
    pub fixed_point_den: BigInt,
    pub fixed_point_rational_str: String,
}

/// Primitive necklace representation containing all cyclic rotations (phases).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveNecklace {
    pub canonical_root: ValuationWord,
    pub period_length: usize,
    pub rotations: Vec<PeriodicOrbitData>,
}

/// Periodic necklace analyzer.
pub struct PeriodicNecklaceAnalyzer;

impl PeriodicNecklaceAnalyzer {
    /// Computes unique periodic orbit data for a word.
    pub fn analyze_word(word: ValuationWord) -> Result<PeriodicOrbitData, String> {
        let (root, _rep) = word.primitive_root();
        let m = MacrostepData::from_word(word.clone()).map_err(|e| e.to_string())?;

        let a = m.multiplier().clone();
        let total_val = m.total_valuation();
        let c = m.constant().clone();
        let b = BigUint::from(1u32) << total_val;

        // eta_w = (7 * a + c - 7 * b) / 32
        let term_a = BigUint::from(Q1_RESIDUE) * &a;
        let term_b = BigUint::from(Q1_RESIDUE) * &b;
        let num_b = &term_a + &c;
        if num_b < term_b {
            return Err("Overflow in eta computation".to_string());
        }
        let eta = (&num_b - &term_b) >> Q1_EXPONENT;

        // k^*_w = eta_w / (2^{A_w} - a_w) < 0 since 2^{A_w} < a_w
        let num_bigint = BigInt::from_biguint(Sign::Plus, eta.clone());
        let den_biguint = BigInt::from_biguint(Sign::Plus, b) - BigInt::from_biguint(Sign::Plus, a.clone());

        let rational_str = format!("{}/{}", num_bigint, den_biguint);

        Ok(PeriodicOrbitData {
            primitive_root: root,
            current_rotation: word.clone(),
            period_length: word.elements().len() / 3,
            a_w: a,
            total_valuation: total_val,
            eta_w: eta,
            fixed_point_num: num_bigint,
            fixed_point_den: den_biguint,
            fixed_point_rational_str: rational_str,
        })
    }

    /// Group words into true primitive necklaces with all their cyclic phases.
    pub fn extract_primitive_necklaces(
        words: &[SymbolicWordData],
    ) -> Result<Vec<PrimitiveNecklace>, String> {
        let mut seen_canonical = HashSet::new();
        let mut necklaces = Vec::new();

        for data in words {
            let (root, _rep) = data.word.primitive_root();
            let canonical = canonical_necklace_rotation(&root);

            if seen_canonical.insert(canonical.elements()) {
                // Generate all rotations of canonical root
                let rotations_words = all_cyclic_rotations(&canonical)?;
                let mut rot_data = Vec::new();
                for rot_w in rotations_words {
                    rot_data.push(Self::analyze_word(rot_w)?);
                }

                necklaces.push(PrimitiveNecklace {
                    canonical_root: canonical.clone(),
                    period_length: canonical.elements().len() / 3,
                    rotations: rot_data,
                });
            }
        }

        Ok(necklaces)
    }
}

/// Returns lexicographically minimal rotation of a primitive root (canonical necklace).
fn canonical_necklace_rotation(word: &ValuationWord) -> ValuationWord {
    let elems = word.elements();
    let n = elems.len();
    if n == 0 {
        return word.clone();
    }
    // Block size is 3 for u ([1,1,2]) and 6 for v ([1,1,2,1,2,2])
    // Shift by 3-element blocks
    let mut min_elems = elems.clone();
    for shift in (0..n).step_by(3) {
        let mut rotated = elems[shift..].to_vec();
        rotated.extend_from_slice(&elems[..shift]);
        if rotated < min_elems {
            min_elems = rotated;
        }
    }
    ValuationWord::from_u32_slice(&min_elems).unwrap()
}

/// Returns all unique cyclic rotations of a primitive root word.
fn all_cyclic_rotations(word: &ValuationWord) -> Result<Vec<ValuationWord>, String> {
    let elems = word.elements();
    let n = elems.len();
    let mut seen = HashSet::new();
    let mut rots = Vec::new();

    for shift in (0..n).step_by(3) {
        let mut rotated = elems[shift..].to_vec();
        rotated.extend_from_slice(&elems[..shift]);
        if seen.insert(rotated.clone()) {
            rots.push(ValuationWord::from_u32_slice(&rotated).map_err(|e| e.to_string())?);
        }
    }
    Ok(rots)
}

/// Valuation histogram entry for depth r.
#[derive(Debug, Clone, PartialEq)]
pub struct ValuationHistogramEntry {
    pub depth_r: usize,
    pub v_count_j: usize,
    pub word_count_combinations: u64,
    pub valuation_a: u64,
    pub cylinder_haar_measure: f64,
}

/// Topological entropy and dual Haar measure engine.
pub struct TopologicalEntropyEngine;

impl TopologicalEntropyEngine {
    /// Computes valuation histogram entries for depth r.
    pub fn compute_histogram(r: usize) -> Vec<ValuationHistogramEntry> {
        let mut entries = Vec::new();
        for j in 0..=r {
            let combinations = n_choose_k(r, j);
            let val_a = 4 * (r - j) as u64 + 9 * j as u64;
            let measure = (combinations as f64) * (2.0f64).powi(-(val_a as i32));

            entries.push(ValuationHistogramEntry {
                depth_r: r,
                v_count_j: j,
                word_count_combinations: combinations,
                valuation_a: val_a,
                cylinder_haar_measure: measure,
            });
        }
        entries
    }

    /// Computes dual Haar measures for depth r.
    /// mu_k(G_r) = (33 / 512)^r
    /// mu_n(G_r) = 2^{-5} * (33 / 512)^r
    pub fn compute_dual_haar_measures(r: usize) -> (f64, f64) {
        let base = 33.0f64 / 512.0f64;
        let mu_k = base.powi(r as i32);
        let mu_n = (2.0f64.powi(-5)) * mu_k;
        (mu_k, mu_n)
    }
}

fn n_choose_k(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let mut res = 1u64;
    for i in 1..=k {
        res = res * (n - i + 1) as u64 / i as u64;
    }
    res
}
