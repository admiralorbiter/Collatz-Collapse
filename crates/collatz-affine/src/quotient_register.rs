use crate::inversion::hensel_inverse_3_pow;
use crate::{AffineError, MacrostepData, ValuationWord};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};

pub const Q1_RESIDUE: u64 = 7;
pub const Q1_EXPONENT: u32 = 5; // modulus 2^5 = 32

/// Exact non-negative quotient register representation k for Q_1 = { n = 32k + 7 }.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Q1Quotient {
    k: BigUint,
}

impl Q1Quotient {
    pub fn from_integer(n: &BigUint) -> Result<Self, AffineError> {
        let mod32 = (n % 32u32).to_u64_digits().first().copied().unwrap_or(0);
        if mod32 != Q1_RESIDUE || n < &BigUint::from(Q1_RESIDUE) {
            return Err(AffineError::InvalidQ1Integer(n.to_string()));
        }
        let k = (n - Q1_RESIDUE) >> Q1_EXPONENT;
        Ok(Self { k })
    }

    pub fn from_k(k: BigUint) -> Self {
        Self { k }
    }

    pub fn to_integer(&self) -> BigUint {
        (&self.k << Q1_EXPONENT) + Q1_RESIDUE
    }

    pub fn value(&self) -> &BigUint {
        &self.k
    }
}

/// Typed return outcome for macrostep transition evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnTransitionOutcome {
    NotExactWord,
    ExactButLeavesBase { image: BigUint },
    BasedReturn { next_k: Q1Quotient, image: BigUint },
}

/// Canonical quotient affine rule parameters for a macrostep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotientAffineRule {
    pub word: ValuationWord,
    pub eta: BigInt,
    pub guard_residue: BigUint,
    pub guard_modulus_exponent: u64,
}

pub struct QuotientRegisterMachine;

impl QuotientRegisterMachine {
    /// Computes \eta_p = (a_p * r + c_p - 2^{A_p} * r) / 2^q for Q_1 (r=7, q=5).
    pub fn compute_eta(macrostep: &MacrostepData) -> Result<BigInt, AffineError> {
        let r = BigInt::from(Q1_RESIDUE);
        let a_int = BigInt::from_biguint(Sign::Plus, macrostep.multiplier().clone());
        let b_int = BigInt::from_biguint(Sign::Plus, macrostep.denominator().clone());
        let c_int = macrostep.constant_int();

        let num = (&a_int * &r + c_int) - (&b_int * &r);
        let q_div = BigInt::from(1i32 << Q1_EXPONENT);

        let (quot, rem) = (num.clone() / &q_div, &num % &q_div);
        if !rem.is_zero() {
            return Err(AffineError::Overflow);
        }
        Ok(quot)
    }

    /// Derives the QuotientAffineRule for any valid macrostep.
    pub fn derive_rule(macrostep: &MacrostepData) -> Result<QuotientAffineRule, AffineError> {
        let eta = Self::compute_eta(macrostep)?;
        let mod_exp = macrostep.total_valuation();
        let modulus = BigUint::one() << mod_exp;

        let inv_a = hensel_inverse_3_pow(mod_exp);
        let k_steps = macrostep.odd_steps() as u32;

        // inv_a_k = (3^k)^-1 mod 2^A
        let inv_a_k = inv_a.modpow(&BigUint::from(k_steps), &modulus);

        // g \equiv -\eta * a^{-1} (mod 2^A)
        let neg_eta_mod = if eta.is_negative() {
            let abs_eta = eta.abs().to_biguint().unwrap() % &modulus;
            (&modulus - abs_eta) % &modulus
        } else {
            let eta_mod = eta.to_biguint().unwrap() % &modulus;
            (&modulus - (eta_mod % &modulus)) % &modulus
        };

        let guard_residue = (neg_eta_mod * inv_a_k) % modulus;

        Ok(QuotientAffineRule {
            word: macrostep.word().clone(),
            eta,
            guard_residue,
            guard_modulus_exponent: mod_exp,
        })
    }

