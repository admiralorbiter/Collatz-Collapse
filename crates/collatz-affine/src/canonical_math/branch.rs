use crate::canonical_math::cocycle::{compile_semantic_return, SemanticReturnCompilation};
use crate::canonical_math::types::{
    BranchEndpointAnchor, BranchSourceAnchor, CanonicalEndpointCoordinate,
    CanonicalSourceCoordinate, CoreAffineConstant, J0CertificationError, OrdinaryOdd,
    Q1RegisterState, ValuationWord,
};
use crate::counterexample_capture::OrdinaryToCanonicalPrefixExtractor;
use num_bigint::{BigInt, BigUint};
use num_traits::One;
use serde::{Deserialize, Serialize};

/// Authoritative canonical coordinate chart parameters for a branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCoordinateChart {
    pub source_translation: BigInt,
    pub target_translation: BigInt,
    pub core_shift: CoreAffineConstant,
}

impl CanonicalCoordinateChart {
    pub fn j0_chart() -> Self {
        Self {
            source_translation: BigInt::from(281u32),
            target_translation: BigInt::from(400u32),
            core_shift: CoreAffineConstant(BigInt::from(26u32)),
        }
    }
}

/// Authoritative canonical branch structure for gap j.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalBranch {
    pub gap: u32,
    pub odd_steps: u32,
    pub precision: u32,
    pub modulus: BigUint,
    pub multiplier: BigUint,
    pub source_residue: BigUint,
    pub source_core: BranchSourceAnchor,
    pub endpoint_core: BranchEndpointAnchor,
    pub beta: CoreAffineConstant,
}

/// Returns the exact frozen H-phase canonical branch for gap j.
pub fn canonical_branch(gap: u32) -> Result<CanonicalBranch, String> {
    if gap > 3 {
        return Err(format!("Gap {} exceeds frozen table limit (supported: j <= 3)", gap));
    }

    let odd_steps = 6 + 3 * gap;
    let precision = 9 + 4 * gap;
    let modulus = BigUint::one() << precision;
    let multiplier = BigUint::from(3u32).pow(odd_steps);

    // Exact frozen core coordinates matching frozen H-phase math:
    // j=0: C_0 = 342, D_0 = 487
    // j=1: C_1 = 7392, D_1 = 17761
    // j=2: C_2 = 86208, D_2 = 349537
    // j=3: C_3 = 1764032, D_3 = 12069670
    let (c_val, d_val) = match gap {
        0 => (342u64, 487u64),
        1 => (7392u64, 17761u64),
        2 => (86208u64, 349537u64),
        3 => (1764032u64, 12069670u64),
        _ => unreachable!(),
    };

    let source_core = BranchSourceAnchor(BigInt::from(c_val));
    let endpoint_core = BranchEndpointAnchor(BigInt::from(d_val));
    let source_residue = (BigUint::from(423u32) + (BigUint::from(512u32) * gap)) % &modulus;

    // \beta_j = M_j * D_j - Q_j * C_j
    let beta_val = (BigInt::from(modulus.clone()) * &endpoint_core.0)
        - (BigInt::from(multiplier.clone()) * &source_core.0);
    let beta = CoreAffineConstant(beta_val);

    Ok(CanonicalBranch {
        gap,
        odd_steps,
        precision,
        modulus,
        multiplier,
        source_residue,
        source_core,
        endpoint_core,
        beta,
    })
}

/// Certified j=0 Q1-to-Q1 Canonical Return Certificate.
/// Invariant: Can ONLY be constructed if source passes all semantic return guards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedJ0Q1Return {
    source: OrdinaryOdd,
    target: OrdinaryOdd,
    word: ValuationWord,
    live_source: Q1RegisterState,
    live_target: Q1RegisterState,
    canonical_source: CanonicalSourceCoordinate,
    canonical_target: CanonicalEndpointCoordinate,
    chart: CanonicalCoordinateChart,
}

