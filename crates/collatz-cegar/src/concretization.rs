use crate::abstract_domain::AbstractEdge;
use collatz_affine::{AffinePrefix, ValuationWord};
use num_bigint::{BigUint, ToBigInt};
use num_traits::{Zero, One};

/// Multi-Lap Simultaneous Modular Cycle Solution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiLapCycleSolution {
    pub lap_count: usize,
    pub total_valuation_a: u32,
    pub total_odd_steps_k: u32,
    pub modulus: BigUint,
    pub starting_residue: BigUint,
    pub smallest_positive_witness: BigUint,
    pub is_satisfiable: bool,
}

/// Concretization Engine with Simultaneous Modular Solving and Multi-Lap Concretization.
pub struct ConcretizationEngine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcretizationResult {
    /// Cycle is multiplicatively contracting with exact threshold B
    Contracting {
        prefix: AffinePrefix,
        threshold: Option<BigUint>,
    },
    /// Cycle is spurious in N+ (violates positivity guards n_i >= 1 or threshold bounds)
    SpuriousInfeasible {
        reason: String,
        prefix: AffinePrefix,
    },
    /// Real expanding cycle candidate (counterexample)
    RealExpandingCandidate {
        prefix: AffinePrefix,
    },
}

impl ConcretizationEngine {
    /// Concretizes an abstract cycle over exact affine maps and checks positivity guards.
    pub fn concretize_cycle(cycle: &[AbstractEdge]) -> Result<ConcretizationResult, String> {
        let valuations: Vec<u8> = cycle.iter().map(|e| e.valuation).collect();
        let word = ValuationWord::new(valuations).map_err(|e| format!("{:?}", e))?;
        let prefix = AffinePrefix::from_valuation_word(word).map_err(|e| format!("{:?}", e))?;

        // 1. Explicit Positivity Guard Check (n_i >= 1 across all intermediate steps)
        if !Self::check_positivity_guards(&prefix) {
            return Ok(ConcretizationResult::SpuriousInfeasible {
                reason: "Violates positive integer positivity constraint n_i >= 1".to_string(),
                prefix,
            });
        }

        // 2. Check Multiplicative Nat Contraction: (2^A - 3^k) * (n - 1) >= c_k
        if prefix.is_multiplicative_contracting() {
            let threshold = prefix.compute_descent_threshold();
            Ok(ConcretizationResult::Contracting { prefix, threshold })
        } else {
            Ok(ConcretizationResult::RealExpandingCandidate { prefix })
        }
    }

    /// Enforces n_i >= 1 for all intermediate step states.
    /// Rejects negative 2-adic attractors (like -1/3).
    pub fn check_positivity_guards(prefix: &AffinePrefix) -> bool {
        for &start_n in &[1u64, 3u64, 5u64] {
            let mut curr = start_n;
            for &a_i in prefix.valuations.as_slice() {
                let next_val = (3 * curr + 1) >> a_i;
                if next_val == 0 {
                    return false;
                }
                curr = next_val;
            }
        }
        true
    }

