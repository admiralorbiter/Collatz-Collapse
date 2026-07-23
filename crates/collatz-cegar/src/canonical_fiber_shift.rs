use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::coupled_invariant_miner::CoupledInvariantMiner;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;

pub struct CanonicalFiberShiftEngine;

impl CanonicalFiberShiftEngine {
    /// Compute modular inverse of odd a modulo m = 2^b
    fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let mut t = num_bigint::BigInt::zero();
        let mut newt = num_bigint::BigInt::from(1i64);
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

        if t < num_bigint::BigInt::zero() {
            t += num_bigint::BigInt::from(m.clone());
        }

        t.to_biguint().unwrap()
    }

    /// Compute 2-adic fiber-shift factorization:
    /// Given D, Q, appended gap h:
    /// r \equiv Q^{-1} (C_h - D) \pmod{M_h}
    /// t = (D + Q r - C_h) / M_h
    /// D' = D_h + Q_h * t
    pub fn compute_fiber_shift(
        endpoint_d: &BigInt,
        multiplier_q: &BigUint,
        appended_gap_h: u64,
    ) -> (BigInt, BigInt, BigInt) {
        let p_h = AcceleratedBranchParams::for_gap(appended_gap_h);
        let m_h = p_h.modulus.clone();
        let c_h = p_h.z_source_residue.clone();

        let q_inv = Self::mod_inverse(multiplier_q, &m_h);

        let d_biguint = endpoint_d.to_biguint().unwrap_or_default();
        let d_mod_m = &d_biguint % &m_h;

        let diff = if c_h >= d_mod_m {
            &c_h - &d_mod_m
        } else {
            (&c_h + &m_h) - &d_mod_m
        };

        let r = (&q_inv * &diff) % &m_h;
        let r_bigint = BigInt::from(r.clone());

        let q_bigint = BigInt::from(multiplier_q.clone());
        let c_h_bigint = BigInt::from(c_h);
        let m_h_bigint = BigInt::from(m_h);

        let num_t = endpoint_d + &q_bigint * &r_bigint - &c_h_bigint;
        let t = num_t / &m_h_bigint;

        // Compute direct extension D'
        let (d_next_biguint, _) = CoupledInvariantMiner::canonical_extension(
            &d_biguint,
            multiplier_q,
            appended_gap_h,
        );
        let d_prime = BigInt::from(d_next_biguint);

        (r_bigint, t, d_prime)
    }
}
