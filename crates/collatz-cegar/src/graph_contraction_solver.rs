use collatz_cert::graph_contraction::{
    compute_canonical_graph_hash, verify_graph_contraction_certificate,
    GraphContractionCertificateJson, GraphEdge, ObstructionCycleJson, RationalRatioJson,
};
use std::collections::HashMap;

pub struct GraphContractionSolver {
    pub p: u64,      // 8
    pub q: u64,      // 5 (2^8 = 256 > 243 = 3^5)
    pub margin: i64, // 1
}

impl Default for GraphContractionSolver {
    fn default() -> Self {
        Self {
            p: 8,
            q: 5,
            margin: 1,
        }
    }
}

impl GraphContractionSolver {
    pub fn new(p: u64, q: u64, margin: i64) -> Self {
        Self { p, q, margin }
    }

    /// Solves difference constraints h(v) - h(u) >= p - q * a_e + margin for all edges e: u -> v.
    /// Emits a verified GraphContractionCertificateJson upon success.
    #[allow(clippy::result_large_err)]
    pub fn solve(
        &self,
        edges: &[GraphEdge],
        vertex_ids: &[String],
    ) -> Result<GraphContractionCertificateJson, ObstructionCycleJson> {
        let mut potentials: HashMap<String, i64> = HashMap::new();
        for id in vertex_ids {
            potentials.insert(id.clone(), 0);
        }

        // Run Bellman-Ford / Shortest Path refinement to compute exact potentials
        let num_vertices = vertex_ids.len();
        for _ in 0..num_vertices {
            let mut updated = false;
            for edge in edges {
                let h_u = *potentials.get(&edge.u).unwrap_or(&0);
                let h_v = *potentials.get(&edge.v).unwrap_or(&0);
                let w_e = self.p as i64 - (self.q as i64 * edge.valuation as i64);
                let required_min_hv = h_u + w_e + self.margin;

                if h_v < required_min_hv {
                    potentials.insert(edge.v.clone(), required_min_hv);
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }

        // Check if any edge violates the potential inequality (indicating a noncontracting cycle)
        for edge in edges {
            let h_u = *potentials.get(&edge.u).unwrap_or(&0);
            let h_v = *potentials.get(&edge.v).unwrap_or(&0);
            let w_e = self.p as i64 - (self.q as i64 * edge.valuation as i64);
            let target = w_e + self.margin;

            if h_v - h_u < target {
                // Noncontracting cycle / obstruction detected! Emit ObstructionCycleJson artifact.
                let valuations: Vec<u32> = edges.iter().map(|e| e.valuation).collect();
                let vertex_seq: Vec<String> = edges.iter().map(|e| e.u.clone()).collect();

                return Err(ObstructionCycleJson {
                    schema_version: "obstruction_cycle_v1".to_string(),
                    cycle_length: edges.len(),
                    vertex_sequence: vertex_seq,
                    valuation_word: valuations,
                    total_twos: edges.iter().map(|e| e.valuation as u64).sum(),
                    odd_steps: edges.len() as u64,
                    constant: "0".to_string(),
                    primary_obstruction: "TransitionInfeasible".to_string(),
                    positive_realizable: false,
                });
            }
        }

        let mut string_potentials = HashMap::new();
        for (k, v) in potentials {
            string_potentials.insert(k, v.to_string());
        }

        let canonical_hash = compute_canonical_graph_hash(edges, vertex_ids);

        let cert = GraphContractionCertificateJson {
            schema_version: "graph_contraction_v1".to_string(),
            graph_hash: canonical_hash,
            log2_3_upper_bound: RationalRatioJson {
                numerator: self.p.to_string(),
                denominator: self.q.to_string(),
            },
            strict_margin: self.margin.to_string(),
            vertex_potentials: string_potentials,
            edge_count: edges.len(),
        };

        // Self-verify certificate before returning
        verify_graph_contraction_certificate(&cert, edges).map_err(|e| ObstructionCycleJson {
            schema_version: "obstruction_cycle_v1".to_string(),
            cycle_length: edges.len(),
            vertex_sequence: vertex_ids.to_vec(),
            valuation_word: edges.iter().map(|e| e.valuation).collect(),
            total_twos: 0,
            odd_steps: 0,
            constant: "0".to_string(),
            primary_obstruction: format!("SelfVerificationFailed: {}", e),
            positive_realizable: false,
        })?;

        Ok(cert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_contraction_solver_cyclic_success() {
        let solver = GraphContractionSolver::default();
        // Strongly connected graph with true directed cycle 1 -> 3 -> 1 (valuations a1=2, a2=2)
        let edges = vec![
            GraphEdge {
                u: "1".to_string(),
                v: "3".to_string(),
                valuation: 2,
            },
            GraphEdge {
                u: "3".to_string(),
                v: "1".to_string(),
                valuation: 2,
            },
        ];
        let vertices = vec!["1".to_string(), "3".to_string()];

        let res = solver.solve(&edges, &vertices);
        assert!(res.is_ok());
        let cert = res.unwrap();
        assert_eq!(cert.schema_version, "graph_contraction_v1");
        assert_eq!(cert.edge_count, 2);
    }

    #[test]
    fn test_graph_contraction_solver_failed_synthesis_obstruction() {
        let solver = GraphContractionSolver::default();
        // Noncontracting cycle 1 -> 3 -> 1 with expanding valuations a1=1, a2=1 (2^2 = 4 < 9 = 3^2)
        let edges = vec![
            GraphEdge {
                u: "1".to_string(),
                v: "3".to_string(),
                valuation: 1,
            },
            GraphEdge {
                u: "3".to_string(),
                v: "1".to_string(),
                valuation: 1,
            },
        ];
        let vertices = vec!["1".to_string(), "3".to_string()];

        let res = solver.solve(&edges, &vertices);
        assert!(res.is_err());
        let obstruction = res.unwrap_err();
        assert_eq!(obstruction.schema_version, "obstruction_cycle_v1");
        assert_eq!(obstruction.primary_obstruction, "TransitionInfeasible");
        assert!(!obstruction.positive_realizable);
    }
}
