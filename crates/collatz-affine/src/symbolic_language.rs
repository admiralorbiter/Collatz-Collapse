use crate::{
    hensel_inverse_3_pow, solve_starting_residue_exact, AffineError, CanonicalCylinder,
    MacrostepData, Q1Quotient, QuotientRegisterMachine, ReturnTransitionOutcome, ValuationWord,
    Q1_EXPONENT, Q1_RESIDUE,
};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use std::collections::HashMap;

/// Symbolic word metadata including lift digit and periodic properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicWordData {
    pub word: ValuationWord,
    pub a: BigUint,
    pub total_valuation: u64,
    pub c: BigUint,
    pub d: BigInt,
    pub eta: BigUint,

    pub guard_residue: BigUint,
    pub guard_modulus_exponent: u64,
    pub least_source_n: BigUint,

    pub final_quotient_k: BigUint,

    pub parent_lift_digit: Option<BigUint>,
    pub is_zero_lift_from_parent: Option<bool>,

    pub primitive_root: ValuationWord,
    pub repetition_count: usize,
}

impl SymbolicWordData {
    pub fn from_word(word: ValuationWord) -> Result<Self, AffineError> {
        let m = MacrostepData::from_word(word.clone())?;

        let a = m.multiplier().clone();
        let total_val = m.total_valuation();
        let c = m.constant().clone();
        let d = m.d().clone();
        let b = BigUint::from(1u32) << total_val;

        // eta = (7 * a + c - 7 * b) / 32
        let term_a = BigUint::from(Q1_RESIDUE) * &a;
        let term_b = BigUint::from(Q1_RESIDUE) * &b;
        let num = &term_a + &c;
        if num < term_b {
            return Err(AffineError::Overflow);
        }
        let diff = &num - &term_b;
        let eta = &diff >> Q1_EXPONENT;

        // Guard r_s = (-eta * (3^K)^{-1}) mod 2^A
        let inv_3 = hensel_inverse_3_pow(total_val);
        let a_inv = inv_3.modpow(&BigUint::from(word.len()), &b);
        let mod_2a = &b;
        let eta_mod = &eta % mod_2a;
        let prod = (&eta_mod * &a_inv) % mod_2a;
        let r_s = if prod.is_zero() {
            BigUint::zero()
        } else {
            mod_2a - &prod
        };

        let least_source_n = (BigUint::from(32u32) * &r_s) + Q1_RESIDUE;
        let final_quotient_k = ((&a * &r_s) + &eta) >> total_val;

        let (primitive_root, repetition_count) = word.primitive_root();

        Ok(Self {
            word,
            a,
            total_valuation: total_val,
            c,
            d,
            eta,
            guard_residue: r_s,
            guard_modulus_exponent: total_val,
            least_source_n,
            final_quotient_k,
            parent_lift_digit: None,
            is_zero_lift_from_parent: None,
            primitive_root,
            repetition_count,
        })
    }
}

/// Enumerates the symbolic return language {u,v}^* and calculates child lift digits.
pub struct SymbolicLanguageEnumerator;