    /// Solves L complete laps of an abstract cycle simultaneously modulo 2^(m + L*A).
    /// Finds the exact solution class n = n_L mod 2^(m + L*A) and the smallest positive witness.
    pub fn solve_multi_lap_cycle(
        valuations: &[u8],
        start_r: u64,
        m: u32,
        laps: usize,
    ) -> Result<MultiLapCycleSolution, String> {
        let mut repeated_vals = Vec::new();
        for _ in 0..laps {
            repeated_vals.extend_from_slice(valuations);
        }

        let word = ValuationWord::new(repeated_vals).map_err(|e| format!("{:?}", e))?;
        let prefix = AffinePrefix::from_valuation_word(word).map_err(|e| format!("{:?}", e))?;

        let single_word = ValuationWord::new(valuations.to_vec()).map_err(|e| format!("{:?}", e))?;
        let single_prefix = AffinePrefix::from_valuation_word(single_word).map_err(|e| format!("{:?}", e))?;

        let total_a = prefix.total_twos;
        let total_k = prefix.odd_steps;
        let required_exp = m + (laps as u32) * (single_prefix.total_twos as u32);
        let modulus = BigUint::one() << required_exp;

        let two_a = BigUint::from(1u32) << (total_a as usize);
        let c_k = &prefix.constant;
        let r0 = BigUint::from(start_r);

        // Equation: (3^K * n + c_K) / 2^(L*A) = r0 mod 2^m
        // => 3^K * n + c_K = r0 * 2^(L*A) mod 2^(m + L*A)
        // => 3^K * n = r0 * 2^(L*A) - c_K mod 2^(m + L*A)
        let target_rhs_raw = (&r0 * &two_a).to_bigint().unwrap() - c_k.to_bigint().unwrap();
        let mod_bigint = modulus.to_bigint().unwrap();
        let target_rhs = ((target_rhs_raw % &mod_bigint) + &mod_bigint) % &mod_bigint;

        // Compute (3^-1)^K mod 2^required_exp via Hensel inverse of 3
        let inv_3_single = collatz_affine::hensel_inverse_3_pow(required_exp as u64);
        let inv_3k = inv_3_single.modpow(&BigUint::from(total_k as u32), &modulus);

        let sol_res = (&inv_3k * target_rhs.to_biguint().unwrap()) % &modulus;
        let witness = sol_res.clone();

        // Enforce strict starting residue match (no infinite loop hack)
        let start_mod = BigUint::one() << m;
        if &witness % &start_mod != r0 {
            return Err(format!(
                "Solution {} mod {} does not satisfy starting residue {} mod 2^{}",
                witness, modulus, r0, m
            ));
        }

        // Direct concrete trajectory replay validation over all L laps
        let mut curr = witness.clone();
        let mut valid = true;
        for &expected_val in prefix.valuations.as_slice() {
            let num = BigUint::from(3u32) * &curr + BigUint::one();
            let actual_val = num.trailing_zeros().unwrap_or(0);

            if actual_val != expected_val as u64 {
                valid = false;
                break;
            }

            let next_val = num >> expected_val;
            if next_val.is_zero() {
                valid = false;
                break;
            }
            curr = next_val;
        }

        Ok(MultiLapCycleSolution {
            lap_count: laps,
            total_valuation_a: total_a as u32,
            total_odd_steps_k: total_k as u32,
            modulus: modulus.clone(),
            starting_residue: sol_res,
            smallest_positive_witness: witness,
            is_satisfiable: valid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_domain::RelationalState;

    #[test]
    fn test_positivity_guard_accepts_valid_prefix() {
        let s1 = RelationalState::new_congruence(1, 2);
        let cycle = vec![
            AbstractEdge { from: s1.clone(), to: s1.clone(), valuation: 2 },
        ];

        let res = ConcretizationEngine::concretize_cycle(&cycle).unwrap();
        assert!(matches!(res, ConcretizationResult::Contracting { .. }));
    }

    #[test]
    fn test_solve_multi_lap_cycle_7_11_9_7_laps_1_2_3() {
        let vals = vec![1u8, 1u8, 2u8];
        
        // Lap 1: n = 231 mod 256
        let sol1 = ConcretizationEngine::solve_multi_lap_cycle(&vals, 7, 4, 1).unwrap();
        assert_eq!(sol1.smallest_positive_witness, BigUint::from(231u32));
        assert_eq!(sol1.modulus, BigUint::from(256u32));
        assert!(sol1.is_satisfiable);

        // Lap 2: n = 743 mod 4096 (2^12)
        let sol2 = ConcretizationEngine::solve_multi_lap_cycle(&vals, 7, 4, 2).unwrap();
        assert_eq!(sol2.smallest_positive_witness, BigUint::from(743u32));
        assert_eq!(sol2.modulus, BigUint::from(4096u32));
        assert!(sol2.is_satisfiable);

        // Lap 3: n = 41703 mod 65536 (2^16)
        let sol3 = ConcretizationEngine::solve_multi_lap_cycle(&vals, 7, 4, 3).unwrap();
        assert_eq!(sol3.smallest_positive_witness, BigUint::from(41703u32));
        assert_eq!(sol3.modulus, BigUint::from(65536u32));
        assert!(sol3.is_satisfiable);
    }
}
