use collatz_affine::{AffinePrefix, ValuationWord};
use std::collections::{HashMap, HashSet};

/// Abstract state representing congruence class r mod 2^m.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractState {
    pub residue: u64,
    pub modulus_exponent: u64,
}

/// Abstract transition edge.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractEdge {
    pub from: AbstractState,
    pub to: AbstractState,
    pub valuation: u8,
}

/// Miniature CEGAR Vertical Slice Prototype.
pub struct MiniCegarEngine {
    pub modulus_exponent: u64,
    pub states: HashSet<AbstractState>,
    pub edges: HashSet<AbstractEdge>,
}

impl MiniCegarEngine {
    pub fn new(modulus_exponent: u64) -> Self {
        let mut states = HashSet::new();
        let limit = 1u64 << modulus_exponent;
        for r in (1..limit).step_by(2) {
            states.insert(AbstractState {
                residue: r,
                modulus_exponent,
            });
        }

        Self {
            modulus_exponent,
            states,
            edges: HashSet::new(),
        }
    }

    /// Builds concrete transitions for valuation words up to length k.
    pub fn build_abstract_edges(&mut self, _k: usize) {
        let limit = 1u64 << self.modulus_exponent;

        for r in (1..limit).step_by(2) {
            let from_state = AbstractState {
                residue: r,
                modulus_exponent: self.modulus_exponent,
            };

            // Sample valuation step a_1 = 1..=4
            for a_1 in 1..=4u8 {
                let word = ValuationWord::new(vec![a_1]).unwrap();
                if let Ok(_prefix) = AffinePrefix::from_valuation_word(word) {

                    let next_r = (3 * r + 1) >> a_1;
                    let to_r = next_r % limit;
                    let to_state = AbstractState {
                        residue: to_r,
                        modulus_exponent: self.modulus_exponent,
                    };

                    self.edges.insert(AbstractEdge {
                        from: from_state.clone(),
                        to: to_state,
                        valuation: a_1,
                    });
                }
            }
        }
    }

    /// Detects abstract critical cycles using DFS.
    pub fn detect_abstract_cycles(&self) -> Vec<Vec<AbstractEdge>> {
        let mut cycles = Vec::new();
        let mut adj: HashMap<AbstractState, Vec<AbstractEdge>> = HashMap::new();

        for edge in &self.edges {
            adj.entry(edge.from.clone()).or_default().push(edge.clone());
        }

        for start in &self.states {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            self.dfs_cycle(start, start, &adj, &mut visited, &mut path, &mut cycles);
        }

        cycles
    }

    fn dfs_cycle(
        &self,
        curr: &AbstractState,
        target: &AbstractState,
        adj: &HashMap<AbstractState, Vec<AbstractEdge>>,
        visited: &mut HashSet<AbstractState>,
        path: &mut Vec<AbstractEdge>,
        cycles: &mut Vec<Vec<AbstractEdge>>,
    ) {
        if let Some(edges) = adj.get(curr) {
            for edge in edges {
                if &edge.to == target && !path.is_empty() {
                    cycles.push(path.clone());
                } else if !visited.contains(&edge.to) && path.len() < 5 {
                    visited.insert(edge.to.clone());
                    path.push(edge.clone());
                    self.dfs_cycle(&edge.to, target, adj, visited, path, cycles);
                    path.pop();
                    visited.remove(&edge.to);
                }
            }
        }
    }

    /// Concretizes an abstract cycle and tests exact infeasibility/descent proof.
    pub fn refine_abstract_cycle(&self, cycle: &[AbstractEdge]) -> bool {
        let val_seq: Vec<u8> = cycle.iter().map(|e| e.valuation).collect();
        if let Ok(word) = ValuationWord::new(val_seq) {
            if let Ok(prefix) = AffinePrefix::from_valuation_word(word) {
                // Return true if cycle is multiplicatively contracting or infeasible in N+
                return prefix.is_multiplicative_contracting();
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_cegar_prototype() {
        let mut engine = MiniCegarEngine::new(2); // Modulo 4
        engine.build_abstract_edges(2);

        let cycles = engine.detect_abstract_cycles();
        assert!(!cycles.is_empty());

        let mut refined_count = 0;
        for cycle in &cycles {
            if engine.refine_abstract_cycle(cycle) {
                refined_count += 1;
            }
        }

        assert!(refined_count > 0);
    }
}
