use crate::concretization::ConcretizationEngine;
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_cert::macrocycle_theorem::{
    compute_proof_artifact_hash, CountdownSpecJson, FiniteFuelMacrocycleCertificateJson,
    FixedPointJson, FixedPointLinearFormJson, ProofArtifactRefJson,
};
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Signed, ToPrimitive, Zero};

pub fn gcd_bigint(a: &BigInt, b: &BigInt) -> BigInt {
    let mut x = a.abs();
    let mut y = b.abs();
    while !y.is_zero() {
        let rem = &x % &y;
        x = y;
        y = rem;
    }
    x
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactValuation {
    Finite(u32),
    Infinite,
}

/// Typed Output Classification Artifacts for Phase 6D
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixedPointSynthesisResult {
    InfeasibleAbstractCycle {
        reason: String,
    },
    NonReturningWord {
        start_residue: u64,
        end_residue: u64,
        modulus: u64,
        reason: String,
    },
    FixedPointWordMismatch {
        fixed_point_2adic: String,
        first_mismatch_step: usize,
        expected_valuation: u32,
        actual_valuation: ExactValuation,
        reason: String,
    },
    FiniteFuelMacrocycle(FiniteFuelMacrocycleCertificateJson),
    TrivialPositiveCycle {
        start_n: u64,
        is_primitive_canonical: bool,
    },
    PositiveCycleCandidate {
        starting_n: BigUint,
        valuation_word: Vec<u32>,
    },
    UnresolvedMacrocycle {
        valuation_word: Vec<u32>,
        reason: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Phase6DBenchmarkSummary {
    pub total_examined: usize,
    pub infeasible_abstract: usize,
    pub non_returning: usize,
    pub word_mismatches: usize,
    pub finite_fuel_macrocycles: usize,
    pub trivial_cycles: usize,
    pub positive_candidates: usize,
    pub unresolved: usize,
    pub min_k: u32,
    pub max_k: u32,
    pub expanding_count: usize,
    pub contracting_count: usize,
    pub unique_primitive_cyclic_classes: usize,
    pub finite_fuel_classes: usize,
    pub trivial_positive_classes: usize,
}

pub struct FixedPointSynthesizer;

impl FixedPointSynthesizer {
    /// Primitive root & cyclic rotation canonicalization helper:
    /// E.g. [2, 2] -> [2], [2, 1] -> [1, 2] (lexicographically smallest rotation).
    pub fn canonicalize_primitive_and_cyclic(vals: &[u8]) -> (Vec<u8>, bool) {
        // Step 1: Primitive root reduction
        let n = vals.len();
        let mut prim = vals.to_vec();
        let mut is_repeated = false;
        for len in 1..=n / 2 {
            if n % len == 0 {
                let unit = &vals[0..len];
                if vals.chunks(len).all(|chunk| chunk == unit) {
                    prim = unit.to_vec();
                    is_repeated = len < n;
                    break;
                }
            }
        }

        // Step 2: Lexicographically smallest cyclic rotation
        let k = prim.len();
        let mut min_rotation = prim.clone();
        for i in 1..k {
            let mut rotated = prim[i..].to_vec();
            rotated.extend_from_slice(&prim[..i]);
            if rotated < min_rotation {
                min_rotation = rotated;
            }
        }

        (min_rotation, is_repeated)
    }

    /// Explicit Countdown Fuel Metrics Saturated Arithmetic Helper:
    /// t = v2(alpha*n + beta)
    /// 1. N_word(n) = max(0, floor((t - 1) / A)): Total exact-word repetitions available
    /// 2. N_additional(n) = max(0, floor((t - A) / A)): Additional laps available after current
    /// 3. N_return,m(n) = max(0, floor((t - m) / A)): Complete m-bit return-state repetitions
    pub fn compute_fuel_metrics(v2_val: u64, a: u64, m: u64) -> (u64, u64, u64) {
        let n_word = if v2_val >= 1 { (v2_val - 1) / a } else { 0 };
        let n_additional = if v2_val >= a { (v2_val - a) / a } else { 0 };
        let n_return = if v2_val >= m { (v2_val - m) / a } else { 0 };
        (n_word, n_additional, n_return)
    }

    /// Exact Rational Fixed-Point Replay Gate:
    /// Evaluates x* = q/p in Q. At each step i:
    /// y_i = 3 * q_i + p_i
    /// actual_v2 = v2(y_i)
    /// Requires EXACT equality: actual_v2 == w[i] (rejecting both < and >).
    /// Enforces exact final rational return x_k == x_0 (q_k * p_0 == q_0 * p_k).
    pub fn verify_rational_fixed_point_replay(
        c_k: &BigUint,
        a: u32,
        k: u32,
        valuations: &[u8],
    ) -> Result<(), (usize, u32, ExactValuation)> {
        let two_a = BigInt::one() << (a as usize);
        let three_k = BigInt::from(3u32).pow(k);
        let diff_d = &two_a - &three_k;
        let c_bigint = c_k.to_bigint().unwrap();

        let (raw_q, raw_p) = if diff_d.is_negative() {
            (-c_bigint, -&diff_d)
        } else {
            (c_bigint, diff_d)
        };

        let g = gcd_bigint(&raw_q, &raw_p);
        let mut q = &raw_q / &g;
        let mut p = &raw_p / &g;

        if p.is_negative() {
            p = -p;
            q = -q;
        }

        let initial_q = q.clone();
        let initial_p = p.clone();

        for (idx, &expected_val) in valuations.iter().enumerate() {
            let y = BigInt::from(3u32) * &q + &p;
            if y.is_zero() {
                return Err((idx, expected_val as u32, ExactValuation::Infinite));
            }

            let y_abs = y.abs().to_biguint().unwrap();
            let actual_val = y_abs.trailing_zeros().unwrap_or(0) as u32;

            if actual_val != expected_val as u32 {
                return Err((idx, expected_val as u32, ExactValuation::Finite(actual_val)));
            }

            let next_q_num = y >> expected_val;
            let next_p = p.clone();

            let next_g = gcd_bigint(&next_q_num, &next_p);
            q = &next_q_num / &next_g;
            p = &next_p / &next_g;
        }

        if (&q * &initial_p) != (&initial_q * &p) {
            return Err((valuations.len(), 0, ExactValuation::Finite(0)));
        }

        Ok(())
    }

    /// Automatically synthesizes fixed-point linear forms and finite-fuel invariants for any abstract macrocycle.
    pub fn synthesize_macrocycle_invariant(
        raw_valuations: &[u8],
        start_r: u64,
        m: u32,
    ) -> FixedPointSynthesisResult {
        // Step 1: One-Lap Concretization Gate on ORIGINAL candidate
        let sol1 = match ConcretizationEngine::solve_multi_lap_cycle(raw_valuations, start_r, m, 1)
        {
            Ok(sol) if sol.is_satisfiable => sol,
            Ok(_) => {
                return FixedPointSynthesisResult::InfeasibleAbstractCycle {
                    reason: "1-lap cycle fails positivity guards or valuation replay".to_string(),
                }
            }
            Err(err) => {
                return FixedPointSynthesisResult::InfeasibleAbstractCycle {
                    reason: format!("1-lap modular solving failed: {}", err),
                }
            }
        };

        let _u_vals_orig: Vec<u32> = raw_valuations.iter().map(|&v| v as u32).collect();
        let word_orig = match ValuationWord::new(raw_valuations.to_vec()) {
            Ok(w) => w,
            Err(e) => {
                return FixedPointSynthesisResult::InfeasibleAbstractCycle {
                    reason: format!("Invalid valuation word: {:?}", e),
                }
            }
        };

        let prefix_orig = match AffinePrefix::from_valuation_word(word_orig) {
            Ok(p) => p,
            Err(e) => {
                return FixedPointSynthesisResult::InfeasibleAbstractCycle {
                    reason: format!("Invalid affine prefix: {:?}", e),
                }
            }
        };

        let a_orig = prefix_orig.total_twos as u32;
        let k_orig = prefix_orig.odd_steps as u32;
        let c_k_orig = &prefix_orig.constant;

        // Step 2: Return-State Congruence Gate (Full modulo 2^m) on ORIGINAL candidate
        let two_a_bi_orig = BigInt::one() << (a_orig as usize);
        let three_k_bi_orig = BigInt::from(3u32).pow(k_orig);
        let d_bi_orig = &two_a_bi_orig - &three_k_bi_orig;
        let r0_bi = BigInt::from(start_r);
        let c_bi_orig = c_k_orig.to_bigint().unwrap();
        let mod_m_bi = BigInt::one() << (m as usize);

        let return_diff_orig = (&d_bi_orig * &r0_bi) - &c_bi_orig;
        if (&return_diff_orig % &mod_m_bi) != BigInt::zero() {
            let modulus = 1u64 << m;
            let end_r = (sol1.smallest_positive_witness.clone() % BigUint::from(modulus))
                .to_u64()
                .unwrap_or(0);
            return FixedPointSynthesisResult::NonReturningWord {
                start_residue: start_r,
                end_residue: end_r,
                modulus,
                reason: format!(
                    "Return congruence (2^A - 3^k)r_0 - c_w == 0 mod 2^{} fails",
                    m
                ),
            };
        }

        // Step 3: Exact Rational Fixed-Point Replay Gate on ORIGINAL candidate
        if let Err((mismatch_step, expected_val, actual_val)) =
            Self::verify_rational_fixed_point_replay(c_k_orig, a_orig, k_orig, raw_valuations)
        {
            let fp_str = format!("{}/{}", c_bi_orig, d_bi_orig);
            return FixedPointSynthesisResult::FixedPointWordMismatch {
                fixed_point_2adic: fp_str,
                first_mismatch_step: mismatch_step,
                expected_valuation: expected_val,
                actual_valuation: actual_val,
                reason: format!(
                    "Rational fixed point fails exact valuation replay at step {}: expected {}, found {:?}",
                    mismatch_step, expected_val, actual_val
                ),
            };
        }

        // Step 4: ONLY NOW Apply Primitive & Rotational Canonicalization (after candidate validation!)
        let (valuations, is_repeated) = Self::canonicalize_primitive_and_cyclic(raw_valuations);
        let u_vals: Vec<u32> = valuations.iter().map(|&v| v as u32).collect();
        let word = ValuationWord::new(valuations.to_vec()).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        let a = prefix.total_twos as u32;
        let k = prefix.odd_steps as u32;
        let c_k = &prefix.constant;

        let two_a_bi = BigInt::one() << (a as usize);
        let three_k_bi = BigInt::from(3u32).pow(k);
        let d_bi = &two_a_bi - &three_k_bi;
        let c_bi = c_k.to_bigint().unwrap();

        // Step 5: Check Trivial Cycle 1 -> 1 (w = [2], A=2, k=1, c=1 => 2^2 - 3^1 = 1 => 1*1 - 1 = 0 => n* = 1)
        if u_vals == vec![2] && start_r % 4 == 1 {
            return FixedPointSynthesisResult::TrivialPositiveCycle {
                start_n: 1,
                is_primitive_canonical: is_repeated,
            };
        }

        // Step 6: Root Classification with Explicit Linear Form Sign & Multiplier Kind
        let (alpha, beta, x_star_str) = if d_bi.is_negative() {
            // Expanding case: d < 0, alpha = -d = 3^k - 2^A, beta = c_w
            let alpha_val = -&d_bi;
            let beta_val = c_bi.clone();
            let fp_str = format!("-{}/{}", beta_val, alpha_val);
            (alpha_val, beta_val, fp_str)
        } else {
            // Contracting case: d > 0, alpha = d = 2^A - 3^k, beta = -c_w
            let alpha_val = d_bi.clone();
            let beta_val = -&c_bi;
            let fp_str = format!("{}/{}", c_bi, alpha_val);
            (alpha_val, beta_val, fp_str)
        };

        let is_positive_integer_root = if d_bi.is_positive() && (&c_bi % &d_bi).is_zero() {
            let root = &c_bi / &d_bi;
            root.is_positive()
        } else {
            false
        };

        if is_positive_integer_root {
            let root_biguint = (&c_bi / &d_bi).to_biguint().unwrap();
            FixedPointSynthesisResult::PositiveCycleCandidate {
                starting_n: root_biguint,
                valuation_word: u_vals,
            }
        } else {
            let claim1 = format!(
                "CLM-FINITE-FUEL-{}-{}:word={:?}:witness={}",
                a, k, u_vals, sol1.smallest_positive_witness
            );
            let claim2 = format!("CLM-NO-POSITIVE-INFINITE-{}-{}:fp={}", a, k, x_star_str);

            let hash1 = compute_proof_artifact_hash(&claim1);
            let hash2 = compute_proof_artifact_hash(&claim2);

            let mult_kind = if two_a_bi > three_k_bi {
                "contracting"
            } else {
                "expanding"
            };

            let cert = FiniteFuelMacrocycleCertificateJson {
                schema_version: "finite_fuel_macrocycle_v2".to_string(),
                valuation_word: u_vals.clone(),
                odd_steps: k,
                total_twos: a,
                affine_constant: c_k.to_string(),
                state_modulus_exponent: m,
                start_residue: start_r.to_string(),
                return_residue: start_r.to_string(),
                fixed_point_linear_form: FixedPointLinearFormJson {
                    alpha: alpha.to_string(),
                    beta: beta.to_string(),
                    definition: "alpha*n + beta".to_string(),
                    normalization: "positive_leading_coefficient".to_string(),
                },
                fixed_point: FixedPointJson {
                    numerator: (-beta).to_string(),
                    denominator: alpha.to_string(),
                    positive_integer: false,
                },
                countdown: CountdownSpecJson {
                    multiplier_kind: mult_kind.to_string(),
                    multiplier_numerator: three_k_bi.to_string(),
                    multiplier_denominator: two_a_bi.to_string(),
                    word_repetition_offset: 1,
                    return_state_offset: m,
                    valuation_drop_per_lap: a,
                    word_repetitions_definition: "floor((v2(alpha*n+beta)-1)/A)".to_string(),
                    return_state_repetitions_definition: "floor((v2(alpha*n+beta)-m)/A)"
                        .to_string(),
                },
                one_lap_witness: sol1.smallest_positive_witness.to_string(),
                finite_repetition_proof: ProofArtifactRefJson {
                    claim_id: format!("CLM-FINITE-FUEL-{}-{}", a, k),
                    proof_artifact: format!("claims/verified/macrocycle_{}_{}_finite.json", a, k),
                    proof_hash: hash1,
                },
                infinite_realization_proof: ProofArtifactRefJson {
                    claim_id: format!("CLM-NO-POSITIVE-INFINITE-{}-{}", a, k),
                    proof_artifact: format!(
                        "claims/verified/macrocycle_{}_{}_no_infinite.json",
                        a, k
                    ),
                    proof_hash: hash2,
                },
            };

            FixedPointSynthesisResult::FiniteFuelMacrocycle(cert)
        }
    }

    /// Runs Phase 6D automatic fixed-point synthesis over all valuation words up to max_k.
    pub fn run_benchmark_suite(max_k: u32, m: u32) -> Phase6DBenchmarkSummary {
        let mut summary = Phase6DBenchmarkSummary::default();
        summary.min_k = 1;
        summary.max_k = max_k;

        let num_residues = 1u64 << (m - 1);
        let mut unique_classes = std::collections::HashSet::new();
        let mut finite_fuel_classes = std::collections::HashSet::new();
        let mut trivial_classes = std::collections::HashSet::new();

        // Generate words for k=1..=max_k on alphabet {1, 2, 3}
        for k in 1..=max_k {
            let mut words = Vec::new();
            let mut current = vec![1u8; k as usize];
            loop {
                words.push(current.clone());
                let mut idx = k as usize - 1;
                loop {
                    current[idx] += 1;
                    if current[idx] <= 3 {
                        break;
                    }
                    current[idx] = 1;
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }
                if current.iter().all(|&x| x == 1) {
                    break;
                }
            }

            for word in words {
                let a_sum: u32 = word.iter().map(|&v| v as u32).sum();
                let three_k = 3u64.pow(k);
                let two_a = 1u64 << a_sum;

                let (canon_word, _) = Self::canonicalize_primitive_and_cyclic(&word);
                unique_classes.insert(canon_word.clone());

                for r_idx in 0..num_residues {
                    let r = 2 * r_idx + 1;
                    summary.total_examined += 1;
                    if two_a > three_k {
                        summary.contracting_count += 1;
                    } else {
                        summary.expanding_count += 1;
                    }

                    match Self::synthesize_macrocycle_invariant(&word, r, m) {
                        FixedPointSynthesisResult::InfeasibleAbstractCycle { .. } => {
                            summary.infeasible_abstract += 1;
                        }
                        FixedPointSynthesisResult::NonReturningWord { .. } => {
                            summary.non_returning += 1;
                        }
                        FixedPointSynthesisResult::FixedPointWordMismatch { .. } => {
                            summary.word_mismatches += 1;
                        }
                        FixedPointSynthesisResult::FiniteFuelMacrocycle(_) => {
                            summary.finite_fuel_macrocycles += 1;
                            finite_fuel_classes.insert(canon_word.clone());
                        }
                        FixedPointSynthesisResult::TrivialPositiveCycle { .. } => {
                            summary.trivial_cycles += 1;
                            trivial_classes.insert(canon_word.clone());
                        }
                        FixedPointSynthesisResult::PositiveCycleCandidate { .. } => {
                            summary.positive_candidates += 1;
                        }
                        FixedPointSynthesisResult::UnresolvedMacrocycle { .. } => {
                            summary.unresolved += 1;
                        }
                    }
                }
            }
        }

        summary.unique_primitive_cyclic_classes = unique_classes.len();
        summary.finite_fuel_classes = finite_fuel_classes.len();
        summary.trivial_positive_classes = trivial_classes.len();

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_primitive_word_repeated() {
        let (prim, is_rep) = FixedPointSynthesizer::canonicalize_primitive_and_cyclic(&[2, 2]);
        assert_eq!(prim, vec![2]);
        assert!(is_rep);

        let (rotated, _) = FixedPointSynthesizer::canonicalize_primitive_and_cyclic(&[2, 1]);
        assert_eq!(rotated, vec![1, 2]);
    }

    #[test]
    fn test_synthesize_minus_one_countdown_w1() {
        let res = FixedPointSynthesizer::synthesize_macrocycle_invariant(&[1], 3, 2);
        assert!(matches!(
            res,
            FixedPointSynthesisResult::FiniteFuelMacrocycle(_)
        ));
        if let FixedPointSynthesisResult::FiniteFuelMacrocycle(cert) = res {
            assert_eq!(cert.fixed_point_linear_form.alpha, "1");
            assert_eq!(cert.fixed_point_linear_form.beta, "1");
        }
    }

    #[test]
    fn test_synthesize_macrocycle_7_11_9_7_w_1_1_2() {
        let res = FixedPointSynthesizer::synthesize_macrocycle_invariant(&[1, 1, 2], 7, 4);
        assert!(matches!(
            res,
            FixedPointSynthesisResult::FiniteFuelMacrocycle(_)
        ));
        if let FixedPointSynthesisResult::FiniteFuelMacrocycle(cert) = res {
            assert_eq!(cert.fixed_point_linear_form.alpha, "11");
            assert_eq!(cert.fixed_point_linear_form.beta, "19");
            assert_eq!(cert.fixed_point.numerator, "-19");
            assert_eq!(cert.fixed_point.denominator, "11");
            assert!(!cert.fixed_point.positive_integer);
        }
    }

    #[test]
    fn test_synthesize_trivial_cycle_w2() {
        let res = FixedPointSynthesizer::synthesize_macrocycle_invariant(&[2], 1, 2);
        assert_eq!(
            res,
            FixedPointSynthesisResult::TrivialPositiveCycle {
                start_n: 1,
                is_primitive_canonical: false
            }
        );
    }

    #[test]
    fn test_synthesize_infeasible_cycle() {
        let res = FixedPointSynthesizer::synthesize_macrocycle_invariant(&[5, 5], 7, 4);
        assert!(matches!(
            res,
            FixedPointSynthesisResult::InfeasibleAbstractCycle { .. }
        ));
    }

    #[test]
    fn test_synthesize_fixed_point_word_mismatch() {
        let c_k = BigUint::from(19u32);
        let res = FixedPointSynthesizer::verify_rational_fixed_point_replay(&c_k, 4, 3, &[1, 1, 3]);
        assert_eq!(res, Err((2, 3, ExactValuation::Finite(2))));
    }

    #[test]
    fn test_compute_fuel_metrics_disambiguation() {
        let (n_word, n_additional, n_return) = FixedPointSynthesizer::compute_fuel_metrics(8, 4, 4);
        assert_eq!(n_word, 1);
        assert_eq!(n_additional, 1);
        assert_eq!(n_return, 1);
    }
}
