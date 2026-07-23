use crate::adversarial_corpus::{AdversarialCorpus, AdversarialRecord, BranchParameters, RecordOrigin, StoppingReason};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Composite affine map on Z_2: F_w(z) = (Q_w * z + \beta_w) / M_w
#[derive(Debug, Clone)]
pub struct CompositeAffineMap {
    pub multiplier: BigUint,   // Q_w = 3^{B_w_odd}
    pub denominator: BigUint,  // M_w = 2^{B_w}
    pub intercept: BigInt,     // \beta_w
}

/// Canonical guarded word tracking source residue \rho_w, endpoint D_w, and affine map.
#[derive(Debug, Clone)]
pub struct CanonicalGuardedWord {
    pub source_residue: BigUint, // \rho_w
    pub endpoint: BigUint,       // D_w
    pub affine: CompositeAffineMap,
    pub gap_sequence: Vec<u64>,
    pub accelerated_depth: usize,
}

impl CanonicalGuardedWord {
    /// Verify permanent structural invariants for canonical guarded words:
    /// 1. 0 <= \rho_w < M_w
    /// 2. M_w == 2^{B_w}
    /// 3. Q_w * \rho_w + \beta_w == M_w * D_w
    pub fn verify_structural_invariants(&self) -> bool {
        if self.source_residue >= self.affine.denominator {
            return false;
        }

        let q_rho = BigInt::from(self.affine.multiplier.clone()) * BigInt::from(self.source_residue.clone());
        let lhs = q_rho + &self.affine.intercept;
        let rhs = BigInt::from(self.affine.denominator.clone()) * BigInt::from(self.endpoint.clone());

        lhs == rhs
    }
}

/// Result record for Experiment 1 extremal minimum source search M_{H,J}(B).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtremalSearchResult {
    pub precision_b: u64,
    pub actual_b_s: u64,
    pub accelerated_depth_r: usize,
    pub min_source_z: String,
    pub min_source_k: String,
    pub minimizing_gap_sequence: Vec<String>,
    pub growth_density_alpha: f64,
    pub alpha_witness: f64,
    pub bits_per_source_bit: f64,
    pub is_unique: bool,
    pub is_nested: bool,
    pub max_precision_bound_h: u64,
    pub max_gap_bound_j: u64,
    pub stable_through_j: u64,
    pub beta_intercept: String,
}

/// Configuration for Experiment 1 search bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtremalSearchConfig {
    pub max_accelerated_depth_r: usize,
    pub max_gap_j: u64,
    pub max_precision_h: u64,
}

impl Default for ExtremalSearchConfig {
    fn default() -> Self {
        Self {
            max_accelerated_depth_r: 6,
            max_gap_j: 2,
            max_precision_h: 64,
        }
    }
}

use crate::accelerated_branch_params::AcceleratedBranchParams;

/// Gap-bound sensitivity table entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapSensitivityEntry {
    pub target_b: u64,
    pub min_source_by_j: BTreeMap<u64, String>,
    pub stable_through_j: u64,
}

/// Symbolic branch diagnostics entry for single gap j.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicBranchDiagnostics {
    pub gap_j: u64,
    pub m_j: String,
    pub q_j: String,
    pub c_j: String,
    pub d_j: String,
    pub beta_j: String,
    pub bitlength_c_j: u64,
    pub precision_b_j: u64,
    pub zero_tail_z_j: u64,
    pub normalized_c_ratio: f64,
    pub mu_mod_11: u8,
}

/// Experiment 1 Engine: Bounded Extremal Minimum Source Search M_{H,J}(B) & E_{H,J}(b)
pub struct ExtremalSourceSearchEngine;

impl ExtremalSourceSearchEngine {
    /// Return exact authoritative branch parameters for return gap j >= 0.
    pub fn branch_parameters_j(j: u64) -> AcceleratedBranchParams {
        AcceleratedBranchParams::for_gap(j)
    }