impl CertifiedJ0Q1Return {
    /// Deterministic source-only constructor enforcing ALL semantic return criteria for j=0.
    pub fn try_from_source(source: &OrdinaryOdd) -> Result<Self, J0CertificationError> {
        let n = source.value();

        // Compile semantic return using generic compiler for w_0 = [1, 1, 2, 1, 2, 2], r_t = 7, q = 5
        let w_0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2])
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;
        let compiled_res = compile_semantic_return(&w_0, 7, 5)
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;

        let compiled = match compiled_res {
            SemanticReturnCompilation::Compatible(c) => c,
            SemanticReturnCompilation::Incompatible { exact_word, .. } => {
                return Err(J0CertificationError::RefinedCylinderMismatch {
                    expected: exact_word.residue,
                    actual: n % &exact_word.modulus,
                });
            }
        };

        let expected_residue = compiled.refined_source_residue;
        let expected_modulus = compiled.refined_source_modulus;

        // 1. Destination-refined return cylinder check (derived dynamically: n \equiv 1959 \pmod{16384})
        let actual_residue = n % &expected_modulus;
        if actual_residue != expected_residue {
            return Err(J0CertificationError::RefinedCylinderMismatch {
                expected: expected_residue,
                actual: actual_residue,
            });
        }

        // 2. Replay exact 6 Syracuse steps deterministically
        let (n1, a1) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(n);
        let (n2, a2) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&n1);
        let (n3, a3) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&n2);
        let (n4, a4) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&n3);
        let (n5, a5) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&n4);
        let (target_val, a6) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&n5);

        let word_exponents = vec![a1, a2, a3, a4, a5, a6];
        if word_exponents != [1, 1, 2, 1, 2, 2] {
            return Err(J0CertificationError::ExactWordMismatch {
                expected: vec![1, 1, 2, 1, 2, 2],
                actual: word_exponents,
            });
        }

        let target = OrdinaryOdd::new(target_val.clone())
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;

        // 3. Destination section guard check (n' \equiv 7 \pmod{32})
        if target.section_residue_mod32() != 7 {
            return Err(J0CertificationError::DestinationPhaseMismatch {
                actual: target.section_residue_mod32() as u8,
            });
        }

        let live_source = Q1RegisterState::from_ordinary_odd(source)
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;
        let live_target = Q1RegisterState::from_ordinary_odd(&target)
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;

        // 4. Live Quotient Intertwining with dynamically derived \eta
        let k_s = live_source.quotient();
        let k_t = live_target.quotient();

        let expected_eta = compiled.live_affine_constant
            .ok_or_else(|| J0CertificationError::InvalidInput { message: "Failed to derive live eta".to_string() })?;

        let lhs_live = BigInt::from(512u32) * k_t;
        let rhs_live = (BigInt::from(729u32) * k_s) + &expected_eta.0;

        if lhs_live != rhs_live {
            return Err(J0CertificationError::LiveAffineMismatch {
                lhs: lhs_live,
                rhs: rhs_live,
            });
        }

        let chart = CanonicalCoordinateChart::j0_chart();

        // 5. Canonical Coordinate Chart: C = k_s + b_s, D = k_t + b_t
        let c_coord = CanonicalSourceCoordinate(k_s + &chart.source_translation);
        let d_coord = CanonicalEndpointCoordinate(k_t + &chart.target_translation);

        // 6. Canonical Intertwining: 512 * D == 729 * C + \beta_0
        let lhs_canon = BigInt::from(512u32) * &d_coord.0;
        let rhs_canon = (BigInt::from(729u32) * &c_coord.0) + &chart.core_shift.0;

        if lhs_canon != rhs_canon {
            return Err(J0CertificationError::CanonicalAffineMismatch {
                lhs: lhs_canon,
                rhs: rhs_canon,
            });
        }

        let word = ValuationWord::new(word_exponents)
            .map_err(|e| J0CertificationError::InvalidInput { message: e })?;

        Ok(Self {
            source: source.clone(),
            target,
            word,
            live_source,
            live_target,
            canonical_source: c_coord,
            canonical_target: d_coord,
            chart,
        })
    }

    /// Strict acceptance constructor enforcing ALL semantic return criteria for j=0 given source, target, and word.
    pub fn try_from_transition(
        source: &OrdinaryOdd,
        target: &OrdinaryOdd,
        word: &ValuationWord,
    ) -> Result<Self, J0CertificationError> {
        let cert = Self::try_from_source(source)?;
        if cert.target() != target {
            return Err(J0CertificationError::InvalidInput {
                message: format!("Provided target {} != derived target {}", target.value(), cert.target().value()),
            });
        }
        if cert.word() != word {
            return Err(J0CertificationError::ExactWordMismatch {
                expected: cert.word().exponents().to_vec(),
                actual: word.exponents().to_vec(),
            });
        }
        Ok(cert)
    }

    pub fn source(&self) -> &OrdinaryOdd {
        &self.source
    }

    pub fn target(&self) -> &OrdinaryOdd {
        &self.target
    }

    pub fn word(&self) -> &ValuationWord {
        &self.word
    }

    pub fn canonical_source(&self) -> &CanonicalSourceCoordinate {
        &self.canonical_source
    }

    pub fn canonical_target(&self) -> &CanonicalEndpointCoordinate {
        &self.canonical_target
    }

    pub fn chart(&self) -> &CanonicalCoordinateChart {
        &self.chart
    }
}
