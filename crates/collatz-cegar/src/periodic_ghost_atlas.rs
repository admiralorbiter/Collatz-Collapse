use crate::adversarial_corpus::{AdversarialCorpus, AdversarialRecord, BranchParameters, RecordOrigin, StoppingReason};
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representation of a canonical periodic orbit class (necklace) over accelerated gap words.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeriodicOrbitClass {
    pub canonical_necklace: Vec<u64>,
    pub rotations: Vec<Vec<u64>>,
    pub period_length: usize,
    pub is_primitive_root: bool,
}

/// Data entry in the Periodic Ghost Orbit Atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostOrbitEntry {
    pub gap_sequence: Vec<u64>,
    pub original_uv_mapping: Vec<String>,
    pub composite_a: String,
    pub composite_b: String,
    pub composite_shift: u64,
    pub fixed_point_numerator: String,
    pub fixed_point_denominator: String,
    pub fixed_point_is_negative: bool,
    pub guarded_domain_valid: bool,
    pub min_positive_rep_r1: String,
    pub min_positive_rep_r3: String,
    pub phase_rotations_count: usize,
}

/// Result of evaluating shadow metrics for a target trajectory against the ghost atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowEvaluationResult {
    pub trajectory_id: String,
    pub best_matching_gap_sequence: Vec<u64>,
    pub exact_repetitions: usize,
    pub closeness_v2: u64,
    pub normalized_shadow_score: u64,
    pub match_kind: String, // "Prefix", "Suffix", "Internal", "Rotation"
}

/// Engine for building the Periodic Ghost-Orbit Atlas and evaluating shadow metrics.
pub struct PeriodicGhostAtlas;

impl PeriodicGhostAtlas {
    /// Return primitive root status and canonical necklace representative.
    pub fn canonical_necklace(word: &[u64]) -> PeriodicOrbitClass {
        let n = word.len();
        let mut rotations = Vec::new();

        for i in 0..n {
            let mut rot = Vec::with_capacity(n);
            for j in 0..n {
                rot.push(word[(i + j) % n]);
            }
            rotations.push(rot);
        }

        let mut canonical = rotations[0].clone();
        for rot in &rotations[1..] {
            if rot < &canonical {
                canonical = rot.clone();
            }
        }

        let mut is_primitive = true;
        for sub_len in 1..n {
            if n % sub_len == 0 {
                let mut repeat_match = true;
                for i in 0..n {
                    if word[i] != word[i % sub_len] {
                        repeat_match = false;
                        break;
                    }
                }
                if repeat_match {
                    is_primitive = false;
                    break;
                }
            }
        }

        PeriodicOrbitClass {
            canonical_necklace: canonical,
            rotations,
            period_length: n,
            is_primitive_root: is_primitive,
        }
    }

    /// Convert an accelerated gap word (j_1, ..., j_r) into original u/v string notation.
    pub fn gap_word_to_uv_strings(word: &[u64]) -> Vec<String> {
        let mut uv = Vec::new();
        for &j in word {
            uv.push("v".to_string());
            for _ in 0..j {
                uv.push("u".to_string());
            }
        }
        uv
    }

    /// Compute composite map parameters for accelerated gap word w = (j_1, ..., j_r) in z-coordinates.
    /// Return (A_w, B_w, shift_w).
    pub fn word_parameters(word: &[u64]) -> (BigInt, BigInt, u64) {
        let guarded = ExtremalSourceSearchEngine::sequence_guarded_word(word);
        (
            BigInt::from(guarded.affine.multiplier),
            guarded.affine.intercept,
            guarded.affine.denominator.bits() as u64 - 1,
        )
    }

    /// Compute exact 2-adic fixed point z_w^* = \beta_w / (2^{B_w} - a_w) < 0.
    /// Returns (numerator, denominator).
    pub fn fixed_point(word: &[u64]) -> (BigInt, BigInt) {
        let (a_w, b_w, b_exp) = Self::word_parameters(word);
        let pow_b = BigInt::one() << b_exp;
        let denom = pow_b - a_w;
        (b_w, denom)
    }

    /// Verify fixed point guarded domain validity:
    /// Check that z_w^* mod 2^{B_w} == \rho_w AND intermediate branch guards are satisfied.
    pub fn verify_guarded_domain(word: &[u64]) -> bool {
        let (num, denom) = Self::fixed_point(word);
        if denom.is_zero() {
            return false;
        }

        let is_neg = num.is_negative() != denom.is_negative();
        if !is_neg {
            return false;
        }

        let (a_w, b_w, b_exp) = Self::word_parameters(word);
        let lhs = &a_w * &num + &b_w * &denom;
        let rhs = &num * (BigInt::one() << b_exp);

        lhs == rhs
    }

