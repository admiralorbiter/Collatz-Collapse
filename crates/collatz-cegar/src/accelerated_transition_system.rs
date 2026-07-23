use crate::induced_v_map::InducedVMapEngine;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::Signed;

/// Complete edge normal form for an accelerated transition j -> j_next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceleratedEdge {
    pub source_gap: u64,
    pub target_gap: u64,
    pub source_parameter_residue: BigUint, // R_{j,j'}
    pub source_parameter_modulus: BigUint, // M_{j'}
    pub target_parameter_offset: BigInt,   // S_{j,j'}
    pub target_parameter_multiplier: BigUint, // Q_j
    pub minimum_free_parameter: BigUint,   // max(0, ceil(-S / Q))
}

/// Accelerated dyadic branch transition system.
pub struct AcceleratedTransitionSystemEngine;

impl AcceleratedTransitionSystemEngine {
    /// Computes modular inverse of an odd number a modulo 2^k.
    pub fn mod_inverse_pow2(a: &BigUint, k: u32) -> BigUint {
        let modulus = BigUint::from(1u32) << k;
        let a_mod = a % &modulus;
        // Newton-Raphson 2-adic inverse iteration x_{n+1} = x_n * (2 - a * x_n) mod 2^k
        let mut x = BigUint::from(1u32);
        for _ in 0..(k + 5) {
            let term = &modulus + BigUint::from(2u32) - ((&a_mod * &x) % &modulus);
            x = (&x * &term) % &modulus;
        }
        x
    }

    /// Computes the complete edge normal form for transition j -> j_next.
    pub fn compute_complete_edge(j: u64, j_next: u64) -> Result<AcceleratedEdge, String> {
        let branch_j = InducedVMapEngine::get_branch_normal_form(j)?;
        let branch_j_next = InducedVMapEngine::get_branch_normal_form(j_next)?;

        let m_j_next = branch_j_next.modulus_c.clone();
        let k_exp = m_j_next.trailing_zeros().unwrap_or(0) as u32;

        // Solve Q_j * R \equiv (C_{j_next} - D_j) \pmod{M_{j_next}}
        let q_inv = Self::mod_inverse_pow2(&branch_j.multiplier_d, k_exp);

        let diff = if branch_j_next.c_j_normalized >= branch_j.d_j_normalized {
            (&branch_j_next.c_j_normalized - &branch_j.d_j_normalized) % &m_j_next
        } else {
            let mod_diff = (&branch_j.d_j_normalized - &branch_j_next.c_j_normalized) % &m_j_next;
            if mod_diff == BigUint::from(0u32) {
                BigUint::from(0u32)
            } else {
                &m_j_next - mod_diff
            }
        };

        let r_j_jnext = (&q_inv * &diff) % &m_j_next;

        // Compute S_{j,j'} = (D_j + Q_j * R_{j,j'} - C_{j'}) / M_{j'}
        let d_j_bi = branch_j.d_j_normalized.to_bigint().unwrap();
        let q_j_bi = branch_j.multiplier_d.to_bigint().unwrap();
        let r_bi = r_j_jnext.to_bigint().unwrap();
        let c_next_bi = branch_j_next.c_j_normalized.to_bigint().unwrap();
        let m_next_bi = m_j_next.to_bigint().unwrap();

        let num_s = &d_j_bi + (&q_j_bi * &r_bi) - &c_next_bi;
        if &num_s % &m_next_bi != BigInt::from(0i32) {
            return Err("Exact division failed in S_{j,j'} computation".to_string());
        }
        let s_j_jnext = &num_s / &m_next_bi;

        // Minimum free parameter h >= max(0, ceil(-S / Q))
        let min_h = if s_j_jnext.is_negative() {
            let neg_s = (-&s_j_jnext).to_biguint().unwrap();
            let q_b = &branch_j.multiplier_d;
            let (q_quot, q_rem) = (&neg_s / q_b, &neg_s % q_b);
            if q_rem > BigUint::from(0u32) {
                q_quot + BigUint::from(1u32)
            } else {
                q_quot
            }
        } else {
            BigUint::from(0u32)
        };

        Ok(AcceleratedEdge {
            source_gap: j,
            target_gap: j_next,
            source_parameter_residue: r_j_jnext,
            source_parameter_modulus: m_j_next,
            target_parameter_offset: s_j_jnext,
            target_parameter_multiplier: branch_j.multiplier_d,
            minimum_free_parameter: min_h,
        })
    }

    /// Computes all (max_j + 1)^2 edges for j, j_next in 0..=max_j.
    pub fn build_bounded_transition_system(max_j: u64) -> Result<Vec<AcceleratedEdge>, String> {
        let mut edges = Vec::new();
        for j in 0..=max_j {
            for j_next in 0..=max_j {
                let edge = Self::compute_complete_edge(j, j_next)?;
                edges.push(edge);
            }
        }
        Ok(edges)
    }
}
