use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationalRatioJson {
    pub numerator: String,   // p
    pub denominator: String, // q
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GraphContractionCertificateJson {
    pub schema_version: String,
    pub graph_hash: String,
    pub log2_3_upper_bound: RationalRatioJson,
    pub strict_margin: String,
    pub vertex_potentials: HashMap<String, String>,
    pub edge_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObstructionCycleJson {
    pub schema_version: String,
    pub cycle_length: usize,
    pub vertex_sequence: Vec<String>,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: u64,
    pub constant: String,
    pub primary_obstruction: String,
    pub positive_realizable: bool,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub u: String,
    pub v: String,
    pub valuation: u32,
}

/// Computes deterministic SHA-256 canonical hash of the abstract state graph.
pub fn compute_canonical_graph_hash(edges: &[GraphEdge], vertices: &[String]) -> String {
    let mut sorted_vertices = vertices.to_vec();
    sorted_vertices.sort();

    let mut sorted_edges: Vec<String> = edges
        .iter()
        .map(|e| format!("({},{},{})", e.u, e.v, e.valuation))
        .collect();
    sorted_edges.sort();

    let canonical_repr = format!(
        "graph_v1:vertices=[{}]:edges=[{}]",
        sorted_vertices.join(","),
        sorted_edges.join(",")
    );

    let mut hasher = Sha256::new();
    hasher.update(canonical_repr.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GraphContractionError {
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: String, found: String },

    #[error("Graph hash mismatch: computed {expected}, found in cert {found}")]
    GraphHashMismatch { expected: String, found: String },

    #[error("Invalid log2(3) upper bound: 2^{p} is not strictly greater than 3^{q}")]
    InvalidLogBound { p: String, q: String },

    #[error("Strict margin must be strictly positive (> 0), found {0}")]
    InvalidMargin(String),

    #[error("Missing potential for vertex {0}")]
    MissingPotential(String),

    #[error("Parse integer error: {0}")]
    ParseIntError(String),

    #[error("Edge potential contraction inequality violated for edge {u}->{v} (val={valuation}): h(v)-h(u) = {diff} < W(e)+margin = {target}")]
    EdgeContractionViolated {
        u: String,
        v: String,
        valuation: u32,
        diff: String,
        target: String,
    },
}

/// Pure-Rust independent verifier function for GraphContractionCertificateJson.
pub fn verify_graph_contraction_certificate(
    cert: &GraphContractionCertificateJson,
    edges: &[GraphEdge],
) -> Result<(), GraphContractionError> {
    if cert.schema_version != "graph_contraction_v1" {
        return Err(GraphContractionError::SchemaMismatch {
            expected: "graph_contraction_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    // Step 0: Verify canonical SHA-256 graph hash bound to vertices and edges
    let vertices: Vec<String> = cert.vertex_potentials.keys().cloned().collect();
    let computed_hash = compute_canonical_graph_hash(edges, &vertices);
    if cert.graph_hash != computed_hash {
        return Err(GraphContractionError::GraphHashMismatch {
            expected: computed_hash,
            found: cert.graph_hash.clone(),
        });
    }

    let p = BigUint::from_str(&cert.log2_3_upper_bound.numerator).map_err(|_| {
        GraphContractionError::ParseIntError(cert.log2_3_upper_bound.numerator.clone())
    })?;
    let q = usize::from_str(&cert.log2_3_upper_bound.denominator).map_err(|_| {
        GraphContractionError::ParseIntError(cert.log2_3_upper_bound.denominator.clone())
    })?;

    // Step 1: Verify exact integer log bound 2^p > 3^q

    let two_p = BigUint::one() << p.to_u64_digits().first().copied().unwrap_or(0) as usize;
    let pow3_q = BigUint::from(3u32).pow(q as u32);

    if two_p <= pow3_q {
        return Err(GraphContractionError::InvalidLogBound {
            p: cert.log2_3_upper_bound.numerator.clone(),
            q: cert.log2_3_upper_bound.denominator.clone(),
        });
    }

    // Step 2: Verify strict margin epsilon > 0
    let margin = BigInt::from_str(&cert.strict_margin)
        .map_err(|_| GraphContractionError::ParseIntError(cert.strict_margin.clone()))?;
    if margin <= BigInt::zero() {
        return Err(GraphContractionError::InvalidMargin(
            cert.strict_margin.clone(),
        ));
    }

    let p_big = BigInt::from_biguint(num_bigint::Sign::Plus, p.clone());
    let q_big = BigInt::from(q);

    // Step 3: Verify potential inequality h(v) - h(u) >= p - q * a_e + margin for all legal edges
    for edge in edges {
        let h_u_str = cert
            .vertex_potentials
            .get(&edge.u)
            .ok_or_else(|| GraphContractionError::MissingPotential(edge.u.clone()))?;
        let h_v_str = cert
            .vertex_potentials
            .get(&edge.v)
            .ok_or_else(|| GraphContractionError::MissingPotential(edge.v.clone()))?;

        let h_u = BigInt::from_str(h_u_str)
            .map_err(|_| GraphContractionError::ParseIntError(h_u_str.clone()))?;
        let h_v = BigInt::from_str(h_v_str)
            .map_err(|_| GraphContractionError::ParseIntError(h_v_str.clone()))?;

        let diff = &h_v - &h_u;
        let w_e = &p_big - (&q_big * BigInt::from(edge.valuation));
        let target = &w_e + &margin;

        if diff < target {
            return Err(GraphContractionError::EdgeContractionViolated {
                u: edge.u.clone(),
                v: edge.v.clone(),
                valuation: edge.valuation,
                diff: diff.to_string(),
                target: target.to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_graph_contraction_certificate() {
        let mut potentials = HashMap::new();
        potentials.insert("1".to_string(), "0".to_string());
        potentials.insert("3".to_string(), "10".to_string());

        let edges = vec![GraphEdge {
            u: "1".to_string(),
            v: "3".to_string(),
            valuation: 2,
        }];
        let vertices = vec!["1".to_string(), "3".to_string()];
        let canonical_hash = compute_canonical_graph_hash(&edges, &vertices);

        let cert = GraphContractionCertificateJson {
            schema_version: "graph_contraction_v1".to_string(),
            graph_hash: canonical_hash,
            log2_3_upper_bound: RationalRatioJson {
                numerator: "8".to_string(),
                denominator: "5".to_string(),
            },
            strict_margin: "1".to_string(),
            vertex_potentials: potentials,
            edge_count: 1,
        };

        assert!(verify_graph_contraction_certificate(&cert, &edges).is_ok());
    }
}