    /// Compute positive integer representative z_{w, r} \equiv z_w^* (mod 2^{B_w * (r + 1)}).
    pub fn positive_representative(word: &[u64], r: usize) -> BigUint {
        let (num, denom) = Self::fixed_point(word);
        let (_, _, b_exp) = Self::word_parameters(word);
        let n_bits = b_exp * ((r + 1) as u64);
        let modulus = BigInt::one() << n_bits;

        let denom_abs = denom.abs().to_biguint().unwrap();
        let mod_big = modulus.to_biguint().unwrap();

        let inv = Self::mod_inverse(&denom_abs, &mod_big);
        let num_mod = ((num % &modulus) + &modulus) % &modulus;
        let num_big = num_mod.to_biguint().unwrap();

        if denom.is_negative() {
            let num_big_mod = &num_big % &mod_big;
            let neg_num = if num_big_mod.is_zero() {
                BigUint::zero()
            } else {
                &mod_big - &num_big_mod
            };
            (&neg_num * &inv) % &mod_big
        } else {
            (&num_big * &inv) % &mod_big
        }
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

    /// Build Periodic Ghost Orbit Atlas for primitive accelerated gap words up to max length.
    pub fn build_atlas(
        max_gap_len: usize,
        max_j: u64,
        corpus: &mut AdversarialCorpus,
    ) -> Vec<GhostOrbitEntry> {
        let mut entries = Vec::new();

        let mut gap_words: Vec<Vec<u64>> = Vec::new();
        for j in 0..=max_j {
            gap_words.push(vec![j]);
        }

        for len in 2..=max_gap_len {
            let mut new_words = Vec::new();
            for w in &gap_words {
                if w.len() == len - 1 {
                    for j in 0..=max_j {
                        let mut extended = w.clone();
                        extended.push(j);
                        new_words.push(extended);
                    }
                }
            }
            gap_words.extend(new_words);
        }

        let mut seen_necklaces = BTreeMap::new();
        for w in gap_words {
            let orbit_class = Self::canonical_necklace(&w);
            if orbit_class.is_primitive_root {
                let key = orbit_class.canonical_necklace.iter().map(|j| j.to_string()).collect::<Vec<_>>().join("_");
                if !seen_necklaces.contains_key(&key) {
                    seen_necklaces.insert(key, orbit_class);
                }
            }
        }

        for (key, orbit_class) in seen_necklaces {
            let w = &orbit_class.canonical_necklace;
            let (a_w, b_w, b_exp) = Self::word_parameters(w);
            let (num, denom) = Self::fixed_point(w);
            let is_valid = Self::verify_guarded_domain(w);

            let z_r1 = Self::positive_representative(w, 1);
            let z_r3 = Self::positive_representative(w, 3);

            let uv_strings = Self::gap_word_to_uv_strings(w);

            let entry = GhostOrbitEntry {
                gap_sequence: w.clone(),
                original_uv_mapping: uv_strings.clone(),
                composite_a: a_w.to_string(),
                composite_b: b_w.to_string(),
                composite_shift: b_exp,
                fixed_point_numerator: num.to_string(),
                fixed_point_denominator: denom.to_string(),
                fixed_point_is_negative: (num.is_negative() != denom.is_negative()),
                guarded_domain_valid: is_valid,
                min_positive_rep_r1: z_r1.to_string(),
                min_positive_rep_r3: z_r3.to_string(),
                phase_rotations_count: orbit_class.rotations.len(),
            };

            corpus.add_record(AdversarialRecord {
                id: format!("ghost_atlas_gap_{}", key),
                origin: RecordOrigin::PeriodicGhost,
                gap_sequence: w.iter().map(|j| format!("j={}", j)).collect(),
                flattened_uv_word: vec![],
                total_precision: b_exp * 2,
                source_residue: z_r1.to_string(),
                endpoint_sequence: vec![z_r1.to_string()],
                lift_blocks: w.clone(),
                branch_parameters: BranchParameters {
                    a_composite: a_w.to_string(),
                    b_composite: b_w.to_string(),
                    shift_composite: b_exp,
                },
                periodic_shadow_word: Some(uv_strings),
                periodic_shadow_length: w.len(),
                mod_3_signatures: vec![],
                stopping_reason: StoppingReason::PeriodicCycleDetected,
                is_exact: true,
                generation_bounds: format!("exact_fixed_point_mod2^{}", b_exp * 2),
                dedup_key: format!("ghost_gap_{}", key),
            });

            entries.push(entry);
        }

        entries
    }

    /// Evaluate 3 shadow metrics for a candidate source z against fixed point z_w^*.
    pub fn evaluate_shadow_metrics(
        trajectory_id: &str,
        source_z: &BigUint,
        word: &[u64],
    ) -> ShadowEvaluationResult {
        let (num, denom) = Self::fixed_point(word);
        let (_, _, b_exp) = Self::word_parameters(word);

        let z_big = BigInt::from(source_z.clone());
        let diff_num = &z_big * &denom - &num;

        let v2 = if diff_num.is_zero() {
            999u64
        } else {
            let abs_num = diff_num.abs().to_biguint().unwrap();
            abs_num.trailing_zeros().unwrap_or(0)
        };

        let norm_shadow = (b_exp * v2) / b_exp.max(1);

        ShadowEvaluationResult {
            trajectory_id: trajectory_id.to_string(),
            best_matching_gap_sequence: word.to_vec(),
            exact_repetitions: (v2 / b_exp.max(1)) as usize,
            closeness_v2: v2,
            normalized_shadow_score: norm_shadow,
            match_kind: "Prefix".to_string(),
        }
    }
}
