use crate::induced_v_map::InducedVMapEngine;
use crate::zero_lift_continuation::ZeroLiftContinuationEngine;
use num_bigint::BigUint;

/// Witness object for a canonical zero-lift suffix run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroLiftWitness {
    pub initial_branch_j0: u64,
    pub gap_sequence: Vec<u64>,
    pub constant_source_residue: BigUint,
    pub endpoint_trajectory: Vec<BigUint>,
    pub branch_parameters: Vec<BigUint>,
    pub features_2adic: Vec<u64>,
    pub features_3adic: Vec<u64>,
    pub zero_lift_suffix_length: usize,
    pub termination_reason: String,
}

/// Zero-lift record search explorer.
pub struct ZeroLiftExplorer;

impl ZeroLiftExplorer {
    /// Searches canonical zero-lift suffixes starting from initial branch states j_0 = 0..max_start_j.
    pub fn search_canonical_record_suffixes(
        max_start_j: u64,
        max_steps: usize,
    ) -> Result<Vec<ZeroLiftWitness>, String> {
        let mut witnesses = Vec::new();

        for start_j in 0..=max_start_j {
            let (child, steps) = ZeroLiftContinuationEngine::trace_zero_lift_suffix(start_j, max_steps)?;

            let mut gap_seq = vec![start_j];
            let mut trajectory = vec![child.endpoint_z.clone()];
            let mut residuals = vec![BigUint::from(0u32)];
            let mut feat_2a = vec![child.endpoint_z.trailing_zeros().unwrap_or(0)];
            let mut feat_3a = vec![(&child.endpoint_z % BigUint::from(27u32)).to_u64_digits().first().copied().unwrap_or(0)];

            for st in &steps {
                gap_seq.push(st.j);
                trajectory.push(st.next_endpoint_z.clone());
                residuals.push(st.residual_e.clone());
                feat_2a.push(st.next_endpoint_z.trailing_zeros().unwrap_or(0));
                feat_3a.push((&st.next_endpoint_z % BigUint::from(27u32)).to_u64_digits().first().copied().unwrap_or(0));
            }

            let last_endpoint = trajectory.last().unwrap();
            let failed_reason = format!(
                "Canonical child endpoint D_{} = {} (mod 27 = {}) left all zero-lift branch domains C_j",
                start_j,
                last_endpoint,
                last_endpoint % BigUint::from(27u32)
            );

            witnesses.push(ZeroLiftWitness {
                initial_branch_j0: start_j,
                gap_sequence: gap_seq,
                constant_source_residue: child.source_residue,
                endpoint_trajectory: trajectory,
                branch_parameters: residuals,
                features_2adic: feat_2a,
                features_3adic: feat_3a,
                zero_lift_suffix_length: steps.len(),
                termination_reason: failed_reason,
            });
        }

        Ok(witnesses)
    }

    /// Evaluates concrete orbit trajectory for z_0 = C_{j_0} + M_{j_0} * n_0 over r steps.
    pub fn evaluate_concrete_orbit(
        j_0: u64,
        n_0: BigUint,
        r_steps: usize,
    ) -> Result<Vec<u64>, String> {
        let branch = InducedVMapEngine::get_branch_normal_form(j_0)?;
        let mut current_z = &branch.c_j_normalized + (&branch.modulus_c * &n_0);
        let mut gaps = Vec::new();

        for _ in 0..r_steps {
            if let Some(j) = ZeroLiftContinuationEngine::find_exact_successor(&current_z)? {
                gaps.push(j);
                let branch_j = InducedVMapEngine::get_branch_normal_form(j)?;
                let e = (&current_z - &branch_j.c_j_normalized) / &branch_j.modulus_c;
                current_z = &branch_j.d_j_normalized + (&branch_j.multiplier_d * &e);
            } else {
                break;
            }
        }

        Ok(gaps)
    }
}
