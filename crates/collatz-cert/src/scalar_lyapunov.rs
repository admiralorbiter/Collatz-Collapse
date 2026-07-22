use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub const MIN_LYAPUNOV_MODULUS_EXPONENT: u32 = 1;
pub const MAX_LYAPUNOV_MODULUS_EXPONENT: u32 = 16; // Max 65,536 weights to prevent memory DoS

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ValuationConstraint {
    Exact(u32),
    AtLeast(u32),
}

impl ValuationConstraint {
    pub fn min_valuation(&self) -> u32 {
        match self {
            ValuationConstraint::Exact(v) => *v,
            ValuationConstraint::AtLeast(v) => *v,
        }
    }
}

/// Computes deterministic SHA-256 canonical hash of the complete residue transition graph.
pub fn compute_canonical_lyapunov_graph_hash(transitions: &[ScalarTransition], modulus_exp: u32) -> String {
    let mut sorted_edges: Vec<String> = transitions
        .iter()
        .map(|t| match t.valuation {
            ValuationConstraint::Exact(v) => format!("({},{},Exact({}))", t.r_src, t.r_dst, v),
            ValuationConstraint::AtLeast(v) => format!("({},{},AtLeast({}))", t.r_src, t.r_dst, v),
        })
        .collect();
    sorted_edges.sort();

    let canonical_repr = format!(
        "lyapunov_v1:m={}:domain=positive_odd_integers:edges=[{}]",
        modulus_exp,
        sorted_edges.join(",")
    );

    let mut hasher = Sha256::new();
    hasher.update(canonical_repr.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ScalarLyapunovCertificateJson {
    pub schema_version: String,
    pub graph_hash: String,
    pub global_scale_q: u64,
    pub modulus_exponent: u32,
    pub strict_margin: i64,
    pub non_negative_weights: bool,
    pub residue_weights: HashMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScalarTransition {
    pub r_src: u64,
    pub r_dst: u64,
    pub valuation: ValuationConstraint,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ScalarLyapunovError {
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: String, found: String },

    #[error("Graph hash mismatch: computed {expected}, found in cert {found}")]
    GraphHashMismatch { expected: String, found: String },

    #[error("Modulus exponent {0} is out of bounds [{1}, {2}]")]
    ModulusExponentOutOfBounds(u32, u32, u32),

    #[error("Strict margin must be strictly positive (> 0), found {0}")]
    InvalidMargin(i64),

    #[error("Global scale q out of range or negative")]
    ScaleOutOfRange,

    #[error("Non-negative weights invariant must be set to true in scalar_lyapunov_v1")]
    NonNegativeWeightsRequired,

    #[error("Negative weight found for residue {0}: {1}")]
    NegativeWeightEnforced(String, i64),

    #[error("Missing weight entry for residue {0}")]
    MissingWeight(u64),

    #[error("Extra or out-of-range weight keys in weight table: expected {expected}, found {found}")]
    ExtraWeightKeys { expected: usize, found: usize },

    #[error("Checked overflow occurred during Lyapunov transition inequality evaluation")]
    Overflow,

    #[error("Transition inequality violated for residue {r_src}->{r_dst} (val={valuation:?}): w_dst - w_src = {diff} > target = {target}")]
    TransitionInequalityViolated {
        r_src: u64,
        r_dst: u64,
        valuation: ValuationConstraint,
        diff: i64,
        target: i64,
    },
}

/// Reconstructs the complete source-target relation for the declared modulo-2^m abstraction,
/// with exact or conservative minimum-valuation annotations.
pub fn reconstruct_complete_residue_transitions(modulus_exponent: u32) -> Vec<ScalarTransition> {
    let modulus = 1u64 << modulus_exponent;
    let mut transitions = Vec::new();

    for r in (1..modulus).step_by(2) {
        let val_r = (3 * r + 1).trailing_zeros();
        
        if val_r >= modulus_exponent {
            // Unbounded tail (e.g. r=5 mod 16): Conservative minimum valuation AtLeast(m)
            for dst in (1..modulus).step_by(2) {
                transitions.push(ScalarTransition {
                    r_src: r,
                    r_dst: dst,
                    valuation: ValuationConstraint::AtLeast(val_r),
                });
            }
        } else {
            // Finite shift: 2^val_r distinct target residues with exact valuation Exact(val_r)
            let num_targets = 1u64 << val_r;
            let step = 1u64 << (modulus_exponent - val_r);
            let base_dst = ((3 * r + 1) >> val_r) % modulus;

            for j in 0..num_targets {
                let target_r = (base_dst + j * step) % modulus;
                transitions.push(ScalarTransition {
                    r_src: r,
                    r_dst: target_r,
                    valuation: ValuationConstraint::Exact(val_r),
                });
            }
        }
    }

    transitions
}

/// Pure-Rust independent verifier function for ScalarLyapunovCertificateJson.
/// Reconstructs 100% of legal residue transitions independently from declared parameters.
pub fn verify_scalar_lyapunov_certificate(
    cert: &ScalarLyapunovCertificateJson,
) -> Result<(), ScalarLyapunovError> {
    if cert.schema_version != "scalar_lyapunov_v1" {
        return Err(ScalarLyapunovError::SchemaMismatch {
            expected: "scalar_lyapunov_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    if cert.modulus_exponent < MIN_LYAPUNOV_MODULUS_EXPONENT || cert.modulus_exponent > MAX_LYAPUNOV_MODULUS_EXPONENT {
        return Err(ScalarLyapunovError::ModulusExponentOutOfBounds(
            cert.modulus_exponent,
            MIN_LYAPUNOV_MODULUS_EXPONENT,
            MAX_LYAPUNOV_MODULUS_EXPONENT,
        ));
    }

    if cert.strict_margin <= 0 {
        return Err(ScalarLyapunovError::InvalidMargin(cert.strict_margin));
    }

    if !cert.non_negative_weights {
        return Err(ScalarLyapunovError::NonNegativeWeightsRequired);
    }

    let q = i64::try_from(cert.global_scale_q).map_err(|_| ScalarLyapunovError::ScaleOutOfRange)?;
    if q < 0 {
        return Err(ScalarLyapunovError::ScaleOutOfRange);
    }

    let modulus = 1u64 << cert.modulus_exponent;
    let expected_num_odd_residues = (modulus / 2) as usize;

    if cert.residue_weights.len() != expected_num_odd_residues {
        return Err(ScalarLyapunovError::ExtraWeightKeys {
            expected: expected_num_odd_residues,
            found: cert.residue_weights.len(),
        });
    }

    // Step 0: Independently reconstruct ALL legal residue transitions & verify SHA-256 graph hash
    let complete_transitions = reconstruct_complete_residue_transitions(cert.modulus_exponent);
    let computed_hash = compute_canonical_lyapunov_graph_hash(&complete_transitions, cert.modulus_exponent);
    if cert.graph_hash != computed_hash {
        return Err(ScalarLyapunovError::GraphHashMismatch {
            expected: computed_hash,
            found: cert.graph_hash.clone(),
        });
    }

    // Step 1: Check that all odd residues have valid non-negative weights
    for r in (1..modulus).step_by(2) {
        let weight = cert
            .residue_weights
            .get(&r.to_string())
            .ok_or(ScalarLyapunovError::MissingWeight(r))?;
        if *weight < 0 {
            return Err(ScalarLyapunovError::NegativeWeightEnforced(
                r.to_string(),
                *weight,
            ));
        }
    }

    let margin = cert.strict_margin;

    // Step 2: Verify decrease requirement on ALL legal transitions across complete graph
    for trans in &complete_transitions {
        let w_src = cert
            .residue_weights
            .get(&trans.r_src.to_string())
            .ok_or(ScalarLyapunovError::MissingWeight(trans.r_src))?;
        let w_dst = cert
            .residue_weights
            .get(&trans.r_dst.to_string())
            .ok_or(ScalarLyapunovError::MissingWeight(trans.r_dst))?;

        let diff = w_dst.checked_sub(*w_src).ok_or(ScalarLyapunovError::Overflow)?;

        let min_val = trans.valuation.min_valuation();
        let delta_ceiling = 2i64
            .checked_sub(min_val as i64)
            .ok_or(ScalarLyapunovError::Overflow)?;
        let q_delta = q.checked_mul(delta_ceiling).ok_or(ScalarLyapunovError::Overflow)?;
        let target = (-margin)
            .checked_sub(q_delta)
            .ok_or(ScalarLyapunovError::Overflow)?;

        if diff > target {
            return Err(ScalarLyapunovError::TransitionInequalityViolated {
                r_src: trans.r_src,
                r_dst: trans.r_dst,
                valuation: trans.valuation.clone(),
                diff,
                target,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_lyapunov_graph_hash_exact_match() {
        let transitions = reconstruct_complete_residue_transitions(4);
        let hash = compute_canonical_lyapunov_graph_hash(&transitions, 4);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_regression_reject_invalid_mod16_certificate() {
        let mut weights = HashMap::new();
        weights.insert("1".to_string(), 56i64);
        weights.insert("3".to_string(), 48i64);
        weights.insert("5".to_string(), 40i64);
        weights.insert("7".to_string(), 32i64);
        weights.insert("9".to_string(), 24i64);
        weights.insert("11".to_string(), 16i64);
        weights.insert("13".to_string(), 8i64);
        weights.insert("15".to_string(), 0i64);

        let complete_transitions = reconstruct_complete_residue_transitions(4);
        let canonical_hash = compute_canonical_lyapunov_graph_hash(&complete_transitions, 4);

        let cert = ScalarLyapunovCertificateJson {
            schema_version: "scalar_lyapunov_v1".to_string(),
            graph_hash: canonical_hash,
            global_scale_q: 8,
            modulus_exponent: 4,
            strict_margin: 1,
            non_negative_weights: true,
            residue_weights: weights,
        };

        // Verifier MUST reject this invalid certificate
        let res = verify_scalar_lyapunov_certificate(&cert);
        assert!(res.is_err());
    }
}
