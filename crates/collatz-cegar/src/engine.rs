use crate::abstract_domain::{AbstractEdge, RelationalState};
use crate::concretization::ConcretizationEngine;
use crate::karp_cycle::KarpCycleEngine;
use crate::refinement::{NegativeRefinementLemmaJson, RefinementEngine};
use collatz_cert::verify_descent_certificate;
use collatz_cert::DescentCertificateJson;
use std::collections::HashSet;

pub struct CegarEngineConfig {
    pub max_depth: usize,
    pub max_iterations: usize,
    pub max_states: usize,
}

impl Default for CegarEngineConfig {
    fn default() -> Self {
        Self {
            max_depth: 20,
            max_iterations: 500,
            max_states: 100_000,
        }
    }
}

pub struct CegarEngineReport {
    pub total_states: usize,
    pub total_edges: usize,
    pub dangerous_cycles_found: usize,
    pub certificates_generated: Vec<DescentCertificateJson>,
    pub negative_refinement_lemma: Option<NegativeRefinementLemmaJson>,
}

pub struct CegarEngine {
    pub config: CegarEngineConfig,
    pub states: HashSet<RelationalState>,
    pub edges: HashSet<AbstractEdge>,
}

impl CegarEngine {
    pub fn new(config: CegarEngineConfig) -> Self {
        Self {
            config,
            states: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    /// Initializes abstract state graph for modulus 2^m.
    pub fn build_abstract_graph(&mut self, modulus_exponent: u64) {
        let limit = 1u64 << modulus_exponent;
        for r in (1..limit).step_by(2) {
            let state = RelationalState::new_congruence(r, modulus_exponent);
            self.states.insert(state);
        }

        for r in (1..limit).step_by(2) {
            let from_state = RelationalState::new_congruence(r, modulus_exponent);
            for a_1 in 1..=4u8 {
                let next_r = ((3 * r + 1) >> a_1) % limit;
                let to_state = RelationalState::new_congruence(next_r, modulus_exponent);
                self.edges.insert(AbstractEdge {
                    from: from_state.clone(),
                    to: to_state,
                    valuation: a_1,
                });
            }
        }
    }

    /// Runs full CEGAR loop up to max_iterations.
    pub fn run_cegar_loop(&mut self) -> CegarEngineReport {
        let mut generated_certs = Vec::new();
        let mut iterations = 0;
        let mut dangerous_cycles_found = 0;

        while iterations < self.config.max_iterations && self.states.len() <= self.config.max_states {
            iterations += 1;
            let cycles = KarpCycleEngine::find_cycles(&self.states, &self.edges, self.config.max_depth);
            let dangerous_cycles: Vec<_> = cycles
                .into_iter()
                .filter(|c| KarpCycleEngine::is_dangerous_cycle(c))
                .collect();

            if dangerous_cycles.is_empty() {
                break; // Soundness achieved: All abstract cycles are strictly contracting!
            }

            dangerous_cycles_found += dangerous_cycles.len();
            let mut pruned_edges = 0;

            for cycle in &dangerous_cycles {
                if let Ok(res) = ConcretizationEngine::concretize_cycle(cycle) {
                    if let Some(cert) = RefinementEngine::process_concretization_result(res) {
                        if verify_descent_certificate(&cert).is_ok() {
                            generated_certs.push(cert);
                            // Prune edge to refine abstract transition graph
                            if let Some(edge_to_remove) = cycle.first() {
                                self.edges.remove(edge_to_remove);
                                pruned_edges += 1;
                            }
                        }
                    }
                }
            }

            if pruned_edges == 0 {
                break; // Refinement plateau reached
            }
        }

        let negative_refinement_lemma = if iterations >= self.config.max_iterations || self.states.len() > self.config.max_states {
            Some(RefinementEngine::emit_negative_refinement_lemma(
                self.config.max_depth,
                iterations,
                dangerous_cycles_found,
            ))
        } else {
            None
        };

        CegarEngineReport {
            total_states: self.states.len(),
            total_edges: self.edges.len(),
            dangerous_cycles_found,
            certificates_generated: generated_certs,
            negative_refinement_lemma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cegar_engine_run() {
        let config = CegarEngineConfig {
            max_depth: 5,
            max_iterations: 10,
            max_states: 100,
        };
        let mut engine = CegarEngine::new(config);
        engine.build_abstract_graph(2); // Modulo 4

        let report = engine.run_cegar_loop();
        assert!(report.total_states > 0);
    }
}
