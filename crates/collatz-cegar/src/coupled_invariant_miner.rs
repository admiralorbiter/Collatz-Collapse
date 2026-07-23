use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::precision_aware_cylinder::Cylinder;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Coupled Abstract Residue State (D_u, Q_u) mod 2^m
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoupledResidueState {
    pub endpoint_residue: BigUint,
    pub multiplier_residue: BigUint,
    pub precision: u64,
}

/// Outcome of Candidate Invariant Mining & Inductiveness Audit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantAuditOutcome {
    NoSeparationFound {
        tested_modulus_exponent: u64,
        reason: String,
    },
    CoupledCandidateFound {
        precision_m: u64,
        reachable_states_count: usize,
        is_inductive: bool,
        is_dangerous_disjoint: bool,
    },
}

/// Phase 7.3S.3B: State-Coupled Invariant Miner Engine
#[derive(Debug, Clone)]
pub struct CoupledInvariantMiner {
    pub max_prefix_depth: usize,
    pub max_gap_bound: u64,
}

impl CoupledInvariantMiner {
    pub fn new(max_prefix_depth: usize, max_gap_bound: u64) -> Self {
        Self {
            max_prefix_depth,
            max_gap_bound,
        }
    }

    /// Compute exact canonical extension step for state (D_u, Q_u) appended with gap h:
    /// r \equiv Q_u^{-1} * (C_h - D_u) \pmod{M_h}
    /// D_{uh} = D_h + Q_h * M_h * D_u + Q_u * r - C_h
    /// Q_{uh} = Q_h * Q_u
    pub fn canonical_extension(d_u: &BigUint, q_u: &BigUint, h: u64) -> (BigUint, BigUint) {
        let p_h = AcceleratedBranchParams::for_gap(h);
        let m_h = &p_h.modulus;
        let q_h = &p_h.multiplier;
        let c_h = &p_h.z_source_residue;

        // Compute r \equiv Q_u^{-1} * (C_h - D_u) \pmod{M_h}
        let inv_q_u = Self::mod_inverse(q_u, m_h);
        let diff = if c_h >= d_u {
            (c_h - d_u) % m_h
        } else {
            let rem = (d_u - c_h) % m_h;
            if rem.is_zero() {
                BigUint::zero()
            } else {
                m_h - rem
            }
        };
        let r = (&inv_q_u * &diff) % m_h;

        // Permanent assertion: Q_u * r \equiv C_h - D_u \pmod{M_h}
        let check_rem = (q_u * &r) % m_h;
        assert_eq!(check_rem, diff, "Modular inverse assertion Q_u * r == C_h - D_u mod M_h failed!");

        // D_{uh} = (Q_h * (D_u + Q_u * r) + beta_h) / M_h
        let term_d_qr = d_u + q_u * &r;
        let q_term = q_h * term_d_qr;
        let num_big = BigInt::from(q_term) + &p_h.affine_intercept;
        let m_big = BigInt::from(m_h.clone());

        let (quot, rem) = (&num_big / &m_big, &num_big % &m_big);
        assert_eq!(rem, BigInt::zero(), "Divisibility assertion M_h | (Q_h (D_u + Q_u r) + beta_h) failed!");

        let d_uh = quot.to_biguint().unwrap();
        let q_uh = q_h * q_u;
        (d_uh, q_uh)
    }

    /// Mine reachable coupled states (D_u mod 2^m, Q_u mod 2^m) across canonical prefixes
    pub fn mine_reachable_coupled_states(&self, precision_m: u64) -> HashSet<CoupledResidueState> {
        let mod_val = BigUint::one() << precision_m;
        let mut states = HashSet::new();

        // Level 1: 1-symbol canonical words j
        let mut current_level: Vec<(BigUint, BigUint)> = Vec::new();
        for j in 0..=self.max_gap_bound {
            let p_j = AcceleratedBranchParams::for_gap(j);
            let d_j = p_j.z_endpoint.clone();
            let q_j = p_j.multiplier.clone();

            states.insert(CoupledResidueState {
                endpoint_residue: &d_j % &mod_val,
                multiplier_residue: &q_j % &mod_val,
                precision: precision_m,
            });
            current_level.push((d_j, q_j));
        }

        // Levels 2..=max_prefix_depth
        for _depth in 2..=self.max_prefix_depth {
            let mut next_level = Vec::new();
            for (d_u, q_u) in &current_level {
                for h in 0..=self.max_gap_bound {
                    let (d_uh, q_uh) = Self::canonical_extension(d_u, q_u, h);
                    states.insert(CoupledResidueState {
                        endpoint_residue: &d_uh % &mod_val,
                        multiplier_residue: &q_uh % &mod_val,
                        precision: precision_m,
                    });
                    next_level.push((d_uh, q_uh));
                }
            }
            current_level = next_level;
        }

        states
    }

    /// Check dangerous set disjointness against full-precision E_2^J two-zero cylinders
    pub fn verify_dangerous_disjointness(
        &self,
        reachable_states: &HashSet<CoupledResidueState>,
        dangerous_cylinders: &[((u64, u64), Cylinder)],
        precision_m: u64,
    ) -> bool {
        let mod_val = BigUint::one() << precision_m;

        for state in reachable_states {
            for (_pair, cyl) in dangerous_cylinders {
                let cyl_mod = BigUint::one() << cyl.precision;
                let gcd_m = Self::gcd(&mod_val, &cyl_mod);

                if (&state.endpoint_residue % &gcd_m) == (&cyl.residue % &gcd_m) {
                    return false; // Intersects dangerous cylinder!
                }
            }
        }

        true
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

    fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
        let mut x = a.clone();
        let mut y = b.clone();
        while !y.is_zero() {
            let rem = &x % &y;
            x = y;
            y = rem;
        }
        x
    }
}