    /// Evaluates exact word and Q_1 return status for any macrostep.
    pub fn eval_transition(
        macrostep: &MacrostepData,
        quotient: &Q1Quotient,
    ) -> Result<ReturnTransitionOutcome, AffineError> {
        let n = quotient.to_integer();
        let a = macrostep.multiplier();
        let b = macrostep.denominator();
        let c = macrostep.constant();

        let num = (a * &n) + c;
        let mod_exact = b << 1u32;

        // Exact word check: (a * n + c) \equiv b (mod 2^{A+1})
        if (&num % &mod_exact) != *b {
            return Ok(ReturnTransitionOutcome::NotExactWord);
        }

        let image = &num / b;
        let mod32 = (&image % 32u32)
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);

        if mod32 != Q1_RESIDUE {
            Ok(ReturnTransitionOutcome::ExactButLeavesBase { image })
        } else {
            let next_k = Q1Quotient::from_integer(&image)?;
            Ok(ReturnTransitionOutcome::BasedReturn { next_k, image })
        }
    }

    /// Specialized fast evaluation for u = [1,1,2].
    pub fn eval_u_transition(quotient: &Q1Quotient) -> ReturnTransitionOutcome {
        let k = quotient.value();
        // u is exact for every k \in Q_1
        let image = (BigUint::from(54u32) * k) + 13u32;
        let mod32 = (&image % 32u32)
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);

        if mod32 != Q1_RESIDUE {
            ReturnTransitionOutcome::ExactButLeavesBase { image }
        } else {
            // k' = (27k + 3) / 16
            let next_k_val = ((BigUint::from(27u32) * k) + 3u32) >> 4u32;
            ReturnTransitionOutcome::BasedReturn {
                next_k: Q1Quotient::from_k(next_k_val),
                image,
            }
        }
    }

    /// Specialized fast evaluation for v = [1,1,2,1,2,2].
    pub fn eval_v_transition(quotient: &Q1Quotient) -> ReturnTransitionOutcome {
        let k = quotient.value();
        let mod32_k = (k % 32u32).to_u64_digits().first().copied().unwrap_or(0);

        // v is exact for k \equiv 29 (mod 32)
        if mod32_k != 29 {
            return ReturnTransitionOutcome::NotExactWord;
        }

        let n = quotient.to_integer();
        let image = ((BigUint::from(729u32) * n) + 881u32) >> 9u32;
        let mod32_img = (&image % 32u32)
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);

        if mod32_img != Q1_RESIDUE {
            ReturnTransitionOutcome::ExactButLeavesBase { image }
        } else {
            // k' = (729k + 75) / 512
            let next_k_val = ((BigUint::from(729u32) * k) + 75u32) >> 9u32;
            ReturnTransitionOutcome::BasedReturn {
                next_k: Q1Quotient::from_k(next_k_val),
                image,
            }
        }
    }

    /// Solves predecessor guard for a successor guard:
    /// k \equiv (2^A * g' - \eta) * a^{-1} (mod 2^{A+M})
    pub fn preimage_guard(
        rule: &QuotientAffineRule,
        target_guard_residue: &BigUint,
        target_guard_exp: u64,
    ) -> (BigUint, u64) {
        let total_exp = rule.guard_modulus_exponent + target_guard_exp;
        let modulus = BigUint::one() << total_exp;

        let b_val = BigUint::one() << rule.guard_modulus_exponent;
        let target_term = BigInt::from_biguint(Sign::Plus, b_val * target_guard_residue);

        let diff = target_term - &rule.eta;
        let diff_mod = if diff.is_negative() {
            let abs_d = diff.abs().to_biguint().unwrap() % &modulus;
            (&modulus - abs_d) % &modulus
        } else {
            diff.to_biguint().unwrap() % &modulus
        };

        let k_steps = rule.word.len() as u32;
        let inv_a = hensel_inverse_3_pow(total_exp).modpow(&BigUint::from(k_steps), &modulus);

        let pred_residue = (diff_mod * inv_a) % modulus;
        (pred_residue, total_exp)
    }
}