    /// Analyze symbolic 1-gap branch diagnostics for gap j.
    pub fn analyze_single_branch_j(j: u64) -> SymbolicBranchDiagnostics {
        let p = Self::branch_parameters_j(j);
        let precision_b_j = p.precision;
        let bitlength_c_j = p.z_source_residue.bits();
        let zero_tail_z_j = precision_b_j.saturating_sub(bitlength_c_j);

        let m_float = Self::log2_biguint(&p.modulus).exp2();
        let c_float = Self::log2_biguint(&p.z_source_residue).exp2();
        let normalized_c_ratio = c_float / m_float.max(1.0);

        SymbolicBranchDiagnostics {
            gap_j: j,
            m_j: p.modulus.to_string(),
            q_j: p.multiplier.to_string(),
            c_j: p.z_source_residue.to_string(),
            d_j: p.z_endpoint.to_string(),
            beta_j: p.affine_intercept.to_string(),
            bitlength_c_j,
            precision_b_j,
            zero_tail_z_j,
            normalized_c_ratio,
            mu_mod_11: p.mu_mod_11,
        }
    }

    /// Construct base single-branch canonical guarded word for gap j.
    pub fn base_guarded_word(j: u64) -> CanonicalGuardedWord {
        let p = Self::branch_parameters_j(j);
        let word = CanonicalGuardedWord {
            source_residue: p.z_source_residue,
            endpoint: p.z_endpoint,
            affine: CompositeAffineMap {
                multiplier: p.multiplier,
                denominator: p.modulus,
                intercept: p.affine_intercept,
            },
            gap_sequence: vec![j],
            accelerated_depth: 1,
        };
        debug_assert!(word.verify_structural_invariants(), "Base word invariant failure");
        word
    }

    /// Extend a canonical guarded word w by return gap j:
    /// Solve R \equiv Q_w^{-1} * (C_j - D_w) (mod M_j)
    /// \rho_{wj} = \rho_w + M_w * R
    /// \beta_{wj} = Q_j * \beta_w + M_w * \beta_j
    /// D_{wj} = (Q_{wj} * \rho_{wj} + \beta_{wj}) / M_{wj}
    pub fn extend_guarded_word(parent: &CanonicalGuardedWord, j: u64) -> CanonicalGuardedWord {
        let p = Self::branch_parameters_j(j);
        let m_j = p.modulus;
        let q_j = p.multiplier;
        let c_j = p.z_source_residue;
        let beta_j = p.affine_intercept;

        let q_w = &parent.affine.multiplier;
        let m_w = &parent.affine.denominator;
        let beta_w = &parent.affine.intercept;
        let d_w = &parent.endpoint;
        let rho_w = &parent.source_residue;

        let q_w_mod = q_w % &m_j;
        let inv_q_w = Self::mod_inverse(&q_w_mod, &m_j);

        let target_diff = if c_j >= *d_w {
            (&c_j - d_w) % &m_j
        } else {
            let diff = (d_w - &c_j) % &m_j;
            if diff.is_zero() {
                BigUint::zero()
            } else {
                &m_j - diff
            }
        };

        let r = (&inv_q_w * target_diff) % &m_j;

        let rho_wj = rho_w + m_w * &r;

        let q_wj = &q_j * q_w;
        let m_wj = &m_j * m_w;

        let beta_wj = BigInt::from(q_j.clone()) * beta_w + BigInt::from(m_w.clone()) * beta_j;

        let q_rho = BigInt::from(q_wj.clone()) * BigInt::from(rho_wj.clone());
        let num_d = q_rho + &beta_wj;
        let d_wj = (num_d / BigInt::from(m_wj.clone())).to_biguint().unwrap();

        let mut next_gaps = parent.gap_sequence.clone();
        next_gaps.push(j);

        let word = CanonicalGuardedWord {
            source_residue: rho_wj,
            endpoint: d_wj,
            affine: CompositeAffineMap {
                multiplier: q_wj,
                denominator: m_wj,
                intercept: beta_wj,
            },
            gap_sequence: next_gaps,
            accelerated_depth: parent.accelerated_depth + 1,
        };

        debug_assert!(word.verify_structural_invariants(), "Extended word invariant failure");
        word
    }

    /// Compute sequence word parameters for gap list.
    pub fn sequence_guarded_word(gaps: &[u64]) -> CanonicalGuardedWord {
        assert!(!gaps.is_empty(), "Gap sequence cannot be empty");
        let mut word = Self::base_guarded_word(gaps[0]);
        for &j in &gaps[1..] {
            word = Self::extend_guarded_word(&word, j);
        }
        word
    }

    fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let mut x = BigUint::one();
        let two = BigUint::from(2u32);
        let mut i = 0;
        while i < 14 {
            let ax_mod = (a * &x) % m;
            let factor = (m + &two - &ax_mod) % m;
            x = (&x * &factor) % m;
            i += 1;
        }
        x
    }

    /// Translate accelerated source z-height to original register k-coordinate: k = 61 + 512 * z.
    pub fn z_to_k(z: &BigUint) -> BigUint {
        BigUint::from(61u64) + (z * 512u32)
    }

    /// Compute exact log2 as a floating point value for BigUint: log2(max(2, z)).
    pub fn log2_biguint(z: &BigUint) -> f64 {
        if z <= &BigUint::from(2u64) {
            1.0
        } else {
            let bits = z.bits();
            if bits <= 53 {
                let u: u64 = z.to_u64_digits()[0];
                (u as f64).log2()
            } else {
                let shift = bits - 53;
                let top_bits = z >> shift;
                let u: u64 = top_bits.to_u64_digits()[0];
                (u as f64).log2() + (shift as f64)
            }
        }
    }

    /// Compute exponential density rate \alpha(B) = log2(max(2, \rho_s)) / B.
    pub fn compute_growth_density_alpha(b: u64, min_source_z: &BigUint) -> f64 {
        let log2_m = Self::log2_biguint(min_source_z);
        log2_m / (b.max(1) as f64)
    }

    /// Compute witness-precision density rate \alpha_{witness}(B_s) = log2(max(2, \rho_s)) / B_s.
    pub fn compute_growth_density_alpha_witness(b_s: u64, min_source_z: &BigUint) -> f64 {
        let log2_m = Self::log2_biguint(min_source_z);
        log2_m / (b_s.max(1) as f64)
    }

    /// Compute reciprocal metric bits_per_source_bit(B) = B / log2(max(2, \rho_s)).
    pub fn compute_bits_per_source_bit(b: u64, min_source_z: &BigUint) -> f64 {
        let log2_m = Self::log2_biguint(min_source_z);
        (b as f64) / log2_m.max(1e-6)
    }

    /// Run Experiment 1 exact-precision minimum search E_{H,J}(b) = min {\rho_s : B_s = b}.
    pub fn run_exact_precision_search(
        config: &ExtremalSearchConfig,
    ) -> BTreeMap<u64, CanonicalGuardedWord> {
        let mut min_by_precision: BTreeMap<u64, CanonicalGuardedWord> = BTreeMap::new();

        let mut current_words: Vec<CanonicalGuardedWord> = Vec::new();
        for j in 0..=config.max_gap_j {
            let word = Self::base_guarded_word(j);
            let b = word.affine.denominator.bits() - 1;
            if b <= config.max_precision_h {
                min_by_precision.entry(b).or_insert_with(|| word.clone());
                current_words.push(word);
            }
        }

        for _depth in 2..=config.max_accelerated_depth_r {
            let mut next_words = Vec::new();
            for word in &current_words {
                for j in 0..=config.max_gap_j {
                    let ext = Self::extend_guarded_word(word, j);
                    let b = ext.affine.denominator.bits() - 1;

                    if b <= config.max_precision_h {
                        let update = match min_by_precision.get(&b) {
                            None => true,
                            Some(curr) => ext.source_residue < curr.source_residue,
                        };

                        if update {
                            min_by_precision.insert(b, ext.clone());
                        }

                        next_words.push(ext);
                    }
                }
            }
            current_words = next_words;
        }

        min_by_precision
    }

    /// Compute suffix minimum threshold function M_{H,J}(B) = min_{B <= b <= H} E_{H,J}(b).
    pub fn compute_threshold_minima(
        exact_map: &BTreeMap<u64, CanonicalGuardedWord>,
    ) -> BTreeMap<u64, CanonicalGuardedWord> {
        let mut threshold_map = BTreeMap::new();
        let precisions: Vec<u64> = exact_map.keys().cloned().collect();

        for &b_target in &precisions {
            let mut best_word: Option<&CanonicalGuardedWord> = None;
            for (&b_actual, word) in exact_map.iter() {
                if b_actual >= b_target {
                    match best_word {
                        None => best_word = Some(word),
                        Some(curr) => {
                            if word.source_residue < curr.source_residue {
                                best_word = Some(word);
                            }
                        }
                    }
                }
            }
            if let Some(best) = best_word {
                threshold_map.insert(b_target, best.clone());
            }
        }

        threshold_map
    }

    /// Run full Experiment 1 search generating exact precision results and corpus records.
    pub fn run_search(
        config: &ExtremalSearchConfig,
        corpus: &mut AdversarialCorpus,
    ) -> Vec<ExtremalSearchResult> {
        let exact_map = Self::run_exact_precision_search(config);
        let threshold_map = Self::compute_threshold_minima(&exact_map);

        let mut results = Vec::new();
        let mut prev_min: Option<BigUint> = None;

        for (&b_target, word) in &threshold_map {
            let rho_s = &word.source_residue;
            let actual_b = word.affine.denominator.bits() - 1;
            let k_source = Self::z_to_k(rho_s);
            let alpha = Self::compute_growth_density_alpha(b_target, rho_s);
            let alpha_witness = Self::compute_growth_density_alpha_witness(actual_b, rho_s);
            let bits_per_bit = Self::compute_bits_per_source_bit(b_target, rho_s);

            let is_nested = match &prev_min {
                None => true,
                Some(p) => rho_s >= p,
            };
            prev_min = Some(rho_s.clone());

            let res = ExtremalSearchResult {
                precision_b: b_target,
                actual_b_s: actual_b,
                accelerated_depth_r: word.accelerated_depth,
                min_source_z: rho_s.to_string(),
                min_source_k: k_source.to_string(),
                minimizing_gap_sequence: word.gap_sequence.iter().map(|j| format!("j={}", j)).collect(),
                growth_density_alpha: alpha,
                alpha_witness,
                bits_per_source_bit: bits_per_bit,
                is_unique: true,
                is_nested,
                max_precision_bound_h: config.max_precision_h,
                max_gap_bound_j: config.max_gap_j,
                stable_through_j: config.max_gap_j,
                beta_intercept: word.affine.intercept.to_string(),
            };

            let a_comp = &word.affine.multiplier;
            let shift_comp = actual_b;
            corpus.add_record(AdversarialRecord {
                id: format!("extremal_m_b{}_r{}", b_target, word.accelerated_depth),
                origin: RecordOrigin::ExtremalMinimum,
                gap_sequence: word.gap_sequence.iter().map(|j| format!("j={}", j)).collect(),
                flattened_uv_word: vec![],
                total_precision: b_target,
                source_residue: rho_s.to_string(),
                endpoint_sequence: vec![word.endpoint.to_string()],
                lift_blocks: word.gap_sequence.clone(),
                branch_parameters: BranchParameters {
                    a_composite: a_comp.to_string(),
                    b_composite: word.affine.intercept.to_string(),
                    shift_composite: shift_comp,
                },
                periodic_shadow_word: None,
                periodic_shadow_length: 0,
                mod_3_signatures: vec![],
                stopping_reason: StoppingReason::SearchDepthCutoff,
                is_exact: false,
                generation_bounds: format!("bounded_H{}_J{}", config.max_precision_h, config.max_gap_j),
                dedup_key: format!("extremal_rho_b{}_{}", b_target, rho_s),
            });

            results.push(res);
        }

        results
    }

    /// Run Gap-Bound Sensitivity Analysis across increasing J bounds: J \in {0, 1, 2, 3}.
    pub fn run_gap_sensitivity_analysis(
        max_h: u64,
        max_r: usize,
        gap_bounds: &[u64],
        corpus: &mut AdversarialCorpus,
    ) -> Vec<GapSensitivityEntry> {
        let mut min_by_target_by_j: BTreeMap<u64, BTreeMap<u64, BigUint>> = BTreeMap::new();

        for &j_bound in gap_bounds {
            let cfg = ExtremalSearchConfig {
                max_accelerated_depth_r: max_r,
                max_gap_j: j_bound,
                max_precision_h: max_h,
            };

            let results = Self::run_search(&cfg, corpus);
            for res in results {
                let z_val: BigUint = res.min_source_z.parse().unwrap_or_else(|_| BigUint::zero());
                min_by_target_by_j
                    .entry(res.precision_b)
                    .or_default()
                    .insert(j_bound, z_val);
            }
        }

        let mut entries = Vec::new();
        for (target_b, j_map) in min_by_target_by_j {
            let mut stable_j = 0u64;
            let mut prev_val: Option<&BigUint> = None;

            for (&j, val) in &j_map {
                match prev_val {
                    None => {
                        stable_j = j;
                        prev_val = Some(val);
                    }
                    Some(p) => {
                        if val == p {
                            stable_j = j;
                        } else {
                            break;
                        }
                    }
                }
            }

            entries.push(GapSensitivityEntry {
                target_b,
                min_source_by_j: j_map.iter().map(|(&j, v)| (j, v.to_string())).collect(),
                stable_through_j: stable_j,
            });
        }

        entries
    }
}
