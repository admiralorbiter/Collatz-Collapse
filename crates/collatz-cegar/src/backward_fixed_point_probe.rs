use crate::precision_aware_cylinder::Cylinder;

/// Phase 7.3S.2A.1: Backward Zero-Lift Fixed-Point Probe & Predecessor Set Generator.
#[derive(Debug, Clone)]
pub struct BackwardFixedPointProbe {
    pub max_gap: u64,
}

impl BackwardFixedPointProbe {
    pub fn new(max_gap: u64) -> Self {
        Self { max_gap }
    }

    /// Compute 1-step backward predecessor set E_{n+1} = Phi_J(E_n)
    pub fn step_backward(&self, target_set: &[Cylinder]) -> Vec<Cylinder> {
        let mut preds = Vec::new();
        for target in target_set {
            for j in 0..=self.max_gap {
                let pred = Cylinder::pre_j(target, j);
                preds.push(pred);
            }
        }
        preds
    }
}
