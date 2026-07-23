use crate::accelerated_branch_params::AcceleratedBranchParams;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

/// Exact Odd-Denominator 2-Adic Rational Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OddRational2Adic {
    pub numerator: BigInt,
    pub denominator: BigUint, // odd and positive
}

impl OddRational2Adic {
    pub fn c_infinity() -> Self {
        Self {
            numerator: BigInt::from(-320i64),
            denominator: BigUint::from(2673u64),
        }
    }
}

/// Result of Global First-Zero Guard Classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalGuardResult {
    NoFirstZero,
    FirstZeroGuardFound {
        gap_j: u64,
        precision_b_j: u64,
        source_residue: BigUint,
    },
}

/// Result of Global Forbidden Quotient Shell Classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForbiddenShellResult {
    RootEqualitySafe,
    SafeNotOnSpine,
    SafeShellSignatureMismatch {
        t_valuation: u64,
        derived_gap_k: u64,
        actual_byte: u8,
        expected_byte: u8,
    },
    ForbiddenMatch {
        derived_gap_k: u64,
        precision_b_k: u64,
    },
}

/// Explicit 2-Adic Valuation Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwoAdicValuation {
    Finite(u64),
    Infinite,
}

pub struct SpineQuotientOracle;

impl SpineQuotientOracle {
    /// Compute 64-period normalized shell signature byte for index idx = (j + k) mod 64
    /// s_{j,k} \equiv (11 * 3^{3(j+k)+8})^{-1} \pmod{256}
    pub fn shell_signature_byte(idx: u64) -> u8 {
        let period_idx = idx % 64;
        let p_3 = BigUint::from(3u64).pow((3 * period_idx + 8) as u32);
        let denom = BigUint::from(11u64) * p_3;
        let mod256 = BigUint::from(256u64);

        let inv = Self::mod_inverse(&denom, &mod256);
        inv.to_u64_digits().first().cloned().unwrap_or(0) as u8
    }

    /// Compute 64-period normalized source signature byte for index j mod 64
    /// s_j^{source} \equiv (11 * 3^{3j+2})^{-1} \pmod{256}
    pub fn source_signature_byte(j: u64) -> u8 {
        let period_idx = j % 64;
        let p_3 = BigUint::from(3u64).pow((3 * period_idx + 2) as u32);
        let denom = BigUint::from(11u64) * p_3;
        let mod256 = BigUint::from(256u64);

        let inv = Self::mod_inverse(&denom, &mod256);
        inv.to_u64_digits().first().cloned().unwrap_or(0) as u8
    }

    /// Compute 2-adic valuation v_2(n) returning TwoAdicValuation
    pub fn v2_valuation(n: &BigInt) -> TwoAdicValuation {
        if n.is_zero() {
            TwoAdicValuation::Infinite
        } else {
            let abs_n = n.abs().to_biguint().unwrap();
            let mut count = 0u64;
            let one = BigUint::one();
            while (&abs_n & (&one << count)).is_zero() {
                count += 1;
            }
            TwoAdicValuation::Finite(count)
        }
    }

    /// Compute 2-adic valuation v_2(n) for signed n in Z
    pub fn v2_val(n: &BigInt) -> Option<u64> {
        match Self::v2_valuation(n) {
            TwoAdicValuation::Finite(v) => Some(v),
            TwoAdicValuation::Infinite => None,
        }
    }

