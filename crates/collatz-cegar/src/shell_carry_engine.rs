use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::spine_quotient_oracle::{ForbiddenShellResult, GlobalGuardResult, SpineQuotientOracle};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Unified Shell Coordinates Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCoordinates {
    pub quotient_n: BigInt,
    pub shell_unit_y: BigInt,
    pub shell_offset_gamma: BigInt,
    pub shell_carry_z: BigInt,
    pub centered_carry_x: BigInt,
    pub successor: BigInt,
    pub l_successor: BigInt,
}

/// Certificate verifying dual oracle agreement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualOracleCertificate {
    pub first_gap: u64,
    pub quotient_result: ForbiddenShellResult,
    pub successor_result: GlobalGuardResult,
    pub agrees: bool,
}

pub struct ShellCarryEngine;

impl ShellCarryEngine {
    /// Compute exact L(D) = 2673 * D + 320
    pub fn l_val(endpoint: &BigInt) -> BigInt {
        BigInt::from(2673u64) * endpoint + BigInt::from(320u64)
    }

    /// Compute expected u_j = 27^{1-j} mod 256
    pub fn u_j_shell_byte(j: u64) -> u8 {
        let p_27 = BigUint::from(27u64);
        let mod256 = BigUint::from(256u64);

        let inv_27 = Self::mod_inverse(&p_27, &mod256);
        let val = inv_27.modpow(&BigUint::from(j), &mod256);
        let res = (&val * BigUint::from(27u64)) % &mod256;
        res.to_u64_digits().first().cloned().unwrap_or(0) as u8
    }

    /// Compute gamma_j offset for gap j: gamma_j = ( (L(C_j) / 2^{1+4j}) - u_j ) / 256
    pub fn gamma_j_offset(j: u64) -> BigInt {
        let p_j = AcceleratedBranchParams::for_gap(j);
        let c_j_big = BigInt::from(p_j.z_source_residue);
        let l_c_j = Self::l_val(&c_j_big);

        let shift = 1 + 4 * j;
        let y_c_j = l_c_j >> shift;

        let u_j = BigInt::from(Self::u_j_shell_byte(j));
        (y_c_j - u_j) >> 8
    }

    /// Compute exact shell coordinates for endpoint D lying in guard cylinder [C_j]_{B_j}
    pub fn compute_shell_coordinates(endpoint: &BigInt, gap_j: u64) -> Result<ShellCoordinates, String> {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        let c_j_big = BigInt::from(p_j.z_source_residue.clone());
        let m_j_big = BigInt::from(p_j.modulus.clone());

        // Verify D in [C_j]_{B_j}
        let diff_c = endpoint - &c_j_big;
        if (&diff_c % &m_j_big) != BigInt::zero() {
            return Err(format!("Endpoint D does not lie in guard cylinder [C_{}]_{{B_{}}}", gap_j, gap_j));
        }

        let quotient_n = diff_c / &m_j_big;

        let l_d = Self::l_val(endpoint);
        let shift = 1 + 4 * gap_j;
        let shell_unit_y = l_d >> shift;

        let gamma_j = Self::gamma_j_offset(gap_j);
        let centered_carry_x = BigInt::from(2673u64) * &quotient_n;
        let shell_carry_z = &gamma_j + &centered_carry_x;

        let d_j_big = BigInt::from(p_j.z_endpoint.clone());
        let q_j_big = BigInt::from(p_j.multiplier.clone());
        let successor = &d_j_big + &q_j_big * &quotient_n;

        let l_successor = Self::l_val(&successor);

        // Verify Centered Carry Linearization: L(D^+) == L(D_j) + Q_j * X
        let l_d_j = Self::l_val(&d_j_big);
        let expected_l_succ = &l_d_j + &q_j_big * &centered_carry_x;
        assert_eq!(l_successor, expected_l_succ, "Centered carry linearization L(D^+) = L(D_j) + Q_j X failed!");

        Ok(ShellCoordinates {
            quotient_n,
            shell_unit_y,
            shell_offset_gamma: gamma_j,
            shell_carry_z,
            centered_carry_x,
            successor,
            l_successor,
        })
    }

    /// Verify Dual Oracle Certificate: GlobalGuardOracle(D^+) == GlobalForbiddenQuotientOracle(n_u, j)
    pub fn verify_dual_oracle(endpoint: &BigInt, gap_j: u64) -> DualOracleCertificate {
        let coords = Self::compute_shell_coordinates(endpoint, gap_j).unwrap();

        let quotient_result = SpineQuotientOracle::classify_global_forbidden_quotient(&coords.quotient_n, gap_j);
        let successor_result = SpineQuotientOracle::classify_global_zero_guard(&coords.successor);

        let agrees = match (&quotient_result, &successor_result) {
            (ForbiddenShellResult::ForbiddenMatch { derived_gap_k: k1, .. }, GlobalGuardResult::FirstZeroGuardFound { gap_j: k2, .. }) => k1 == k2,
            (ForbiddenShellResult::SafeNotOnSpine, GlobalGuardResult::NoFirstZero) => true,
            (ForbiddenShellResult::SafeShellSignatureMismatch { .. }, GlobalGuardResult::NoFirstZero) => true,
            (ForbiddenShellResult::RootEqualitySafe, GlobalGuardResult::NoFirstZero) => true,
            _ => false,
        };

        DualOracleCertificate {
            first_gap: gap_j,
            quotient_result,
            successor_result,
            agrees,
        }
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
