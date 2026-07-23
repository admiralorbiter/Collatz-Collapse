use crate::{MacrostepData, ValuationWord, Q1_RESIDUE};
use num_bigint::BigUint;

/// Prefix lift state containing total valuation A_s, a_s = 3^{K_s}, guard residue r_s, and endpoint y_s = T_s(r_s).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixLiftState {
    pub word: ValuationWord,
    pub total_valuation: u64,
    pub multiplier_a: BigUint,
    pub guard_residue: BigUint,
    pub endpoint_y: BigUint,
}

impl Default for PrefixLiftState {
    fn default() -> Self {
        Self::empty()
    }
}

impl PrefixLiftState {
    /// Returns the initial empty prefix state (epsilon).
    pub fn empty() -> Self {
        Self {
            word: ValuationWord::from_u32_slice(&[]).unwrap(),
            total_valuation: 0,
            multiplier_a: BigUint::from(1u32),
            guard_residue: BigUint::from(0u32),
            endpoint_y: BigUint::from(0u32),
        }
    }

    /// Construct state from a given valuation word.
    pub fn from_word(word: ValuationWord) -> Result<Self, String> {
        if word.elements().is_empty() {
            return Ok(Self::empty());
        }

        let m = MacrostepData::from_word(word.clone()).map_err(|e| e.to_string())?;
        let a = m.multiplier().clone();
        let val_a = m.total_valuation();
        let c = m.constant().clone();
        let b = BigUint::from(1u32) << val_a;

        let term_a = BigUint::from(Q1_RESIDUE) * &a;
        let term_b = BigUint::from(Q1_RESIDUE) * &b;
        if &term_a + &c < term_b {
            return Err("Overflow in eta computation".to_string());
        }
        let eta = (&term_a + &c - &term_b) >> 5;

        // r_s = (-eta * a^{-1}) \bmod 2^{A_s}
        let a_inv = hensel_inverse_3_pow_local(&a, val_a)?;
        let eta_mod = &eta % &b;
        let neg_eta_mod = if eta_mod == BigUint::from(0u32) {
            BigUint::from(0u32)
        } else {
            &b - &eta_mod
        };
        let r_s = (&neg_eta_mod * &a_inv) % &b;

        // y_s = T_s(r_s) = (a * r_s + eta) / b
        let num_y = &a * &r_s + &eta;
        let y_s = &num_y >> val_a;

        Ok(Self {
            word,
            total_valuation: val_a,
            multiplier_a: a,
            guard_residue: r_s,
            endpoint_y: y_s,
        })
    }
}

/// Lift transition data for appending a symbol p to prefix s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftTransition {
    pub symbol_is_u: bool,
    pub lift_digit: BigUint,
    pub is_zero_lift: bool,
    pub child_state: PrefixLiftState,
}

/// Computes the exact lift transition when extending prefix state s with symbol p (u or v).
pub fn extend_prefix_state(
    state: &PrefixLiftState,
    symbol_is_u: bool,
) -> Result<LiftTransition, String> {
    let (symbol_slice, g_p, a_p_val): (&[u32], BigUint, u64) = if symbol_is_u {
        (&[1, 1, 2], BigUint::from(7u32), 4)
    } else {
        (&[1, 1, 2, 1, 2, 2], BigUint::from(61u32), 9)
    };

    let b_p = BigUint::from(1u32) << a_p_val;

    // Check zero-lift condition: y_s \equiv g_p \pmod{2^{A_p}}
    let y_mod = &state.endpoint_y % &b_p;
    let is_zero_lift = y_mod == g_p;

    // Exact lift digit: tau_p(s) \equiv (g_p - y_s) * a_s^{-1} \pmod{2^{A_p}}
    let a_s_inv = hensel_inverse_3_pow_local(&state.multiplier_a, a_p_val)?;
    let diff = if y_mod <= g_p {
        &g_p - &y_mod
    } else {
        &b_p + &g_p - &y_mod
    };
    let tau_p = (&diff * &a_s_inv) % &b_p;

    // Construct child word
    let mut child_word_elems = state.word.elements();
    child_word_elems.extend_from_slice(symbol_slice);
    let child_word = ValuationWord::from_u32_slice(&child_word_elems).map_err(|e| e.to_string())?;

    let child_state = PrefixLiftState::from_word(child_word)?;

    // Cross-check: r_{sp} == r_s + tau_p * 2^{A_s}
    let b_s = BigUint::from(1u32) << state.total_valuation;
    let expected_r_sp = &state.guard_residue + &tau_p * &b_s;
    if child_state.guard_residue != expected_r_sp {
        return Err(format!(
            "Lift digit cross-check failed! child guard = {}, expected = {}",
            child_state.guard_residue, expected_r_sp
        ));
    }

    Ok(LiftTransition {
        symbol_is_u,
        lift_digit: tau_p,
        is_zero_lift,
        child_state,
    })
}

fn hensel_inverse_3_pow_local(a: &BigUint, exp: u64) -> Result<BigUint, String> {
    if exp == 0 {
        return Ok(BigUint::from(1u32));
    }
    let modulus = BigUint::from(1u32) << exp;
    let mut inv = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    for _ in 0..(exp + 5) {
        let product = (a * &inv) % &modulus;
        let factor = if product <= two {
            &two - &product
        } else {
            &modulus + &two - &product
        };
        inv = (&inv * &factor) % &modulus;
    }
    Ok(inv % modulus)
}