    /// Global First-Zero Guard Oracle: Determine if endpoint D is in any cylinder [C_j]_{B_j} for any j >= 0
    pub fn classify_global_zero_guard(endpoint: &BigInt) -> GlobalGuardResult {
        // v_2(D - C_\infty) = v_2(2673 * D + 320)
        let num_diff = BigInt::from(2673u64) * endpoint + BigInt::from(320u64);
        let t = match Self::v2_val(&num_diff) {
            Some(v) => v,
            None => return GlobalGuardResult::NoFirstZero,
        };

        if (t % 4) != 1 {
            return GlobalGuardResult::NoFirstZero;
        }

        let j = (t - 1) / 4;
        let p_j = AcceleratedBranchParams::for_gap(j);

        // Check normalized byte: (2673 * D + 320) / (2^t * 2673) mod 256
        let shift_val = num_diff >> t;
        let inv_2673 = Self::mod_inverse(&BigUint::from(2673u64), &BigUint::from(256u64));

        let abs_shift = shift_val.abs().to_biguint().unwrap();
        let shift_mod256 = &abs_shift % BigUint::from(256u64);
        let actual_byte = ((&shift_mod256 * &inv_2673) % BigUint::from(256u64)).to_u64_digits().first().cloned().unwrap_or(0) as u8;

        let expected_byte = Self::source_signature_byte(j);

        if actual_byte == expected_byte {
            GlobalGuardResult::FirstZeroGuardFound {
                gap_j: j,
                precision_b_j: p_j.precision,
                source_residue: p_j.z_source_residue,
            }
        } else {
            GlobalGuardResult::NoFirstZero
        }
    }

    /// Global Second-Zero Shell Oracle: Determine if quotient n is in any forbidden cylinder [a_{j,k}]_{B_k} for any k >= 0
    pub fn classify_global_forbidden_quotient(quotient: &BigInt, first_gap_j: u64) -> ForbiddenShellResult {
        let p_j = AcceleratedBranchParams::for_gap(first_gap_j);
        let q_j = &p_j.multiplier;
        let d_j = &p_j.z_endpoint;

        // v_2(n - a_{j,\infty}) = v_2(2673 * Q_j * n + 320 + 2673 * D_j)
        let term1 = BigInt::from(2673u64 * q_j) * quotient;
        let term2 = BigInt::from(320u64);
        let term3 = BigInt::from(2673u64 * d_j);
        let num_diff = term1 + term2 + term3;

        let t = match Self::v2_val(&num_diff) {
            Some(v) => v,
            None => return ForbiddenShellResult::RootEqualitySafe,
        };

        if (t % 4) != 1 {
            return ForbiddenShellResult::SafeNotOnSpine;
        }

        let k = (t - 1) / 4;
        let p_k = AcceleratedBranchParams::for_gap(k);

        // Compute normalized byte
        let shift_val = num_diff >> t;
        let denom_big = BigUint::from(2673u64) * q_j;
        let inv_denom = Self::mod_inverse(&denom_big, &BigUint::from(256u64));

        let abs_shift = shift_val.abs().to_biguint().unwrap();
        let shift_mod256 = &abs_shift % BigUint::from(256u64);
        let actual_byte = ((&shift_mod256 * &inv_denom) % BigUint::from(256u64)).to_u64_digits().first().cloned().unwrap_or(0) as u8;

        let expected_byte = Self::shell_signature_byte(first_gap_j + k);

        if actual_byte == expected_byte {
            ForbiddenShellResult::ForbiddenMatch {
                derived_gap_k: k,
                precision_b_k: p_k.precision,
            }
        } else {
            ForbiddenShellResult::SafeShellSignatureMismatch {
                t_valuation: t,
                derived_gap_k: k,
                actual_byte,
                expected_byte,
            }
        }
    }

    /// Generalized Haar Measure Formula: \mu(E_r) = (1/480)^r = 480^{-r}
    pub fn haar_measure_e_r(r: u32) -> f64 {
        (1.0f64 / 480.0f64).powi(r as i32)
    }

    fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let mut t = num_bigint::BigInt::zero();
        let mut newt = num_bigint::BigInt::one();
        let mut r = num_bigint::BigInt::from(m.clone());
        let mut newr = num_bigint::BigInt::from(a.clone());

        while !newr.is_zero() {
            let quotient = &r / &newr;
            let temp_t = t - &quotient * &newt;
            t = newt;
            newt = temp_t;

            let temp_r = r - &quotient * &newr;
            r = newr;
            newr = temp_r;
        }

        if r > num_bigint::BigInt::one() {
            panic!("Not invertible");
        }
        if t < num_bigint::BigInt::zero() {
            t += num_bigint::BigInt::from(m.clone());
        }

        t.to_biguint().unwrap()
    }
}
