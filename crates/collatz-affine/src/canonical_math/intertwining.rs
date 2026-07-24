use crate::canonical_math::branch::CanonicalBranch;
use crate::canonical_math::types::{
    CoreAffineConstant, LiveBlockConstant, QuotientRegisterState, ValuationWord,
};

/// Failure reasons for intertwining verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntertwiningFailure {
    LiveQuotientMismatch { lhs: num_bigint::BigInt, rhs: num_bigint::BigInt },
    CoreMismatch { lhs: num_bigint::BigInt, rhs: num_bigint::BigInt },
    CoboundaryMismatch { lhs: num_bigint::BigInt, rhs: num_bigint::BigInt },
}

/// Verifies live quotient-register intertwining M_w * k(n') == Q_w * k(n) + \eta_w.
pub fn verify_live_quotient_intertwining(
    source_k: &QuotientRegisterState,
    target_k: &QuotientRegisterState,
    word: &ValuationWord,
    eta: &LiveBlockConstant,
) -> Result<(), IntertwiningFailure> {
    let k_steps = word.k_steps();
    let b = word.total_exponent_b();

    let m_w = num_bigint::BigInt::from(1u32) << b;
    let q_w = num_bigint::BigInt::from(3u32).pow(k_steps);

    let lhs = &m_w * target_k.quotient();
    let rhs = (&q_w * source_k.quotient()) + &eta.0;

    if lhs != rhs {
        Err(IntertwiningFailure::LiveQuotientMismatch { lhs, rhs })
    } else {
        Ok(())
    }
}

/// Verifies periodic-core intertwining M_j * D_j == Q_j * C_j + \beta_j.
pub fn verify_core_intertwining(
    branch: &CanonicalBranch,
) -> Result<(), IntertwiningFailure> {
    let m_j = num_bigint::BigInt::from(branch.modulus.clone());
    let q_j = num_bigint::BigInt::from(branch.multiplier.clone());

    let lhs = &m_j * &branch.endpoint_core.0;
    let rhs = (&q_j * &branch.source_core.0) + &branch.beta.0;

    if lhs != rhs {
        Err(IntertwiningFailure::CoreMismatch { lhs, rhs })
    } else {
        Ok(())
    }
}

/// Verifies coboundary transformation \beta = a * \eta + M * b_t - Q * b_s.
pub fn verify_coboundary_reconciliation(
    eta: &LiveBlockConstant,
    beta: &CoreAffineConstant,
    m: &num_bigint::BigInt,
    q: &num_bigint::BigInt,
    a: &num_bigint::BigInt,
    b_s: &num_bigint::BigInt,
    b_t: &num_bigint::BigInt,
) -> Result<(), IntertwiningFailure> {
    let lhs = beta.0.clone();
    let rhs = (a * &eta.0) + (m * b_t) - (q * b_s);

    if lhs != rhs {
        Err(IntertwiningFailure::CoboundaryMismatch { lhs, rhs })
    } else {
        Ok(())
    }
}
