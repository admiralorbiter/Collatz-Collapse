use crate::AffineError;
use num_bigint::{BigInt, BigUint};
use num_traits::One;

/// Strictly increasing schedule of 2-adic source precisions H_0 < H_1 < H_2 < ...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecisionSchedule {
    pub levels: Vec<u64>,
}

impl PrecisionSchedule {
    pub fn new(levels: Vec<u64>) -> Result<Self, AffineError> {
        if levels.is_empty() {
            return Err(AffineError::EmptyValuationWord);
        }
        for i in 0..levels.len() - 1 {
            if levels[i + 1] <= levels[i] {
                return Err(AffineError::Overflow);
            }
        }
        Ok(Self { levels })
    }
}

/// Least non-negative representative R_n = r_n mod 2^{H_n} in [0, 2^{H_n} - 1].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectiveResidue {
    pub precision: u64,
    pub least_representative: BigUint,
}

impl ProjectiveResidue {
    pub fn from_bigint(r_n: &BigInt, precision: u64) -> Self {
        let mod_val = BigInt::one() << (precision as usize);
        let rem = ((r_n % &mod_val) + &mod_val) % &mod_val;
        Self {
            precision,
            least_representative: rem.to_biguint().unwrap(),
        }
    }
}

/// Variable-width lift block \lambda_{n+1} = (R_{n+1} - R_n) / 2^{H_n} in [0, 2^{H_{n+1}-H_n} - 1].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftBlock {
    pub from_precision: u64,
    pub to_precision: u64,
    pub value: BigUint,
}

/// Certificate asserting stabilization stage K and stabilized value N.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizationCertificate {
    pub stabilization_stage: usize,
    pub stabilized_value: BigUint,
}

/// Verifies compatibility R_{n+1} \equiv R_n mod 2^{H_n} and returns the non-negative lift block.
pub fn verify_compatible_pair(
    previous: &ProjectiveResidue,
    next: &ProjectiveResidue,
) -> Result<LiftBlock, AffineError> {
    if next.precision <= previous.precision {
        return Err(AffineError::Overflow);
    }
    let mod_prev = BigUint::one() << (previous.precision as usize);
    if (&next.least_representative % &mod_prev) != previous.least_representative {
        return Err(AffineError::Overflow);
    }
    // Monotonicity property: R_{next} >= R_{prev}
    if next.least_representative < previous.least_representative {
        return Err(AffineError::Overflow);
    }
    let diff = &next.least_representative - &previous.least_representative;
    let block_val = diff >> (previous.precision as usize);
    Ok(LiftBlock {
        from_precision: previous.precision,
        to_precision: next.precision,
        value: block_val,
    })
}

/// Detects if a finite residue sequence ends in a constant suffix of length >= 2.
pub fn detect_constant_suffix(sequence: &[BigUint]) -> Option<(usize, BigUint)> {
    if sequence.is_empty() {
        return None;
    }
    let last = sequence.last().unwrap();
    let start_idx = sequence
        .iter()
        .rposition(|r| r != last)
        .map(|i| i + 1)
        .unwrap_or(0);
    if sequence.len() - start_idx >= 2 {
        Some((start_idx, last.clone()))
    } else {
        None
    }
}

/// Verifies a stabilization certificate over a finite prefix.
pub fn verify_stabilization_certificate(
    sequence: &[BigUint],
    cert: &StabilizationCertificate,
) -> bool {
    if cert.stabilization_stage >= sequence.len() {
        return false;
    }
    for r in &sequence[cert.stabilization_stage..] {
        if r != &cert.stabilized_value {
            return false;
        }
    }
    true
}
