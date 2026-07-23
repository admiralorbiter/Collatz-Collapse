use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::precision_aware_cylinder::Cylinder;
use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

/// Detailed Zero-Lift Transition Status for an Endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoubleZeroStatus {
    NoZeroLift,
    ExactlyOneZeroLift {
        first_gap: u64,
        endpoint_after_first: String,
    },
    AtLeastTwoZeroLifts {
        first_gap: u64,
        second_gap: u64,
        endpoint_after_first: String,
        endpoint_after_second: String,
    },
}

/// Phase 7.3S.2C.0: Two-Zero Cylinder Characterization Theorem & Gap Uniqueness Module.
#[derive(Debug, Clone)]
pub struct TwoZeroCylinderCharacterization {
    pub max_gap: u64,
}

impl TwoZeroCylinderCharacterization {
    pub fn new(max_gap: u64) -> Self {
        Self { max_gap }
    }

    /// Theorem: Verify [C_j]_{B_j} \cap [C_k]_{B_k} = \emptyset for all 0 <= j != k <= max_gap
    pub fn verify_gap_uniqueness(&self) -> bool {
        for j in 0..=self.max_gap {
            let p_j = AcceleratedBranchParams::for_gap(j);
            for k in (j + 1)..=self.max_gap {
                let p_k = AcceleratedBranchParams::for_gap(k);
                let gcd_m = Self::gcd(&p_j.modulus, &p_k.modulus);
                if (&p_j.z_source_residue % &gcd_m) == (&p_k.z_source_residue % &gcd_m) {
                    return false; // Overlap detected!
                }
            }
        }
        true
    }

    /// Construct exact (max_gap + 1)^2 Two-Zero Cylinders Z_{j,k} = Pre_j([C_k]_{B_k})
    pub fn generate_two_zero_cylinders(&self) -> Vec<((u64, u64), Cylinder)> {
        let mut cylinders = Vec::new();
        for j in 0..=self.max_gap {
            for k in 0..=self.max_gap {
                let p_k = AcceleratedBranchParams::for_gap(k);
                let target_k = Cylinder::new(p_k.z_source_residue.clone(), p_k.precision as u32);
                let pred_jk = Cylinder::pre_j(&target_k, j);
                cylinders.push(((j, k), pred_jk));
            }
        }
        cylinders
    }

    /// Evaluate DoubleZeroStatus for any concrete endpoint D
    pub fn evaluate_endpoint_status(&self, endpoint_d: &BigUint) -> DoubleZeroStatus {
        // Find unique first zero-lift gap j
        let first_match = (0..=self.max_gap).find(|&j| {
            let p_j = AcceleratedBranchParams::for_gap(j);
            (endpoint_d % &p_j.modulus) == (p_j.z_source_residue % &p_j.modulus)
        });

        match first_match {
            None => DoubleZeroStatus::NoZeroLift,
            Some(j) => {
                let p_j = AcceleratedBranchParams::for_gap(j);
                let d_1 = p_j.direct_original_gap_return(endpoint_d);

                // Find unique second zero-lift gap k
                let second_match = (0..=self.max_gap).find(|&k| {
                    let p_k = AcceleratedBranchParams::for_gap(k);
                    (&d_1 % &p_k.modulus) == (p_k.z_source_residue % &p_k.modulus)
                });

                match second_match {
                    None => DoubleZeroStatus::ExactlyOneZeroLift {
                        first_gap: j,
                        endpoint_after_first: d_1.to_string(),
                    },
                    Some(k) => {
                        let p_k = AcceleratedBranchParams::for_gap(k);
                        let d_2 = p_k.direct_original_gap_return(&d_1);
                        DoubleZeroStatus::AtLeastTwoZeroLifts {
                            first_gap: j,
                            second_gap: k,
                            endpoint_after_first: d_1.to_string(),
                            endpoint_after_second: d_2.to_string(),
                        }
                    }
                }
            }
        }
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
