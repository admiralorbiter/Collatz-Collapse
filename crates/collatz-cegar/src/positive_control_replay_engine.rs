use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::two_zero_cylinder_characterization::{DoubleZeroStatus, TwoZeroCylinderCharacterization};
use num_bigint::BigUint;

/// Phase 7.3S.2C.0: Positive Control Replay Engine for (0,0,7) and (2,2,8).
#[derive(Debug, Clone)]
pub struct PositiveControlReplayEngine;

impl PositiveControlReplayEngine {
    /// Replay and verify canonical prefix u=(0,0,7)
    pub fn verify_control_0_0_7() -> (BigUint, DoubleZeroStatus, Vec<bool>) {
        let w0 = ExtremalSourceSearchEngine::base_guarded_word(0);
        let w00 = ExtremalSourceSearchEngine::extend_guarded_word(&w0, 0);
        let w007 = ExtremalSourceSearchEngine::extend_guarded_word(&w00, 7);
        let d_u = w007.endpoint.clone();

        let char_engine = TwoZeroCylinderCharacterization::new(32);
        let status = char_engine.evaluate_endpoint_status(&d_u);

        // Verify all 33 second-step guard rejections explicitly
        let p_0 = AcceleratedBranchParams::for_gap(0);
        let d_1 = p_0.direct_original_gap_return(&d_u);
        let mut second_step_rejections = Vec::with_capacity(33);

        for k in 0..=32 {
            let p_k = AcceleratedBranchParams::for_gap(k);
            let is_rejected = (&d_1 % &p_k.modulus) != (p_k.z_source_residue % &p_k.modulus);
            second_step_rejections.push(is_rejected);
        }

        (d_u, status, second_step_rejections)
    }

    /// Replay and verify canonical prefix u=(2,2,8)
    pub fn verify_control_2_2_8() -> (BigUint, DoubleZeroStatus, Vec<bool>) {
        let w2 = ExtremalSourceSearchEngine::base_guarded_word(2);
        let w22 = ExtremalSourceSearchEngine::extend_guarded_word(&w2, 2);
        let w228 = ExtremalSourceSearchEngine::extend_guarded_word(&w22, 8);
        let d_u = w228.endpoint.clone();

        let char_engine = TwoZeroCylinderCharacterization::new(32);
        let status = char_engine.evaluate_endpoint_status(&d_u);

        let p_0 = AcceleratedBranchParams::for_gap(0);
        let d_1 = p_0.direct_original_gap_return(&d_u);
        let mut second_step_rejections = Vec::with_capacity(33);

        for k in 0..=32 {
            let p_k = AcceleratedBranchParams::for_gap(k);
            let is_rejected = (&d_1 % &p_k.modulus) != (p_k.z_source_residue % &p_k.modulus);
            second_step_rejections.push(is_rejected);
        }

        (d_u, status, second_step_rejections)
    }
}
