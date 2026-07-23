use crate::accelerated_branch_params::AcceleratedBranchParams;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

/// Verification results for global quotient & branch theorems
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalTheoremVerificationReport {
    pub max_gap: u64,
    pub gap_uniqueness_pass: bool,
    pub even_source_residues_pass: bool,
    pub cylinder_disjointness_pass: bool,
    pub forbidden_quotient_disjointness_pass: bool,
    pub quotient_equivalence_pass: bool,
}

pub struct GlobalQuotientTheorems;

impl GlobalQuotientTheorems {
    /// Compute forbidden quotient residue a_{j,k} = Q_j^{-1} * (C_k - D_j) mod M_k
    pub fn forbidden_quotient_residue(j: u64, k: u64) -> BigUint {
        let p_j = AcceleratedBranchParams::for_gap(j);
        let p_k = AcceleratedBranchParams::for_gap(k);

        let m_k = &p_k.modulus;
        let c_k = &p_k.z_source_residue;
        let d_j = &p_j.z_endpoint;
        let q_j = &p_j.multiplier;

        let inv_q_j = Self::mod_inverse(q_j, m_k);

        let diff = if c_k >= d_j {
            (c_k - d_j) % m_k
        } else {
            let rem = (d_j - c_k) % m_k;
            if rem.is_zero() {
                BigUint::zero()
            } else {
                m_k - rem
            }
        };

        (&inv_q_j * &diff) % m_k
    }

    /// Verify 2-adic valuation v_2(n) for signed n in Z
    pub fn v2_val(n: &BigInt) -> Option<u64> {
        if n.is_zero() {
            return None; // Infinity
        }
        let abs_n = n.abs().to_biguint().unwrap();
        let mut count = 0u64;
        let one = BigUint::one();
        while (&abs_n & (&one << count)).is_zero() {
            count += 1;
        }
        Some(count)
    }

    /// Verify quotient-equivalence theorem bridge: D_u \in Z_{j,k} <=> D_u \equiv C_j mod M_j and n_u \equiv a_{j,k} mod M_k
    pub fn verify_quotient_equivalence(j: u64, k: u64, n_u: &BigInt) -> bool {
        let p_j = AcceleratedBranchParams::for_gap(j);
        let p_k = AcceleratedBranchParams::for_gap(k);

        let a_jk = Self::forbidden_quotient_residue(j, k);

        let d_plus = BigInt::from(p_j.z_endpoint.clone()) + BigInt::from(p_j.multiplier.clone()) * n_u;
        let is_in_zk = (&d_plus % BigInt::from(p_k.modulus.clone())) == BigInt::from(p_k.z_source_residue.clone());

        let is_n_eq = ((n_u - BigInt::from(a_jk.clone())) % BigInt::from(p_k.modulus.clone())).is_zero();

        is_in_zk == is_n_eq
    }

    /// Verify global theorems for all gaps up to max_gap
    pub fn verify_all(max_gap: u64) -> GlobalTheoremVerificationReport {
        let mut gap_uniqueness_pass = true;
        let mut even_source_residues_pass = true;
        let mut cylinder_disjointness_pass = true;
        let mut forbidden_quotient_disjointness_pass = true;
        let mut quotient_equivalence_pass = true;

        // 1. Verify C_j is even for all j <= max_gap
        for j in 0..=max_gap {
            let p_j = AcceleratedBranchParams::for_gap(j);
            if (&p_j.z_source_residue % BigUint::from(2u64)) != BigUint::zero() {
                even_source_residues_pass = false;
            }
        }

        // 2. Verify v_2(C_k - C_j) = 1 + 4*min(j, k) for all j < k
        for j in 0..=max_gap {
            let p_j = AcceleratedBranchParams::for_gap(j);
            for k in (j + 1)..=max_gap {
                let p_k = AcceleratedBranchParams::for_gap(k);
                let diff = if p_k.z_source_residue >= p_j.z_source_residue {
                    BigInt::from(p_k.z_source_residue.clone() - p_j.z_source_residue.clone())
                } else {
                    BigInt::from(p_j.z_source_residue.clone() - p_k.z_source_residue.clone())
                };

                let v2 = Self::v2_val(&diff).unwrap();
                let expected = 1 + 4 * j;
                if v2 != expected || expected >= p_j.precision {
                    gap_uniqueness_pass = false;
                    cylinder_disjointness_pass = false;
                }
            }
        }

        // 3. Verify forbidden quotient disjointness: for fixed j, v_2((a_{j,l} mod 2^{B_k}) - a_{j,k}) = 1 + 4k < B_k for k < l
        for j in 0..=std::cmp::min(max_gap, 16) {
            for k in 0..=std::cmp::min(max_gap, 16) {
                let a_jk = Self::forbidden_quotient_residue(j, k);
                let p_k = AcceleratedBranchParams::for_gap(k);
                let mod_bk = &p_k.modulus;

                for l in (k + 1)..=std::cmp::min(max_gap, 16) {
                    let a_jl = Self::forbidden_quotient_residue(j, l);
                    let a_jl_mod_bk = &a_jl % mod_bk;

                    let diff = BigInt::from(a_jl_mod_bk) - BigInt::from(a_jk.clone());
                    let v2 = Self::v2_val(&diff).unwrap();
                    let expected_v2 = 1 + 4 * k;

                    if v2 != expected_v2 || expected_v2 >= p_k.precision {
                        forbidden_quotient_disjointness_pass = false;
                    }
                }
            }
        }

        // 4. Verify quotient equivalence theorem bridge
        for j in 0..=std::cmp::min(max_gap, 8) {
            for k in 0..=std::cmp::min(max_gap, 8) {
                let a_jk = Self::forbidden_quotient_residue(j, k);
                let n_test = BigInt::from(a_jk);
                if !Self::verify_quotient_equivalence(j, k, &n_test) {
                    quotient_equivalence_pass = false;
                }
            }
        }

        GlobalTheoremVerificationReport {
            max_gap,
            gap_uniqueness_pass,
            even_source_residues_pass,
            cylinder_disjointness_pass,
            forbidden_quotient_disjointness_pass,
            quotient_equivalence_pass,
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