impl SymbolicLanguageEnumerator {
    /// Enumerates all non-empty words s \in {u,v}^r for 1 <= r <= max_depth.
    pub fn enumerate(max_depth: usize) -> Result<Vec<SymbolicWordData>, AffineError> {
        let u_word = ValuationWord::from_u32_slice(&[1, 1, 2])?;
        let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2])?;

        let mut results = Vec::new();
        let mut map: HashMap<Vec<u32>, SymbolicWordData> = HashMap::new();

        // Level 1
        let u_data = SymbolicWordData::from_word(u_word.clone())?;
        let v_data = SymbolicWordData::from_word(v_word.clone())?;

        map.insert(u_word.elements(), u_data.clone());
        map.insert(v_word.elements(), v_data.clone());
        results.push(u_data);
        results.push(v_data);

        let mut current_level_words = vec![u_word, v_word];

        for _depth in 2..=max_depth {
            let mut next_level_words = Vec::new();

            for parent_word in &current_level_words {
                let parent_data = map.get(&parent_word.elements()).unwrap().clone();

                for child_symbol in &[
                    ValuationWord::from_u32_slice(&[1, 1, 2])?,
                    ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2])?,
                ] {
                    let mut child_elems = parent_word.elements();
                    child_elems.extend_from_slice(&child_symbol.elements());
                    let child_word = ValuationWord::from_u32_slice(&child_elems)?;

                    let mut child_data = SymbolicWordData::from_word(child_word.clone())?;

                    // r_child = r_parent + lambda * 2^{A_parent}
                    let b_parent = BigUint::from(1u32) << parent_data.total_valuation;
                    let lift_digit = if child_data.guard_residue >= parent_data.guard_residue {
                        (&child_data.guard_residue - &parent_data.guard_residue) / &b_parent
                    } else {
                        // Modulo wrap around fallback if non-normalized
                        let b_child = BigUint::from(1u32) << child_data.total_valuation;
                        ((&child_data.guard_residue + &b_child) - &parent_data.guard_residue) / &b_parent
                    };
                    let is_zero = lift_digit.is_zero();

                    child_data.parent_lift_digit = Some(lift_digit);
                    child_data.is_zero_lift_from_parent = Some(is_zero);

                    map.insert(child_word.elements(), child_data.clone());
                    results.push(child_data);
                    next_level_words.push(child_word);
                }
            }

            current_level_words = next_level_words;
        }

        Ok(results)
    }

    /// Cross-validates 3 independent guard constructions:
    /// 1. Recursive quotient preimages.
    /// 2. Composite quotient map T_s(k) = (a_s * k + eta_s) / 2^{A_s}.
    /// 3. Flattened n-space cylinder.
    pub fn cross_validate_guards(data: &SymbolicWordData) -> Result<bool, AffineError> {
        let _q_base = CanonicalCylinder::new(BigUint::from(7u32), 5);

        // 1. Recursive preimage k_guard
        let mut curr_q = Q1Quotient::from_k(data.guard_residue.clone());
        let elems = data.word.elements();
        let mut symbols = Vec::new();
        let mut i = 0;
        while i < elems.len() {
            if i + 6 <= elems.len() && elems[i..i + 6] == [1, 1, 2, 1, 2, 2] {
                symbols.push(MacrostepData::from_word(ValuationWord::from_u32_slice(
                    &[1, 1, 2, 1, 2, 2],
                )?)?);
                i += 6;
            } else if i + 3 <= elems.len() && elems[i..i + 3] == [1, 1, 2] {
                symbols.push(MacrostepData::from_word(ValuationWord::from_u32_slice(
                    &[1, 1, 2],
                )?)?);
                i += 3;
            } else {
                return Ok(false);
            }
        }

        for step_m in &symbols {
            let outcome = QuotientRegisterMachine::eval_transition(step_m, &curr_q)?;
            match outcome {
                ReturnTransitionOutcome::BasedReturn { next_k, .. } => {
                    curr_q = next_k;
                }
                other => {
                    eprintln!("Method 1 failed at eval_transition for word {:?}: outcome is {:?}", data.word, other);
                    return Ok(false);
                }
            }
        }
        if curr_q.value() != &data.final_quotient_k {
            eprintln!("Method 1 failed final_k mismatch: curr_q={} vs final_k={}", curr_q.value(), data.final_quotient_k);
            return Ok(false);
        }

        // 2. Composite quotient map solver
        let comp_expr = (&data.a * &data.guard_residue) + &data.eta;
        let comp_b = BigUint::from(1u32) << data.total_valuation;
        if (&comp_expr % &comp_b) != BigUint::zero() {
            eprintln!("Method 2 failed non-zero mod");
            return Ok(false);
        }
        let comp_k_next = &comp_expr >> data.total_valuation;
        if comp_k_next != data.final_quotient_k {
            eprintln!("Method 2 failed final_k mismatch");
            return Ok(false);
        }

        // 3. Flattened n-space cylinder
        let exact_cyl_res =
            solve_starting_residue_exact(&data.c, data.word.len(), data.total_valuation)?;
        let n_guard = (BigUint::from(32u32) * &data.guard_residue) + Q1_RESIDUE;
        let mod_n_exact = BigUint::from(1u32) << (data.total_valuation + 1);

        let n_guard_rem = &n_guard % &mod_n_exact;
        if n_guard_rem != exact_cyl_res {
            eprintln!("Method 3 failed: (n_guard % mod_n_exact)={} vs exact_cyl_res={}", n_guard_rem, exact_cyl_res);
            return Ok(false);
        }

        Ok(true)
    }
}
