use collatz_affine::AffinePrefix;
use smallvec::SmallVec;

/// Stack-optimized search state for hot evaluation loops.
#[derive(Debug, Clone, PartialEq)]
pub struct PrefixState {
    pub valuations: SmallVec<[u8; 64]>,
    pub affine: AffinePrefix,
    pub growth_debt: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RejectionReason {
    DescentCertified,
    ExceedsMinimalCounterexampleBound { step_index: usize, bound: String },
    Mod9TernaryContradiction { modulus_2: u64, residue_2: u64 },
    PathSubsumed { target_valuation: Vec<u32> },
    OddEvenEvenContradiction,
    TwoAdicImpostor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SieveResult {
    Keep,
    Reject { reason: RejectionReason },
    Refine { requested_precision: u32 },
}

pub trait PrefixSieve: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, state: &PrefixState) -> SieveResult;
}
