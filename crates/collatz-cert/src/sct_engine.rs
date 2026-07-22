use crate::schema::{
    AffineMapJson, FeatureDefinitionJson, SctEdgeCertificateJson, SizeChangeCertificateJson,
    SizeChangeRelationJson, SizeChangeRelationKind, SizeChangeTransitionGraphJson, SourceGuardJson,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationValue {
    None = 0,
    Weak = 1,
    Strict = 2,
}

impl RelationValue {
    pub fn compose(r1: RelationValue, r2: RelationValue) -> RelationValue {
        match (r1, r2) {
            (RelationValue::None, _) | (_, RelationValue::None) => RelationValue::None,
            (RelationValue::Strict, _) | (_, RelationValue::Strict) => RelationValue::Strict,
            (RelationValue::Weak, RelationValue::Weak) => RelationValue::Weak,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SizeChangeGraph {
    pub source_node: String,
    pub target_node: String,
    pub valuation_word: Vec<u32>,
    /// Bipartite matrix mapping (src_feature, dst_feature) -> RelationValue
    pub matrix: BTreeMap<(String, String), RelationValue>,
}

impl PartialEq for SizeChangeGraph {
    fn eq(&self, other: &Self) -> bool {
        self.source_node == other.source_node
            && self.target_node == other.target_node
            && self.matrix == other.matrix
    }
}

impl Eq for SizeChangeGraph {}

impl std::hash::Hash for SizeChangeGraph {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source_node.hash(state);
        self.target_node.hash(state);
        self.matrix.hash(state);
    }
}

impl PartialOrd for SizeChangeGraph {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SizeChangeGraph {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_node
            .cmp(&other.source_node)
            .then_with(|| self.target_node.cmp(&other.target_node))
            .then_with(|| self.matrix.cmp(&other.matrix))
    }
}

impl SizeChangeGraph {
    pub fn new(
        source_node: impl Into<String>,
        target_node: impl Into<String>,
        valuation_word: Vec<u32>,
    ) -> Self {
        Self {
            source_node: source_node.into(),
            target_node: target_node.into(),
            valuation_word,
            matrix: BTreeMap::new(),
        }
    }

    pub fn add_relation(
        &mut self,
        src_feature: impl Into<String>,
        dst_feature: impl Into<String>,
        rel: RelationValue,
    ) {
        let key = (src_feature.into(), dst_feature.into());
        let current = self.matrix.get(&key).copied().unwrap_or(RelationValue::None);
        self.matrix.insert(key, current.max(rel));
    }

    pub fn get_relation(&self, src_feature: &str, dst_feature: &str) -> RelationValue {
        self.matrix
            .get(&(src_feature.to_string(), dst_feature.to_string()))
            .copied()
            .unwrap_or(RelationValue::None)
    }

    /// Direction convention: G1 o G2 means "execute G1, then G2".
    pub fn compose(&self, other: &Self) -> Option<Self> {
        if self.target_node != other.source_node {
            return None;
        }

        let mut composed_word = self.valuation_word.clone();
        composed_word.extend_from_slice(&other.valuation_word);

        let mut composed = Self::new(
            &self.source_node,
            &other.target_node,
            composed_word,
        );

        let src_features: BTreeSet<String> = self.matrix.keys().map(|k| k.0.clone()).collect();
        let mid_features: BTreeSet<String> = self.matrix.keys().map(|k| k.1.clone()).collect();
        let dst_features: BTreeSet<String> = other.matrix.keys().map(|k| k.1.clone()).collect();

        for u in &src_features {
            for v in &dst_features {
                let mut best = RelationValue::None;
                for m in &mid_features {
                    let r1 = self.get_relation(u, m);
                    let r2 = other.get_relation(m, v);
                    let comp = RelationValue::compose(r1, r2);
                    best = best.max(comp);
                }
                if best != RelationValue::None {
                    composed.add_relation(u, v, best);
                }
            }
        }

        Some(composed)
    }

    pub fn is_idempotent(&self) -> bool {
        if self.source_node != self.target_node {
            return false;
        }
        if let Some(comp) = self.compose(self) {
            comp.matrix == self.matrix
        } else {
            false
        }
    }

    pub fn has_strict_self_edge(&self) -> bool {
        if self.source_node != self.target_node {
            return false;
        }
        let features: BTreeSet<String> = self.matrix.keys().map(|k| k.0.clone()).collect();
        for f in features {
            if self.get_relation(&f, &f) == RelationValue::Strict {
                return true;
            }
        }
        false
    }
}

pub struct SctEngine;

impl SctEngine {
    /// Computes complete transitive closure via Hash-Set Saturation.
    pub fn compute_saturation_closure(
        generators: &[SizeChangeGraph],
        max_iterations: usize,
    ) -> Result<Vec<SizeChangeGraph>, String> {
        let mut closure: HashSet<SizeChangeGraph> = generators.iter().cloned().collect();
        let mut added = true;
        let mut iter_count = 0;

        while added {
            if iter_count >= max_iterations {
                return Err(format!(
                    "Closure saturation aborted: exceeded maximum iteration limit ({})",
                    max_iterations
                ));
            }
            iter_count += 1;
            added = false;

            let current_graphs: Vec<SizeChangeGraph> = closure.iter().cloned().collect();
            for g1 in &current_graphs {
                for g2 in &current_graphs {
                    if let Some(g3) = g1.compose(g2) {
                        if !closure.contains(&g3) {
                            closure.insert(g3);
                            added = true;
                        }
                    }
                }
            }
        }

        let mut result: Vec<SizeChangeGraph> = closure.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Verifies SCT termination property: every idempotent graph contains a strict descending self-edge.
    pub fn check_sct_termination(closure: &[SizeChangeGraph]) -> Result<(), String> {
        for g in closure {
            if g.is_idempotent() {
                if !g.has_strict_self_edge() {
                    return Err(format!(
                        "SCT Termination Violation: Idempotent graph for state {} contains no strict descending self-edge",
                        g.source_node
                    ));
                }
            }
        }
        Ok(())
    }

    /// Generates sound sct_edge_v1 subordinate certificate for an edge.
    pub fn generate_edge_certificate(
        edge_id: &str,
        source_state: &str,
        target_state: &str,
        valuation_word: &[u32],
        source_residue: &str,
        source_exponent: u64,
        affine_constant: &str,
        total_twos: u64,
        features: &[FeatureDefinitionJson],
        proved_relations: &[SizeChangeRelationJson],
    ) -> SctEdgeCertificateJson {
        SctEdgeCertificateJson {
            schema_version: "sct_edge_v1".to_string(),
            edge_id: edge_id.to_string(),
            source_state: source_state.to_string(),
            target_state: target_state.to_string(),
            valuation_word: valuation_word.to_vec(),
            source_guard: SourceGuardJson {
                residue: source_residue.to_string(),
                modulus_exponent: source_exponent,
                positivity_required: true,
            },
            affine_map: AffineMapJson {
                odd_steps: valuation_word.len(),
                total_twos,
                constant: affine_constant.to_string(),
            },
            features: features.to_vec(),
            proved_relations: proved_relations.to_vec(),
            proof_kind: "fixed_point_linear_form".to_string(),
        }
    }

    /// Generates complete size_change_scc_v1 certificate.
    pub fn generate_scc_certificate(
        scc_id: &str,
        feature_vector: &[&str],
        vertices: &[&str],
        generators: &[SizeChangeGraph],
    ) -> SizeChangeCertificateJson {
        let transition_graphs: Vec<SizeChangeTransitionGraphJson> = generators
            .iter()
            .map(|g| {
                let mut rels = Vec::new();
                for ((src, dst), r) in &g.matrix {
                    let r_kind = match r {
                        RelationValue::Strict => SizeChangeRelationKind::Decrease,
                        RelationValue::Weak => SizeChangeRelationKind::NonIncrease,
                        RelationValue::None => SizeChangeRelationKind::Reset,
                    };
                    if *r != RelationValue::None {
                        rels.push(SizeChangeRelationJson {
                            src_feature: src.clone(),
                            relation: r_kind,
                            dst_feature: dst.clone(),
                        });
                    }
                }
                SizeChangeTransitionGraphJson {
                    source_node: g.source_node.clone(),
                    target_node: g.target_node.clone(),
                    valuation_word: g.valuation_word.clone(),
                    relations: rels,
                }
            })
            .collect();

        let canonical_edge_ordering: Vec<String> = generators
            .iter()
            .map(|g| format!("{}->{}", g.source_node, g.target_node))
            .collect();

        SizeChangeCertificateJson {
            schema_version: "size_change_scc_v1".to_string(),
            scc_id: scc_id.to_string(),
            feature_vector: feature_vector.iter().map(|s| s.to_string()).collect(),
            vertices: vertices.iter().map(|s| s.to_string()).collect(),
            transition_graphs,
            canonical_edge_ordering,
            verifier_recomputation_required: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composition_algebra() {
        assert_eq!(
            RelationValue::compose(RelationValue::Strict, RelationValue::Weak),
            RelationValue::Strict
        );
        assert_eq!(
            RelationValue::compose(RelationValue::Weak, RelationValue::Weak),
            RelationValue::Weak
        );
        assert_eq!(
            RelationValue::compose(RelationValue::None, RelationValue::Strict),
            RelationValue::None
        );
    }
}
