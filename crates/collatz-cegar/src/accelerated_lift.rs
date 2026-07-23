use crate::accelerated_transition_system::AcceleratedTransitionSystemEngine;
use crate::induced_v_map::InducedVMapEngine;
use num_bigint::BigUint;

/// Accumulated prefix state for an accelerated sequence of blocks B_j = v u^j.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceleratedPrefixState {
    pub total_precision: u64,
    pub odd_multiplier: BigUint,
    pub source_residue: BigUint,
    pub endpoint_z: BigUint,
}

/// Result of extending an accelerated prefix state by block B_j = v u^j.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceleratedLiftExtension {
    pub child_state: AcceleratedPrefixState,
    pub emitted_block_lift_digit: BigUint,
    pub is_zero_lift: bool,
}

/// Accelerated source-lift digit engine.
pub struct AcceleratedLiftEngine;

impl AcceleratedLiftEngine {
    /// Initial empty prefix state at z = 0.
    pub fn initial_state() -> AcceleratedPrefixState {
        AcceleratedPrefixState {
            total_precision: 0,
            odd_multiplier: BigUint::from(1u32),
            source_residue: BigUint::from(0u32),
            endpoint_z: BigUint::from(0u32),
        }
    }

    /// Evaluates exact block lift digit \Lambda_j(s) \equiv (C_j - y_s) (A_s^{odd})^{-1} \pmod{M_j}.
    pub fn extend(
        prefix: &AcceleratedPrefixState,
        j: u64,
    ) -> Result<AcceleratedLiftExtension, String> {
        let branch_j = InducedVMapEngine::get_branch_normal_form(j)?;
        let m_j = branch_j.modulus_c.clone();
        let k_exp = m_j.trailing_zeros().unwrap_or(0) as u32;

        let a_odd_inv = AcceleratedTransitionSystemEngine::mod_inverse_pow2(&prefix.odd_multiplier, k_exp);

        let diff = if branch_j.c_j_normalized >= prefix.endpoint_z {
            (&branch_j.c_j_normalized - &prefix.endpoint_z) % &m_j
        } else {
            let mod_diff = (&prefix.endpoint_z - &branch_j.c_j_normalized) % &m_j;
            if mod_diff == BigUint::from(0u32) {
                BigUint::from(0u32)
            } else {
                &m_j - mod_diff
            }
        };

        let lambda_j = (&diff * &a_odd_inv) % &m_j;
        let is_zero = lambda_j == BigUint::from(0u32);

        let pow_prec = BigUint::from(1u32) << prefix.total_precision;
        let child_source = &prefix.source_residue + (&lambda_j * &pow_prec);
        let child_prec = prefix.total_precision + 9 + 4 * j;
        let child_odd_mult = &prefix.odd_multiplier * &branch_j.multiplier_d;

        // Evaluate exact shifted child z-endpoint using y_tilde = y_s + A_s^{odd} * \Lambda_j(s)
        let y_tilde = &prefix.endpoint_z + (&prefix.odd_multiplier * &lambda_j);
        if y_tilde < branch_j.c_j_normalized {
            return Err("Shifted endpoint y_tilde < C_j".to_string());
        }
        let q_num = &y_tilde - &branch_j.c_j_normalized;
        if &q_num % &m_j != BigUint::from(0u32) {
            return Err("Exact division failed in child_z endpoint computation".to_string());
        }
        let q = &q_num / &m_j;
        let child_z = &branch_j.d_j_normalized + (&branch_j.multiplier_d * &q);

        Ok(AcceleratedLiftExtension {
            child_state: AcceleratedPrefixState {
                total_precision: child_prec,
                odd_multiplier: child_odd_mult,
                source_residue: child_source,
                endpoint_z: child_z,
            },
            emitted_block_lift_digit: lambda_j,
            is_zero_lift: is_zero,
        })
    }
}
