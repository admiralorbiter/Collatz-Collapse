use crate::accelerated_branch_params::AcceleratedBranchParams;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;

/// Phase 7.3S.1C.0: Eventual-Zero Endpoint Reduction Graph Engine.
///
/// Theorem: Lift block Lambda_{u, j} == 0 iff D_u == C_j (mod M_j).
/// When Lambda_{u, j} == 0, the next endpoint is D_{u j} = D_j + Q_j * ((D_u - C_j) / M_j) = F_j(D_u).
#[derive(Debug, Clone)]
pub struct ZeroLiftEndpointGraph {
    pub max_gap: u64,
}

impl ZeroLiftEndpointGraph {
    pub fn new(max_gap: u64) -> Self {
        Self { max_gap }
    }

    /// Check if endpoint D_u produces zero lift block when extended by gap j.
    pub fn is_zero_lift(&self, endpoint_d_u: &BigUint, gap_j: u64) -> bool {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        (endpoint_d_u % &p_j.modulus) == (p_j.z_source_residue % &p_j.modulus)
    }

    /// Compute exact zero-lift endpoint successor D_{u j} = F_j(D_u).
    pub fn zero_lift_successor(&self, endpoint_d_u: &BigUint, gap_j: u64) -> BigUint {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        assert!(
            self.is_zero_lift(endpoint_d_u, gap_j),
            "Endpoint D_u is not congruent to C_j mod M_j"
        );
        let num = BigInt::from(p_j.multiplier.clone()) * BigInt::from(endpoint_d_u.clone()) + &p_j.affine_intercept;
        let m_big = BigInt::from(p_j.modulus.clone());
        assert_eq!(&num % &m_big, BigInt::zero(), "F_j(D_u) non-integer division");
        (num / m_big).to_biguint().unwrap()
    }
}
