use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};

/// Type-safe wrapper for an ordinary odd Syracuse integer n.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrdinaryOdd(BigUint);

impl OrdinaryOdd {
    pub fn new(n: BigUint) -> Result<Self, String> {
        if n == BigUint::ZERO || (&n % 2u32) == BigUint::ZERO {
            Err(format!("Integer {} is not odd and positive", n))
        } else {
            Ok(Self(n))
        }
    }

    pub fn value(&self) -> &BigUint {
        &self.0
    }

    pub fn section_residue_mod32(&self) -> u32 {
        (&self.0 % 32u32).to_u32_digits().first().copied().unwrap_or(0)
    }
}

/// Type-safe wrapper for a Syracuse valuation word w = (a_1, ..., a_k).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuationWord(Vec<u32>);

impl ValuationWord {
    pub fn new(exponents: Vec<u32>) -> Result<Self, String> {
        for &exp in &exponents {
            if exp == 0 {
                return Err("Valuation exponent must be >= 1".to_string());
            }
        }
        Ok(Self(exponents))
    }

    pub fn exponents(&self) -> &[u32] {
        &self.0
    }

    pub fn k_steps(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn total_exponent_b(&self) -> u32 {
        self.0.iter().sum()
    }
}

/// General Quotient Register state with quotient and residue mod 32.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotientRegisterState {
    quotient: BigInt,
    residue_mod_32: u8,
}

impl QuotientRegisterState {
    pub fn from_ordinary_odd(odd: &OrdinaryOdd) -> Self {
        let n = odd.value();
        let k_val = BigInt::from(n >> 5);
        let res = odd.section_residue_mod32() as u8;
        Self {
            quotient: k_val,
            residue_mod_32: res,
        }
    }

    pub fn quotient(&self) -> &BigInt {
        &self.quotient
    }

    pub fn residue(&self) -> u8 {
        self.residue_mod_32
    }
}

/// Specialized Q1 Register state enforcing n \equiv 7 \pmod{32}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Q1RegisterState(BigInt);

impl Q1RegisterState {
    pub fn from_ordinary_odd(odd: &OrdinaryOdd) -> Result<Self, String> {
        if odd.section_residue_mod32() != 7 {
            Err(format!("Odd integer {} is not in Q1 section (residue != 7)", odd.value()))
        } else {
            let k_val = BigInt::from(odd.value() >> 5);
            Ok(Self(k_val))
        }
    }

    pub fn quotient(&self) -> &BigInt {
        &self.0
    }
}

/// Structured failure reasons during j=0 Q1 certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum J0CertificationError {
    RefinedCylinderMismatch { expected: BigUint, actual: BigUint },
    ExactWordMismatch { expected: Vec<u32>, actual: Vec<u32> },
    DestinationPhaseMismatch { actual: u8 },
    LiveAffineMismatch { lhs: BigInt, rhs: BigInt },
    CanonicalAffineMismatch { lhs: BigInt, rhs: BigInt },
    InvalidInput { message: String },
}

impl std::fmt::Display for J0CertificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RefinedCylinderMismatch { expected, actual } => {
                write!(f, "Refined cylinder mismatch: expected {}, got {}", expected, actual)
            }
            Self::ExactWordMismatch { expected, actual } => {
                write!(f, "Exact word mismatch: expected {:?}, got {:?}", expected, actual)
            }
            Self::DestinationPhaseMismatch { actual } => {
                write!(f, "Destination phase mismatch: expected 7 mod 32, got {}", actual)
            }
            Self::LiveAffineMismatch { lhs, rhs } => {
                write!(f, "Live affine mismatch: {} != {}", lhs, rhs)
            }
            Self::CanonicalAffineMismatch { lhs, rhs } => {
                write!(f, "Canonical affine mismatch: {} != {}", lhs, rhs)
            }
            Self::InvalidInput { message } => write!(f, "Invalid input: {}", message),
        }
    }
}

impl std::error::Error for J0CertificationError {}

/// Type-safe wrapper for Live Syracuse Block shift \eta_w.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveBlockConstant(pub BigInt);

/// Type-safe wrapper for Canonical Source Coordinate C(n).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceCoordinate(pub BigInt);

/// Type-safe wrapper for Canonical Endpoint Coordinate D(n').
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalEndpointCoordinate(pub BigInt);

/// Type-safe wrapper for Branch Source Anchor C_j.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchSourceAnchor(pub BigInt);

/// Type-safe wrapper for Branch Endpoint Anchor D_j.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchEndpointAnchor(pub BigInt);

/// Type-safe wrapper for Core Affine shift \beta_j.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreAffineConstant(pub BigInt);
